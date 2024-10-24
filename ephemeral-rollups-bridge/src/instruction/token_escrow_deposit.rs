use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::{
    processor::token_escrow_deposit,
    state::{token_escrow::TokenEscrow, token_vault::token_vault_generate_pda},
};

pub fn instruction(
    source_authority: &Pubkey,
    source_token_account: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    index: u64,
    amount: u64,
) -> Instruction {
    let program_id = crate::id();
    let token_escrow_pda =
        TokenEscrow::generate_pda(authority, validator, token_mint, index, &program_id);
    let token_vault_pda = token_vault_generate_pda(validator, token_mint, &program_id);

    let accounts = vec![
        AccountMeta::new_readonly(*source_authority, true),
        AccountMeta::new(*source_token_account, false),
        AccountMeta::new(token_escrow_pda, false),
        AccountMeta::new(token_vault_pda, false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&token_escrow_deposit::DISCRIMINANT);
    token_escrow_deposit::Args {
        authority: *authority,
        validator: *validator,
        token_mint: *token_mint,
        index,
        amount,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
