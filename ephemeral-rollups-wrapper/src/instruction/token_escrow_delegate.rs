use borsh::BorshSerialize;
use ephemeral_rollups_sdk::consts::BUFFER;
use ephemeral_rollups_sdk::consts::DELEGATION_PROGRAM_ID;
use ephemeral_rollups_sdk::pda::delegation_metadata_pda_from_delegated_account;
use ephemeral_rollups_sdk::pda::delegation_record_pda_from_delegated_account;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::processor::token_escrow_delegate;
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

    let delegation_buffer_pda = Pubkey::find_program_address(
        &[BUFFER, &token_escrow_pda.to_bytes()],
        &program_id,
    )
    .0;

    let delegation_record_pda =
        delegation_record_pda_from_delegated_account(&token_escrow_pda);
    let delegation_metadata_pda =
        delegation_metadata_pda_from_delegated_account(&token_escrow_pda);
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
        AccountMeta::new_readonly(system_program::ID, false),
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

    Instruction { program_id, accounts, data }
}
