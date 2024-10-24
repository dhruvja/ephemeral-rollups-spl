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
    index: u64,
) -> Instruction {
    let program_id = crate::id();
    let token_escrow_pda =
        TokenEscrow::generate_pda(authority, validator, token_mint, index, &program_id);

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new_readonly(*validator, false),
        AccountMeta::new_readonly(*token_mint, false),
        AccountMeta::new(token_escrow_pda, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&token_escrow_create::DISCRIMINANT);
    token_escrow_create::Args { index }
        .serialize(&mut data)
        .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
