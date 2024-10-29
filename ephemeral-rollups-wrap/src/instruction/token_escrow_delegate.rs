use borsh::BorshSerialize;
use ephemeral_rollups_sdk::{
    consts::{BUFFER, DELEGATION_PROGRAM_ID},
    pda::{delegation_metadata_pda_from_pubkey, delegation_record_pda_from_pubkey},
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::{processor::token_escrow_delegate, state::token_escrow::TokenEscrow};

pub fn instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    slot: u64,
) -> Instruction {
    let program_id = crate::id();
    let token_escrow_pda =
        TokenEscrow::generate_pda(authority, validator, token_mint, slot, &program_id);

    let delegation_buffer_pda =
        Pubkey::find_program_address(&[BUFFER, &token_escrow_pda.to_bytes()], &program_id).0;

    let delegation_record_pda = delegation_record_pda_from_pubkey(&token_escrow_pda);
    let delegation_metadata_pda = delegation_metadata_pda_from_pubkey(&token_escrow_pda);
    let delegation_program_id = DELEGATION_PROGRAM_ID;

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new(token_escrow_pda, false),
        AccountMeta::new(delegation_buffer_pda, false),
        AccountMeta::new(delegation_record_pda, false),
        AccountMeta::new(delegation_metadata_pda, false),
        AccountMeta::new_readonly(delegation_program_id, false),
        AccountMeta::new_readonly(program_id, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&token_escrow_delegate::DISCRIMINANT);
    token_escrow_delegate::Args {
        validator: *validator,
        token_mint: *token_mint,
        slot,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
