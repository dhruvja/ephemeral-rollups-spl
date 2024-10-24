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
async fn test_localnet_token_escrow_create_deposit_transfer_withdraw() -> Result<(), ProgramError> {
    let mut program_context: Box<dyn ProgramContext> =
        Box::new(create_program_test_context().await);

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();

    let authority1 = Keypair::new();
    let authority2 = Keypair::new();

    let index1 = 99;
    let index2 = 42;

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
        &authority1.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        index1,
    )
    .await?;

    // Fund the escrow
    process_token_escrow_deposit(
        &mut program_context,
        &payer,
        &source,
        &source_usdc,
        &authority1.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        index1,
        10_000_000,
    )
    .await?;

    // Fund the escrow some more
    process_token_escrow_deposit(
        &mut program_context,
        &payer,
        &source,
        &source_usdc,
        &authority1.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        index1,
        90_000_000,
    )
    .await?;

    // Create a different escrow (but this one is unfunded)
    process_token_escrow_create(
        &mut program_context,
        &payer,
        &authority2.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        index2,
    )
    .await?;

    // Transfer some from 1->2
    process_token_escrow_transfer(
        &mut program_context,
        &payer,
        &authority1,
        &authority2.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        index1,
        index2,
        1_000_000,
    )
    .await?;

    // Transfer all remaining from 1->2
    process_token_escrow_transfer(
        &mut program_context,
        &payer,
        &authority1,
        &authority2.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        index1,
        index2,
        99_000_000,
    )
    .await?;

    // Transfer back most of it back 2->1
    process_token_escrow_transfer(
        &mut program_context,
        &payer,
        &authority2,
        &authority1.pubkey(),
        &validator,
        &usdc_mint.pubkey(),
        index2,
        index1,
        75_000_000,
    )
    .await?;

    // Withdraw everything after that
    let destination_usdc = process_associated_token_account_get_or_init(
        &mut program_context,
        &payer,
        &usdc_mint.pubkey(),
        &destination.pubkey(),
    )
    .await?;

    // There should be most of it in the first escrow
    process_token_escrow_withdraw(
        &mut program_context,
        &payer,
        &authority1,
        &destination_usdc,
        &validator,
        &usdc_mint.pubkey(),
        index1,
        75_000_000,
    )
    .await?;

    // And there should be exactly the remaining amount in the other one
    process_token_escrow_withdraw(
        &mut program_context,
        &payer,
        &authority2,
        &destination_usdc,
        &validator,
        &usdc_mint.pubkey(),
        index2,
        25_000_000,
    )
    .await?;

    // Done
    Ok(())
}
