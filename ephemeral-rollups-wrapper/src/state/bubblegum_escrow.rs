use std::mem::size_of;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::bubblegum_escrow_seeds_generator;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct BubblegumEscrow {
    pub discriminant: u64,
}

impl BubblegumEscrow {
    pub fn discriminant() -> u64 {
        0xf9a101d13bffaefc
    }

    pub fn space() -> usize {
        size_of::<u64>() + size_of::<u64>()
    }

    pub fn generate_pda(
        authority: &Pubkey,
        validator: &Pubkey,
        asset_id: &Pubkey,
        program_id: &Pubkey,
    ) -> Pubkey {
        Pubkey::find_program_address(
            bubblegum_escrow_seeds_generator!(authority, validator, asset_id),
            program_id,
        )
        .0
    }
}

pub const CNFT_ESCROW_SEEDS_PREFIX: &[u8] = b"cnft_escrow";

#[macro_export]
macro_rules! bubblegum_escrow_seeds_generator {
    ($authority: expr, $validator: expr, $asset_id: expr) => {
        &[
            $crate::state::bubblegum_escrow::CNFT_ESCROW_SEEDS_PREFIX,
            &$authority.to_bytes(),
            &$validator.to_bytes(),
            &$asset_id.to_bytes(),
        ]
    };
}
