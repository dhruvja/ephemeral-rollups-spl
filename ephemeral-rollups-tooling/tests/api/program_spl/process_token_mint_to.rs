use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::instruction::mint_to;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_token_mint_to(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    mint: &Pubkey,
    authority: &Keypair,
    token_account: &Pubkey,
    amount: u64,
) -> Result<(), ProgramError> {
    let instruction = mint_to(
        &spl_token::id(),
        mint,
        token_account,
        &authority.pubkey(),
        &[],
        amount,
    )
    .map_err(ProgramError::Program)?;
    process_instruction_with_signer(program_context, instruction, payer, authority).await
}
