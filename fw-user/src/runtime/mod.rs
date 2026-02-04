// Expose runtime lifecycle control
// Tie together capture, engine, IPC and enforcement
// Define startup and shutdown sequence

mod capture;

pub(crate) use capture::start_capture;
