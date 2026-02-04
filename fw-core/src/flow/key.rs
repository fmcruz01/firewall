#[repr(C)]
pub struct FlowKey {}

// Define how a flow is identified
//
// Define the flowkey structure (addresses, ports, protocol, direction)
// Normalize the keys so both directions map correctly when appropriate
// Ensure keys are cheap to hash and compare
