use borsh::BorshSerialize;
use mpl_bubblegum::utils::get_asset_id;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::{processor::bubblegum_escrow_transfer, state::bubblegum_escrow::BubblegumEscrow};

pub fn instruction(
    //payer: &Pubkey,
    source_authority: &Pubkey,
    destination_authority: &Pubkey,
    validator: &Pubkey,
    tree: &Pubkey,
    nonce: u64,
) -> Instruction {
    let program_id = crate::ID;

    let asset = get_asset_id(tree, nonce);
    let bubblegum_escrow_pda = BubblegumEscrow::generate_pda(validator, &asset, &program_id);

    let accounts = vec![
        AccountMeta::new_readonly(*source_authority, true),
        AccountMeta::new(bubblegum_escrow_pda, false),
        AccountMeta::new_readonly(*tree, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&bubblegum_escrow_transfer::DISCRIMINANT);
    bubblegum_escrow_transfer::Args {
        destination_authority: *destination_authority,
        validator: *validator,
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
