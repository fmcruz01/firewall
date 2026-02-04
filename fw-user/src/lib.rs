pub mod runtime;

pub fn start_processing(verbose: bool) {
    runtime::start(verbose);
}
