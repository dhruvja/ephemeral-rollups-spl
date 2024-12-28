use borsh::BorshSerialize;
use ephemeral_rollups_sdk::consts::MAGIC_CONTEXT_ID;
use ephemeral_rollups_sdk::consts::MAGIC_PROGRAM_ID;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;

use crate::processor::lamport_escrow_undelegate;
use crate::state::lamport_escrow::LamportEscrow;

pub fn instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    slot: u64,
) -> Instruction {
    let program_id = crate::ID;
    let lamport_escrow_pda =
        LamportEscrow::generate_pda(authority, validator, slot, &program_id);

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new(lamport_escrow_pda, false),
        AccountMeta::new(MAGIC_CONTEXT_ID, false),
        AccountMeta::new_readonly(MAGIC_PROGRAM_ID, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&lamport_escrow_undelegate::DISCRIMINANT);
    lamport_escrow_undelegate::Args { validator: *validator, slot }
        .serialize(&mut data)
        .unwrap();

    Instruction { program_id, accounts, data }
}
