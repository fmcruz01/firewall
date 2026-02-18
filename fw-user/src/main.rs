mod ipc;
mod runtime;

use std::env;

const SOCK_DEFAULT_PATH: &str = "/tmp/fw-user.sock";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1);
    let mut sock = SOCK_DEFAULT_PATH.to_string();
    while let Some(a) = args.next() {
        if a == "--socket" {
            if let Some(p) = args.next() {
                sock = p;
            }
        }
    }
    ipc::server::run(sock).await
}
