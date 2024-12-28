use std::mem::size_of;

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

use crate::lamport_escrow_seeds_generator;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct LamportEscrow {
    pub discriminant: u64,
}

impl LamportEscrow {
    pub fn discriminant() -> u64 {
        0x93DE9B7883E25473
    }

    pub fn space() -> usize {
        size_of::<u64>()
    }

    pub fn generate_pda(
        authority: &Pubkey,
        validator: &Pubkey,
        slot: u64,
        program_id: &Pubkey,
    ) -> Pubkey {
        Pubkey::find_program_address(
            lamport_escrow_seeds_generator!(authority, validator, slot),
            program_id,
        )
        .0
    }
}

pub const LAMPORT_ESCROW_SEEDS_PREFIX: &[u8] = b"lamport_escrow";

#[macro_export]
macro_rules! lamport_escrow_seeds_generator {
    ($authority:expr, $validator:expr, $slot:expr) => {
        &[
            $crate::state::lamport_escrow::LAMPORT_ESCROW_SEEDS_PREFIX,
            &$authority.to_bytes(),
            &$validator.to_bytes(),
            &$slot.to_le_bytes(),
        ]
    };
}
