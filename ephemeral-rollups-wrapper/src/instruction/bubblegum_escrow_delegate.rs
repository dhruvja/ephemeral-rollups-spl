use borsh::BorshSerialize;
use ephemeral_rollups_sdk::{
    consts::{BUFFER, DELEGATION_PROGRAM_ID},
    pda::{delegation_metadata_pda_from_pubkey, delegation_record_pda_from_pubkey},
};
use mpl_bubblegum::utils::get_asset_id;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::{processor::bubblegum_escrow_delegate, state::bubblegum_escrow::BubblegumEscrow};

pub fn instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    tree: &Pubkey,
    nonce: u64,
) -> Instruction {
    let program_id = crate::ID;

    let asset = get_asset_id(tree, nonce);
    let bubblegum_escrow_pda = BubblegumEscrow::generate_pda(validator, &asset, &program_id);

    let delegation_buffer_pda =
        Pubkey::find_program_address(&[BUFFER, &bubblegum_escrow_pda.to_bytes()], &program_id).0;

    let delegation_record_pda = delegation_record_pda_from_pubkey(&bubblegum_escrow_pda);
    let delegation_metadata_pda = delegation_metadata_pda_from_pubkey(&bubblegum_escrow_pda);
    let delegation_program_id = DELEGATION_PROGRAM_ID;

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new(bubblegum_escrow_pda, false),
        AccountMeta::new(delegation_buffer_pda, false),
        AccountMeta::new(delegation_record_pda, false),
        AccountMeta::new(delegation_metadata_pda, false),
        AccountMeta::new_readonly(delegation_program_id, false),
        AccountMeta::new_readonly(program_id, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&bubblegum_escrow_delegate::DISCRIMINANT);
    bubblegum_escrow_delegate::Args {
        validator: *validator,
        tree: *tree,
        nonce,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
