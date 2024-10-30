use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::{
    processor::token_escrow_withdraw,
    state::{token_escrow::TokenEscrow, token_vault::token_vault_generate_pda},
};

pub fn instruction(
    authority: &Pubkey,
    destination_token_account: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    slot: u64,
    amount: u64,
) -> Instruction {
    let program_id = crate::ID;

    let token_escrow_pda =
        TokenEscrow::generate_pda(authority, validator, token_mint, slot, &program_id);
    let token_vault_pda = token_vault_generate_pda(validator, token_mint, &program_id);

    let accounts = vec![
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new(*destination_token_account, false),
        AccountMeta::new(token_escrow_pda, false),
        AccountMeta::new(token_vault_pda, false),
        AccountMeta::new_readonly(spl_token::ID, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&token_escrow_withdraw::DISCRIMINANT);
    token_escrow_withdraw::Args {
        validator: *validator,
        token_mint: *token_mint,
        slot,
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
