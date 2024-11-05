use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_system_create(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    account: &Keypair,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
) -> Result<(), ProgramError> {
    let instruction = create_account(&payer.pubkey(), &account.pubkey(), lamports, space, owner);
    process_instruction_with_signer(program_context, instruction, payer, account).await
}
