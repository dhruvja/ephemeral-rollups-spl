use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Escrow {}

impl Escrow {
    pub fn space() -> usize {
        0
    }
}

pub const ESCROW_SEEDS_PREFIX: &[u8] = b"escrow";

#[macro_export]
macro_rules! escrow_seeds_generator {
    ($user_funding: expr, $user_claimer: expr, $validator_id: expr, $index: expr) => {
        &[
            crate::state::escrow::ESCROW_SEEDS_PREFIX,
            &$user_funding.to_bytes(),
            &$user_claimer.to_bytes(),
            &$validator_id.to_bytes(),
            &$index.to_le_bytes(),
        ]
    };
}
