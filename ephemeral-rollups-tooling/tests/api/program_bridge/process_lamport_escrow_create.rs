use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use ephemeral_rollups_bridge::instruction::lamport_escrow_create;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_lamport_escrow_create(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    authority: &Pubkey,
    validator: &Pubkey,
    number: u64,
) -> Result<(), ProgramError> {
    let instruction =
        lamport_escrow_create::instruction(&payer.pubkey(), authority, validator, number);
    process_instruction_with_signer(program_context, instruction, payer, payer).await
}
