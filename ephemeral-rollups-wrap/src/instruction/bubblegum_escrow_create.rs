use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::{processor::bubblegum_escrow_create, state::bubblegum_escrow::BubblegumEscrow};

pub fn instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    slot: u64,
) -> Instruction {
    let program_id = crate::id();
    let bubblegum_escrow_pda = BubblegumEscrow::generate_pda(authority, validator, &program_id);

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(bubblegum_escrow_pda, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&bubblegum_escrow_create::DISCRIMINANT);
    bubblegum_escrow_create::Args {
        authority: *authority,
        validator: *validator,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
