// Define IPC abstractions
//
// Define message types exchanged with fw-ctl
// Define lifecycle of IPC connections

pub mod server;
pub(crate) use server::run;
