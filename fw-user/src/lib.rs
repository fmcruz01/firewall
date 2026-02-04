pub mod runtime;

#[derive(Debug)]
pub enum RuntimeError {
    PermissionDenied,
    InterfaceNotFound,
    CaptureError,
    Shutdown,
}

pub fn start_processing(verbose: bool) -> Result<(), RuntimeError> {
    runtime::start_capture(verbose)
}
