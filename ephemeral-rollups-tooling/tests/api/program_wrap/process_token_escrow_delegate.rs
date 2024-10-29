use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use ephemeral_rollups_wrap::instruction::token_escrow_delegate;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_token_escrow_delegate(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    authority: &Keypair,
    validator: &Pubkey,
    token_mint: &Pubkey,
    number: u64,
) -> Result<(), ProgramError> {
    let instruction = token_escrow_delegate::instruction(
        &payer.pubkey(),
        &authority.pubkey(),
        validator,
        token_mint,
        number,
    );
    process_instruction_with_signer(program_context, instruction, payer, authority).await
}
