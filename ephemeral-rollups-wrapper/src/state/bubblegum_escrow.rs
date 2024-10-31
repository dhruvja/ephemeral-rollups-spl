use std::mem::size_of;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::bubblegum_escrow_seeds_generator;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct BubblegumEscrow {
    pub discriminant: u64,
    pub authority: Pubkey,
}

impl BubblegumEscrow {
    pub fn discriminant() -> u64 {
        0xf9a101d13bffaefc
    }

    pub fn space() -> usize {
        size_of::<u64>() + size_of::<Pubkey>()
    }

    pub fn generate_pda(validator: &Pubkey, asset: &Pubkey, program_id: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            bubblegum_escrow_seeds_generator!(validator, asset),
            program_id,
        )
        .0
    }
}

pub const BUBBLEGUM_ESCROW_SEEDS_PREFIX: &[u8] = b"bubblegum_escrow";

#[macro_export]
macro_rules! bubblegum_escrow_seeds_generator {
    ($validator: expr, $asset_id: expr) => {
        &[
            $crate::state::bubblegum_escrow::BUBBLEGUM_ESCROW_SEEDS_PREFIX,
            &$validator.to_bytes(),
            &$asset_id.to_bytes(),
        ]
    };
}
