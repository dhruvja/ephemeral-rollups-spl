use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct EscrowLamports {
    pub user_funding: Pubkey,
    pub user_claimer: Pubkey,
}

impl EscrowLamports {
    pub const SEEDS_PREFIX: &[u8] = b"escrow_lamports";

    pub fn space() -> usize {
        size_of::<Pubkey>() + size_of::<Pubkey>()
    }
}

#[macro_export]
macro_rules! escrow_lamports_seeds_generator {
    ($user_funding: expr, $user_claimer: expr, $validator_id: expr, $index: expr) => {
        &[
            EscrowLamports::SEEDS_PREFIX,
            &$user_funding.to_bytes(),
            &$user_claimer.to_bytes(),
            &$validator_id.to_bytes(),
            &$index.to_le_bytes(),
        ]
    };
}
