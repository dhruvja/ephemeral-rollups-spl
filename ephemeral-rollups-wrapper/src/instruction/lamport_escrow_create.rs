use borsh::BorshSerialize;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::processor::lamport_escrow_create;
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
        AccountMeta::new(lamport_escrow_pda, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&lamport_escrow_create::DISCRIMINANT);
    lamport_escrow_create::Args {
        authority: *authority,
        validator: *validator,
        slot,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction { program_id, accounts, data }
}
