use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use ephemeral_rollups_wrapper::instruction::bubblegum_escrow_deposit;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_bubblegum_escrow_deposit(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    authority: &Pubkey,
    validator: &Pubkey,
    tree: &Pubkey,
    leaf_owner: &Keypair,
    leaf_delegate: &Pubkey,
    root_hash: &[u8; 32],
    data_hash: &[u8; 32],
    creator_hash: &[u8; 32],
    nonce: u64,
    index: u32,
) -> Result<(), ProgramError> {
    let instruction = bubblegum_escrow_deposit::instruction(
        &payer.pubkey(),
        authority,
        validator,
        tree,
        &leaf_owner.pubkey(),
        leaf_delegate,
        root_hash,
        data_hash,
        creator_hash,
        nonce,
        index,
    );
    process_instruction_with_signer(program_context, instruction, payer, leaf_owner).await
}
