use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::{processor::lamport_escrow_create, state::lamport_escrow::LamportEscrow};

pub fn instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    number: u64,
) -> Instruction {
    let program_id = crate::id();
    let lamport_escrow_pda = LamportEscrow::generate_pda(authority, validator, number, &program_id);

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(lamport_escrow_pda, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&lamport_escrow_create::DISCRIMINANT);
    lamport_escrow_create::Args {
        authority: *authority,
        validator: *validator,
        number,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
