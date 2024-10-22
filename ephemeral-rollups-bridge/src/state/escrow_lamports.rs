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
