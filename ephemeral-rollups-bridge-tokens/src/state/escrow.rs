use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Escrow {
    pub amount: u64,
}

impl Escrow {
    pub fn space() -> usize {
        size_of::<u64>()
    }
}

pub const ESCROW_SEEDS_PREFIX: &[u8] = b"escrow";

#[macro_export]
macro_rules! escrow_token_account_seeds_generator {
    ($user_funding: expr, $user_claimer: expr, $mint: expr, $validator_id: expr, $index: expr) => {
        &[
            crate::state::escrow_token::ESCROW_SEEDS_PREFIX,
            &$user_funding.to_bytes(),
            &$user_claimer.to_bytes(),
            &$mint.to_bytes(),
            &$validator_id.to_bytes(),
            &$index.to_le_bytes(),
        ]
    };
}
