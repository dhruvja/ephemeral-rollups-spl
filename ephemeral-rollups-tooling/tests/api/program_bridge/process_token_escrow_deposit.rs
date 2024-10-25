use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use ephemeral_rollups_bridge::instruction::token_escrow_deposit;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_token_escrow_deposit(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    source_authority: &Keypair,
    source_token_account: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    number: u64,
    amount: u64,
) -> Result<(), ProgramError> {
    let instruction = token_escrow_deposit::instruction(
        &source_authority.pubkey(),
        source_token_account,
        authority,
        validator,
        token_mint,
        number,
        amount,
    );
    process_instruction_with_signer(program_context, instruction, payer, source_authority).await
}
