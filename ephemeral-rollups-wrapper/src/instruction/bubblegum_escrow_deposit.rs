use borsh::BorshSerialize;
use mpl_bubblegum::{accounts::TreeConfig, utils::get_asset_id};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::{processor::bubblegum_escrow_deposit, state::bubblegum_escrow::BubblegumEscrow};

pub fn instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    tree: &Pubkey,
    leaf_owner: &Pubkey,
    leaf_delegate: &Pubkey,
    root_hash: &[u8; 32],
    data_hash: &[u8; 32],
    creator_hash: &[u8; 32],
    nonce: u64,
    index: u32,
) -> Instruction {
    let program_id = crate::ID;

    let asset = get_asset_id(tree, nonce);
    let bubblegum_escrow_pda = BubblegumEscrow::generate_pda(validator, &asset, &program_id);

    let tree_config_pda = TreeConfig::find_pda(tree).0;

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(bubblegum_escrow_pda, false),
        AccountMeta::new(*tree, false),
        AccountMeta::new(tree_config_pda, false),
        AccountMeta::new_readonly(*leaf_owner, true),
        AccountMeta::new_readonly(*leaf_delegate, false),
        AccountMeta::new_readonly(mpl_bubblegum::ID, false),
        AccountMeta::new_readonly(spl_account_compression::ID, false),
        AccountMeta::new_readonly(spl_noop::ID, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&bubblegum_escrow_deposit::DISCRIMINANT);
    bubblegum_escrow_deposit::Args {
        authority: *authority,
        validator: *validator,
        root_hash: *root_hash,
        data_hash: *data_hash,
        creator_hash: *creator_hash,
        nonce,
        index,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}
