use borsh::BorshSerialize;
use ephemeral_rollups_sdk::consts::MAGIC_CONTEXT_ID;
use ephemeral_rollups_sdk::consts::MAGIC_PROGRAM_ID;
use mpl_bubblegum::utils::get_asset_id;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;

use crate::processor::bubblegum_escrow_undelegate;
use crate::state::bubblegum_escrow::BubblegumEscrow;

pub fn instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    tree: &Pubkey,
    nonce: u64,
) -> Instruction {
    let program_id = crate::ID;

    let asset = get_asset_id(tree, nonce);
    let bubblegum_escrow_pda =
        BubblegumEscrow::generate_pda(validator, &asset, &program_id);

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new(bubblegum_escrow_pda, false),
        AccountMeta::new(MAGIC_CONTEXT_ID, false),
        AccountMeta::new_readonly(MAGIC_PROGRAM_ID, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&bubblegum_escrow_undelegate::DISCRIMINANT);
    bubblegum_escrow_undelegate::Args {
        validator: *validator,
        tree: *tree,
        nonce,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction { program_id, accounts, data }
}
