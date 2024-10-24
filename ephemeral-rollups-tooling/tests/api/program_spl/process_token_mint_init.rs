use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use spl_token::instruction::initialize_mint;
use spl_token::state::Mint;

use crate::api::program_context::process_instruction::{
    process_instruction, process_instruction_with_signer,
};
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_token_mint_init(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    mint: &Keypair,
    decimals: u8,
    authority: &Pubkey,
) -> Result<(), ProgramError> {
    let rent_space = Mint::LEN;
    let rent_minimum_lamports = program_context.get_rent_minimum_balance(rent_space).await?;
    let instruction_create = create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent_minimum_lamports,
        rent_space as u64,
        &spl_token::id(),
    );
    process_instruction_with_signer(program_context, instruction_create, payer, mint).await?;
    let instruction_init = initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        authority,
        Some(authority),
        decimals,
    )
    .map_err(ProgramError::Program)?;
    process_instruction(program_context, instruction_init, payer).await?;
    Ok(())
}
