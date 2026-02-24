pub mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod util;

pub const DELEGATION_BUFFER_SEED: &[u8] = b"buffer";

solana_program::declare_id!("DL2q6XaUpXsPsYrDpbieiXG6UisaUpzMSZCTkSvzn2Am");
