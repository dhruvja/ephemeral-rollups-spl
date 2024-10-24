use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::api::program_bridge::process_token_escrow_create::process_token_escrow_create;
use crate::api::program_bridge::process_token_escrow_deposit::process_token_escrow_deposit;
use crate::api::program_bridge::process_token_escrow_transfer::process_token_escrow_transfer;
use crate::api::program_bridge::process_token_escrow_withdraw::process_token_escrow_withdraw;
use crate::api::program_bridge::process_token_vault_init::process_token_vault_init;
use crate::api::program_context::create_program_test_context::create_program_test_context;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;
use crate::api::program_spl::process_associated_token_account_get_or_init::process_associated_token_account_get_or_init;
use crate::api::program_spl::process_token_mint_init::process_token_mint_init;
use crate::api::program_spl::process_token_mint_to::process_token_mint_to;

#[tokio::test]
async fn test_localnet_token_escrowing() -> Result<(), ProgramError> {
    let mut program_context: Box<dyn ProgramContext> =
        Box::new(create_program_test_context().await);

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();
    let authority = Keypair::new();
    let source = Keypair::new();
    let destination = Keypair::new();

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
        6,
        &usdc_mint.pubkey(),
    )
    .await?;

    // Airdrop USDC to our source wallet
    let source_usdc = process_associated_token_account_get_or_init(
        &mut program_context,
        &payer,
        &usdc_mint.pubkey(),
        &source.pubkey(),
    )
    .await?;
    process_token_mint_to(
        &mut program_context,
        &payer,
        &usdc_mint.pubkey(),
        &usdc_mint,
        &source_usdc,
        100_000_000,
    )
    .await?;

    // Prepare being able to escrow USDC for this validator
    process_token_vault_init(
        &mut program_context,
        &payer,
        &validator,
        &usdc_mint.pubkey(),
    )
    .await?;

    // Create an escrow
    process_token_escrow_create(
        &mut program_context,
        &payer,
        &authority.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        42,
    )
    .await?;

    // Fund the escrow
    process_token_escrow_deposit(
        &mut program_context,
        &payer,
        &source,
        &source_usdc,
        &authority.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        42,
        10_000_000,
    )
    .await?;

    process_token_escrow_deposit(
        &mut program_context,
        &payer,
        &source,
        &source_usdc,
        &authority.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        42,
        90_000_000,
    )
    .await?;

    // Create a different escrow
    process_token_escrow_create(
        &mut program_context,
        &payer,
        &authority.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        41,
    )
    .await?;

    // Do a bunch of transfers between escrows

    process_token_escrow_transfer(
        &mut program_context,
        &payer,
        &authority,
        &authority.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        42,
        41,
        1_000_000,
    )
    .await?;

    process_token_escrow_transfer(
        &mut program_context,
        &payer,
        &authority,
        &authority.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        42,
        41,
        49_000_000,
    )
    .await?;

    process_token_escrow_transfer(
        &mut program_context,
        &payer,
        &authority,
        &authority.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        41,
        42,
        25_000_000,
    )
    .await?;

    let destination_usdc = process_associated_token_account_get_or_init(
        &mut program_context,
        &payer,
        &usdc_mint.pubkey(),
        &destination.pubkey(),
    )
    .await?;

    process_token_escrow_withdraw(
        &mut program_context,
        &payer,
        &authority,
        &destination_usdc,
        &validator,
        &usdc_mint.pubkey(),
        42,
        75_000_000,
    )
    .await?;

    process_token_escrow_withdraw(
        &mut program_context,
        &payer,
        &authority,
        &destination_usdc,
        &validator,
        &usdc_mint.pubkey(),
        41,
        25_000_000,
    )
    .await?;

    // Done
    Ok(())
}
