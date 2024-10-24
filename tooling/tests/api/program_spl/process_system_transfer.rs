use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::transfer;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_system_transfer(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    source: &Keypair,
    destination: &Pubkey,
    lamports: u64,
) -> Result<(), ProgramError> {
    let instruction = transfer(&source.pubkey(), destination, lamports);

    process_instruction_with_signer(program_context, instruction, payer, source).await
}
