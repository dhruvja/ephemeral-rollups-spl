use std::mem::size_of;

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

use crate::token_escrow_seeds_generator;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct TokenEscrow {
    pub discriminant: u64,
    pub amount: u64,
}

impl TokenEscrow {
    pub fn discriminant() -> u64 {
        0xA48DAD00222D65A5
    }

    pub fn space() -> usize {
        size_of::<u64>() + size_of::<u64>()
    }

    pub fn generate_pda(
        authority: &Pubkey,
        validator: &Pubkey,
        token_mint: &Pubkey,
        slot: u64,
        program_id: &Pubkey,
    ) -> Pubkey {
        Pubkey::find_program_address(
            token_escrow_seeds_generator!(
                authority, validator, token_mint, slot
            ),
            program_id,
        )
        .0
    }
}

pub const TOKEN_ESCROW_SEEDS_PREFIX: &[u8] = b"token_escrow";

#[macro_export]
macro_rules! token_escrow_seeds_generator {
    ($authority:expr, $validator:expr, $token_mint:expr, $slot:expr) => {
        &[
            $crate::state::token_escrow::TOKEN_ESCROW_SEEDS_PREFIX,
            &$authority.to_bytes(),
            &$validator.to_bytes(),
            &$token_mint.to_bytes(),
            &$slot.to_le_bytes(),
        ]
    };
}
