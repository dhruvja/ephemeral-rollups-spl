use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use ephemeral_rollups_wrapper::instruction::bubblegum_escrow_withdraw;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_bubblegum_escrow_withdraw(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    authority: &Keypair,
    receiver: &Pubkey,
    validator: &Pubkey,
    spill: &Pubkey,
    tree: &Pubkey,
    root_hash: &[u8; 32],
    data_hash: &[u8; 32],
    creator_hash: &[u8; 32],
    nonce: u64,
    index: u32,
) -> Result<(), ProgramError> {
    let instruction = bubblegum_escrow_withdraw::instruction(
        &authority.pubkey(),
        receiver,
        validator,
        spill,
        tree,
        root_hash,
        data_hash,
        creator_hash,
        nonce,
        index,
    );
    process_instruction_with_signer(program_context, instruction, payer, authority).await
}
