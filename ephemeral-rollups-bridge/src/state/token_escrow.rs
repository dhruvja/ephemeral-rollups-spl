use std::mem::size_of;

use borsh::{BorshDeserialize, BorshSerialize};

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
}

pub const TOKEN_ESCROW_SEEDS_PREFIX: &[u8] = b"token_escrow";

#[macro_export]
macro_rules! token_escrow_seeds_generator {
    ($authority: expr, $validator: expr, $token_mint: expr, $index: expr) => {
        &[
            crate::state::token_escrow::TOKEN_ESCROW_SEEDS_PREFIX,
            &$authority.to_bytes(),
            &$validator.to_bytes(),
            &$token_mint.to_bytes(),
            &$index.to_le_bytes(),
        ]
    };
}
