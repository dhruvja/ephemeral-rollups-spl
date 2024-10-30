use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use ephemeral_rollups_wrapper::instruction::token_vault_init;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_token_vault_init(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    validator: &Pubkey,
    token_mint: &Pubkey,
) -> Result<(), ProgramError> {
    let instruction = token_vault_init::instruction(&payer.pubkey(), validator, token_mint);
    process_instruction_with_signer(program_context, instruction, payer, payer).await
}
