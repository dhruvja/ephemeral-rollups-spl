use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct LamportEscrow {
    pub initialized: bool,
}

impl LamportEscrow {
    pub fn space() -> usize {
        size_of::<bool>()
    }
}

pub const LAMPORT_ESCROW_SEEDS_PREFIX: &[u8] = b"lamport_escrow";

#[macro_export]
macro_rules! lamport_escrow_seeds_generator {
    ($authority: expr, $validator_id: expr, $index: expr) => {
        &[
            crate::state::lamport_escrow::LAMPORT_ESCROW_SEEDS_PREFIX,
            &$authority.to_bytes(),
            &$validator_id.to_bytes(),
            &$index.to_le_bytes(),
        ]
    };
}
