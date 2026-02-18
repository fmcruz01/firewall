// Send control commands
//
// Send compiled rulesets
// Request status/metrics
// Handle responses

use tokio::net::UnixStream;
use tokio::process::Command;
use tokio::time::{sleep, Duration};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

const SOCK_PATH: &str = "/tmp/fw-user.sock";

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

pub async fn sniff(interface: String, verbose: bool) -> anyhow::Result<()> {
    let mut stream = match UnixStream::connect(SOCK_PATH).await {
        Ok(s) => s,
        Err(_) => {
            let mut child = Command::new("/home/random/Random/firewall/target/debug/fw-user").arg("--socket").arg(SOCK_PATH).spawn()?;
            let mut stream = connect_with_retry().await?;
            let resp = send(&mut stream, &Request::Start { interface, verbose }).await?;

            if !matches!(resp, Response::Ok) {
                return Err(anyhow::anyhow!("start failed: {resp:?}"));
            }

            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    let _ = send(&mut stream, &Request::Stop).await;
                }
                status = child.wait() => {
                    eprintln!("fw-user exited: {status:?}");
                }
            }
                return Ok(());
        }
    };
    let resp = send(&mut stream, &Request::Start { interface, verbose }).await?;
    if !matches!(resp, Response::Ok) {
        return Err(anyhow::anyhow!("start failed: {resp:?}"));
    }
    Ok(())
}

pub async fn stop() -> anyhow::Result<()> {
    let mut stream = UnixStream::connect(SOCK_PATH).await?;
    let _ = send(&mut stream, &Request::Stop).await?;
    Ok(())
}

async fn connect_with_retry() -> std::io::Result<UnixStream> {
    let mut last = None;
    for _ in 0..50 {
        match UnixStream::connect(SOCK_PATH).await {
            Ok(s) => return Ok(s),
            Err(e) => {
                last = Some(e);
                sleep(Duration::from_millis(50)).await;
            }
        }
    }
    Err(last.unwrap_or_else(|| std::io::Error::other("connect retry failed")))
}

async fn send(stream: &mut UnixStream, req: &Request) -> anyhow::Result<Response> {
    let json = serde_json::to_string(req)?;
    stream.write_all(json.as_bytes()).await?;
    stream.write_all("\n".as_bytes()).await?;
    stream.flush().await?;

    let mut rd = BufReader::new(stream);
    let mut line = String::new();
    rd.read_line(&mut line).await?;
    Ok(serde_json::from_str::<Response>(line.trim_end())?)
}

