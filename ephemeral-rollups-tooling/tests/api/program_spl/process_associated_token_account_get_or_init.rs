use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::api::program_context::process_instruction::process_instruction;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;
use crate::api::program_context::read_account::read_account_exists;

pub async fn process_associated_token_account_get_or_init(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    mint: &Pubkey,
    wallet: &Pubkey,
) -> Result<Pubkey, ProgramError> {
    let pda = spl_associated_token_account::get_associated_token_address(wallet, mint);
    if read_account_exists(program_context, &pda).await? {
        return Ok(pda);
    }
    let instruction =
        spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            &payer.pubkey(),
            wallet,
            mint,
            &spl_token::ID,
        );
    process_instruction(program_context, instruction, payer).await?;
    Ok(pda)
}
