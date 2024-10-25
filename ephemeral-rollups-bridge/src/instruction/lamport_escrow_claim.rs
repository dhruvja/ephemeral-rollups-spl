use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::{processor::lamport_escrow_claim, state::lamport_escrow::LamportEscrow};

pub fn instruction(
    authority: &Pubkey,
    destination: &Pubkey,
    validator: &Pubkey,
    number: u64,
    lamports: u64,
) -> Instruction {
    let program_id = crate::id();
    let lamport_escrow_pda = LamportEscrow::generate_pda(authority, validator, number, &program_id);

    let accounts = vec![
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new(*destination, false),
        AccountMeta::new(lamport_escrow_pda, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&lamport_escrow_claim::DISCRIMINANT);
    lamport_escrow_claim::Args {
        validator: *validator,
        number,
        lamports,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
