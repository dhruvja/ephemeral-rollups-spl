use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use spl_token::ui_amount_to_amount;

use crate::api::program_bridge::process_lamport_escrow_create::process_lamport_escrow_create;
use crate::api::program_context::create_program_test_context::create_program_test_context;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;
use crate::api::program_spl::process_associated_token_account_get_or_init::process_associated_token_account_get_or_init;
use crate::api::program_spl::process_token_mint_init::process_token_mint_init;
use crate::api::program_spl::process_token_mint_to::process_token_mint_to;

#[tokio::test]
async fn test_localnet_basic_success() -> Result<(), ProgramError> {
    let mut program_context: Box<dyn ProgramContext> =
        Box::new(create_program_test_context().await);

    // Participating wallets
    let payer = Keypair::new();
    let authority = Keypair::new();
    let validator = Keypair::new();

    // Mints decimals
    let usdc_mint_decimals = 6;

    // Important amounts
    let liquid_usdc_amount = ui_amount_to_amount(50_000_000., usdc_mint_decimals);

    // Fund payer
    program_context
        .process_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await?;

    // Create USDC mint
    let usdc_mint = Keypair::new();
    process_token_mint_init(
        &mut program_context,
        &payer,
        &usdc_mint,
        usdc_mint_decimals,
        &usdc_mint.pubkey(),
    )
    .await?;

    // Airdrop USDC to our authority wallet
    let authority_usdc = process_associated_token_account_get_or_init(
        &mut program_context,
        &payer,
        &usdc_mint.pubkey(),
        &authority.pubkey(),
    )
    .await?;
    process_token_mint_to(
        &mut program_context,
        &payer,
        &usdc_mint.pubkey(),
        &usdc_mint,
        &authority_usdc,
        liquid_usdc_amount,
    )
    .await?;

    process_lamport_escrow_create(
        &mut program_context,
        &payer,
        &authority.pubkey(),
        &validator.pubkey(),
        42,
    )
    .await?;

    // Done
    Ok(())
}
