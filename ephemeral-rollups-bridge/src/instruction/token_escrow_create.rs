use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::{processor::token_escrow_create, state::token_escrow::TokenEscrow};

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
        AccountMeta::new(token_escrow_pda, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&token_escrow_create::DISCRIMINANT);
    token_escrow_create::Args {
        authority: *authority,
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
