use ephemeral_rollups_wrapper::state::token_escrow::TokenEscrow;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};
use spl_token::state::Account;

use crate::api::create_program_test_context::create_program_test_context;

use crate::api::program_wrapper::process_token_escrow_create::process_token_escrow_create;
use crate::api::program_wrapper::process_token_escrow_deposit::process_token_escrow_deposit;
use crate::api::program_wrapper::process_token_escrow_transfer::process_token_escrow_transfer;
use crate::api::program_wrapper::process_token_escrow_withdraw::process_token_escrow_withdraw;
use crate::api::program_wrapper::process_token_vault_init::process_token_vault_init;

#[tokio::test]
async fn localnet_token_escrow_create_deposit_transfer_withdraw() -> Result<(), EndpointError> {
    let mut endpoint = Endpoint::from(create_program_test_context().await);

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();

    let authority1 = Keypair::new();
    let authority2 = Keypair::new();

    let source = Keypair::new();
    let destination = Keypair::new();

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
            &source.pubkey(),
            &token_mint.pubkey(),
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

    // Escrow accounts we will be creating
    let authority1_token_escrow_slot = 99;
    let authority1_token_escrow_pda = TokenEscrow::generate_pda(
        &authority1.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority1_token_escrow_slot,
        &ephemeral_rollups_wrapper::ID,
    );
    let authority2_token_escrow_slot = 42;
    let authority2_token_escrow_pda = TokenEscrow::generate_pda(
        &authority2.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority2_token_escrow_slot,
        &ephemeral_rollups_wrapper::ID,
    );

    // Prepare being able to escrow token for this validator
    process_token_vault_init(&mut endpoint, &payer, &validator, &token_mint.pubkey()).await?;

    // Create an escrow
    process_token_escrow_create(
        &mut endpoint,
        &payer,
        &authority1.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority1_token_escrow_slot,
    )
    .await?;

    // No balance yet
    assert_eq!(
        0,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority1_token_escrow_pda)
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
        &authority1.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority1_token_escrow_slot,
        10_000_000,
    )
    .await?;

    // New balance
    assert_eq!(
        10_000_000,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority1_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );

    // Fund the escrow some more
    process_token_escrow_deposit(
        &mut endpoint,
        &payer,
        &source,
        &source_token,
        &authority1.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority1_token_escrow_slot,
        90_000_000,
    )
    .await?;

    // New balance
    assert_eq!(
        100_000_000,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority1_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );

    // Create a different escrow (but this one is unfunded)
    process_token_escrow_create(
        &mut endpoint,
        &payer,
        &authority2.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority2_token_escrow_slot,
    )
    .await?;

    // New escrow
    assert_eq!(
        0,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority2_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );

    // Transfer some from 1->2
    process_token_escrow_transfer(
        &mut endpoint,
        &payer,
        &authority1,
        &authority2.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority1_token_escrow_slot,
        authority2_token_escrow_slot,
        1_000_000,
    )
    .await?;

    // Transfer success should be reflected in the balances
    assert_eq!(
        99_000_000,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority1_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );
    assert_eq!(
        1_000_000,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority2_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );

    // Transfer all remaining from 1->2
    process_token_escrow_transfer(
        &mut endpoint,
        &payer,
        &authority1,
        &authority2.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority1_token_escrow_slot,
        authority2_token_escrow_slot,
        99_000_000,
    )
    .await?;

    // Transfer success should be reflected in the balances
    assert_eq!(
        0,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority1_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );
    assert_eq!(
        100_000_000,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority2_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );

    // Transfer back most of it back 2->1
    process_token_escrow_transfer(
        &mut endpoint,
        &payer,
        &authority2,
        &authority1.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority2_token_escrow_slot,
        authority1_token_escrow_slot,
        75_000_000,
    )
    .await?;

    // Transfer success should be reflected in the balances
    assert_eq!(
        75_000_000,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority1_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );
    assert_eq!(
        25_000_000,
        endpoint
            .get_account_data_borsh_deserialized::<TokenEscrow>(&authority2_token_escrow_pda)
            .await?
            .unwrap()
            .amount
    );

    // Withdraw everything after that
    let destination_token = endpoint
        .process_spl_associated_token_account_get_or_init(
            &payer,
            &destination.pubkey(),
            &token_mint.pubkey(),
        )
        .await?;

    // There should be most of it in the first escrow
    process_token_escrow_withdraw(
        &mut endpoint,
        &payer,
        &authority1,
        &destination_token,
        &validator,
        &token_mint.pubkey(),
        authority1_token_escrow_slot,
        75_000_000,
    )
    .await?;

    // And there should be exactly the remaining amount in the other one
    process_token_escrow_withdraw(
        &mut endpoint,
        &payer,
        &authority2,
        &destination_token,
        &validator,
        &token_mint.pubkey(),
        authority2_token_escrow_slot,
        25_000_000,
    )
    .await?;

    // Verify that the on-chain destination token account now has our tokens
    assert_eq!(
        100_000_000,
        endpoint
            .get_account_data_unpacked::<Account>(&destination_token)
            .await?
            .unwrap()
            .amount
    );

    // Done
    Ok(())
}
