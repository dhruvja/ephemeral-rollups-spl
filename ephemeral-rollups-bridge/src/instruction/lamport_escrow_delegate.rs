use borsh::BorshSerialize;
use ephemeral_rollups_sdk::{
    consts::DELEGATION_PROGRAM_ID,
    pda::{
        buffer_pda_from_pubkey, delegation_metadata_pda_from_pubkey,
        delegation_record_pda_from_pubkey,
    },
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::{processor::lamport_escrow_delegate, state::lamport_escrow::LamportEscrow};

pub fn instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    index: u64,
) -> Instruction {
    let program_id = crate::id();
    let lamport_escrow_pda = LamportEscrow::generate_pda(authority, validator, index, &program_id);

    let delegation_buffer_pda = buffer_pda_from_pubkey(&lamport_escrow_pda);
    let delegation_record_pda = delegation_record_pda_from_pubkey(&lamport_escrow_pda);
    let delegation_metadata_pda = delegation_metadata_pda_from_pubkey(&lamport_escrow_pda);
    let delegation_program_id = DELEGATION_PROGRAM_ID;

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new(lamport_escrow_pda, false),
        AccountMeta::new(delegation_buffer_pda, false),
        AccountMeta::new(delegation_record_pda, false),
        AccountMeta::new(delegation_metadata_pda, false),
        AccountMeta::new_readonly(delegation_program_id, false),
        AccountMeta::new_readonly(program_id, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&lamport_escrow_delegate::DISCRIMINANT);
    lamport_escrow_delegate::Args {
        validator: *validator,
        index,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
