// Accept control-plane connections
//
// Listen on UNIX socket
// Authenticate clients if needed
// Receive compile rulesets
// Swap rules atomically

use crate::runtime::start_capture;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::unix::OwnedWriteHalf;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::watch;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Request {
    Start { interface: String, verbose: bool },
    Stop,
    Status,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Response {
    Ok,
    Error { message: String },
    Status { running: bool },
}

#[derive(Clone, Debug)]
struct RunCfg {
    interface: String,
    verbose: bool,
}

#[derive(Clone, Debug)]
enum RunState {
    Stopped,
    Running(RunCfg),
}

pub async fn run(sock_path: String) -> anyhow::Result<()> {
    let _ = std::fs::remove_file(&sock_path);
    let listener = UnixListener::bind(&sock_path)?;

    let (tx, rx) = watch::channel(RunState::Stopped);
    tokio::spawn(runtime_task(rx));

    loop {
        let (stream, _addr) = listener.accept().await?;
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, tx).await {
                eprintln!("ipc client error: {e:#}");
            }
        });
    }
}

async fn handle_client(stream: UnixStream, tx: watch::Sender<RunState>) -> anyhow::Result<()> {
    let (rd, mut wr) = stream.into_split();
    let mut lines = BufReader::new(rd).lines();

    while let Some(line) = lines.next_line().await? {
        let req: Request = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                write_resp(
                    &mut wr,
                    Response::Error {
                        message: e.to_string(),
                    },
                )
                .await?;
                continue;
            }
        };
        let resp = match req {
            Request::Start { interface, verbose } => {
                let _ = tx.send(RunState::Running(RunCfg { interface, verbose }));
                Response::Ok
            }

            Request::Stop => {
                let _ = tx.send(RunState::Stopped);
                Response::Ok
            }

            Request::Status => {
                let running = matches!(*tx.borrow(), RunState::Running(_));
                Response::Status { running }
            }
        };
        write_resp(&mut wr, resp).await?;
    }
    Ok(())
}

async fn write_resp(wr: &mut OwnedWriteHalf, resp: Response) -> anyhow::Result<()> {
    let json = serde_json::to_string(&resp)?;
    wr.write_all(json.as_bytes()).await?;
    wr.write_all(b"\n").await?;
    wr.flush().await?;
    Ok(())
}

async fn runtime_task(mut rx: watch::Receiver<RunState>) {
    let mut current: Option<tokio::task::JoinHandle<()>> = None;

    loop {
        if rx.changed().await.is_err() {
            break;
        }

        // Important: `watch::Receiver::borrow()` returns a non-Send guard.
        // Move the value out immediately so we don't hold the guard across any `.await`.
        let state = { rx.borrow().clone() };

        match state {
            RunState::Stopped => {
                if let Some(h) = current.take() {
                    h.abort();
                    let _ = h.await;
                    println!("capture stopped");
                }
            }
            RunState::Running(cfg) => {
                if current.is_none() {
                    println!(
                        "capture starting on {} (verbose={})",
                        cfg.interface, cfg.verbose
                    );
                    current = Some(tokio::task::spawn_blocking(move || {
                        let _ = start_capture(cfg.verbose);
                    }));
                }
            }
        }
    }
}
