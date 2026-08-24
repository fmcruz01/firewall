use thiserror::Error;
use uuid::Uuid;

pub struct Device {
    pub id: Uuid,
    pub ip: String,
    pub name: String,
}

#[derive(Debug, Error)]
pub enum DiscoverError {
    #[error("failed to {operation} {socket_type} socket")]
    SocketError {
        operation: String,
        socket_type: String,
        #[source]
        source: std::io::Error,
    }
}
