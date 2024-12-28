use borsh::BorshSerialize;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::processor::token_escrow_create;
use crate::state::token_escrow::TokenEscrow;

pub fn instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    slot: u64,
) -> Instruction {
    let program_id = crate::ID;
    let token_escrow_pda = TokenEscrow::generate_pda(
        authority,
        validator,
        token_mint,
        slot,
        &program_id,
    );

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(token_escrow_pda, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&token_escrow_create::DISCRIMINANT);
    token_escrow_create::Args {
        authority: *authority,
        validator: *validator,
        token_mint: *token_mint,
        slot,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction { program_id, accounts, data }
}
