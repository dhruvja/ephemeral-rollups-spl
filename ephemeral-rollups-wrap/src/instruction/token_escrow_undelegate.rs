use borsh::BorshSerialize;
use ephemeral_rollups_sdk::consts::{MAGIC_CONTEXT_ID, MAGIC_PROGRAM_ID};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::{processor::token_escrow_undelegate, state::token_escrow::TokenEscrow};

pub fn instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    number: u64,
) -> Instruction {
    let program_id = crate::id();
    let token_escrow_pda =
        TokenEscrow::generate_pda(authority, validator, token_mint, number, &program_id);

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new(token_escrow_pda, false),
        AccountMeta::new(MAGIC_CONTEXT_ID, false),
        AccountMeta::new_readonly(MAGIC_PROGRAM_ID, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&token_escrow_undelegate::DISCRIMINANT);
    token_escrow_undelegate::Args {
        validator: *validator,
        token_mint: *token_mint,
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
