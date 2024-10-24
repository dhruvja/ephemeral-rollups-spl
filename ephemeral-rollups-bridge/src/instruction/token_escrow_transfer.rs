use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::{processor::token_escrow_transfer, state::token_escrow::TokenEscrow};

pub fn instruction(
    source_authority: &Pubkey,
    destination_authority: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    source_index: u64,
    destination_index: u64,
    amount: u64,
) -> Instruction {
    let program_id = crate::id();

    let source_token_escrow_pda = TokenEscrow::generate_pda(
        source_authority,
        validator,
        token_mint,
        source_index,
        &program_id,
    );
    let destination_token_escrow_pda = TokenEscrow::generate_pda(
        destination_authority,
        validator,
        token_mint,
        destination_index,
        &program_id,
    );

    let accounts = vec![
        AccountMeta::new_readonly(*source_authority, true),
        AccountMeta::new(source_token_escrow_pda, false),
        AccountMeta::new(destination_token_escrow_pda, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&token_escrow_transfer::DISCRIMINANT);
    token_escrow_transfer::Args {
        validator: *validator,
        token_mint: *token_mint,
        destination_authority: *destination_authority,
        source_index,
        destination_index,
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
