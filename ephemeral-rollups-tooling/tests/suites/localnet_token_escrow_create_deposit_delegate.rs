use ephemeral_rollups_wrapper::state::token_escrow::TokenEscrow;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

use crate::api::create_program_test_context::create_program_test_context;

use crate::api::program_wrapper::process_token_escrow_create::process_token_escrow_create;
use crate::api::program_wrapper::process_token_escrow_delegate::process_token_escrow_delegate;
use crate::api::program_wrapper::process_token_escrow_deposit::process_token_escrow_deposit;
use crate::api::program_wrapper::process_token_vault_init::process_token_vault_init;

#[tokio::test]
async fn localnet_token_escrow_create_deposit_delegate() -> Result<(), EndpointError> {
    let mut endpoint = Endpoint::from(create_program_test_context().await);

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();
    let source = Keypair::new();
    let authority = Keypair::new();

    // Fund payer
    endpoint
        .process_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await?;

    // Create token mint
    let token_mint = Keypair::new();
    endpoint
        .process_spl_token_mint_init(&payer, &token_mint, &token_mint.pubkey(), 6)
        .await?;

    // Airdrop token to our source wallet
    let source_token = endpoint
        .process_spl_associated_token_account_get_or_init(
            &payer,
            &token_mint.pubkey(),
            &source.pubkey(),
        )
        .await?;
    endpoint
        .process_spl_token_mint_to(
            &payer,
            &token_mint.pubkey(),
            &token_mint,
            &source_token,
            100_000_000,
        )
        .await?;

    // Escrow account we will be creating
    let authority_token_escrow_slot = 99;
    let authority_token_escrow_pda = TokenEscrow::generate_pda(
        &authority.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority_token_escrow_slot,
        &ephemeral_rollups_wrapper::ID,
    );

    // Prepare being able to escrow token for this validator
    process_token_vault_init(&mut endpoint, &payer, &validator, &token_mint.pubkey()).await?;

    // Create an escrow
    process_token_escrow_create(
        &mut endpoint,
        &payer,
        &authority.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority_token_escrow_slot,
    )
    .await?;

    // No balance yet
    assert_eq!(
        0,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );

    // Fund the escrow
    process_token_escrow_deposit(
        &mut endpoint,
        &payer,
        &source,
        &source_token,
        &authority.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority_token_escrow_slot,
        10_000_000,
    )
    .await?;

    // New balance
    assert_eq!(
        10_000_000,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );

    // Delegate the balance we just deposited
    process_token_escrow_delegate(
        &mut endpoint,
        &payer,
        &authority,
        &validator,
        &token_mint.pubkey(),
        authority_token_escrow_slot,
    )
    .await?;

    // Done
    Ok(())
}
