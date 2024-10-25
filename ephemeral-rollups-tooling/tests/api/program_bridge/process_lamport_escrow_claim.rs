use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use ephemeral_rollups_bridge::instruction::lamport_escrow_claim;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_lamport_escrow_claim(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    authority: &Keypair,
    destination: &Pubkey,
    validator: &Pubkey,
    number: u64,
    lamports: u64,
) -> Result<(), ProgramError> {
    let instruction = lamport_escrow_claim::instruction(
        &authority.pubkey(),
        destination,
        validator,
        number,
        lamports,
    );
    process_instruction_with_signer(program_context, instruction, payer, authority).await
}
