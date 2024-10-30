use ephemeral_rollups_wrapper::state::token_escrow::TokenEscrow;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_token::state::Account;

use crate::api::program_context::create_program_test_context::create_program_test_context;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;
use crate::api::program_context::read_account::{read_account_borsh, read_account_packed};
use crate::api::program_spl::process_associated_token_account_get_or_init::process_associated_token_account_get_or_init;
use crate::api::program_spl::process_token_mint_init::process_token_mint_init;
use crate::api::program_spl::process_token_mint_to::process_token_mint_to;
use crate::api::program_wrapper::process_token_escrow_create::process_token_escrow_create;
use crate::api::program_wrapper::process_token_escrow_deposit::process_token_escrow_deposit;
use crate::api::program_wrapper::process_token_escrow_transfer::process_token_escrow_transfer;
use crate::api::program_wrapper::process_token_escrow_withdraw::process_token_escrow_withdraw;
use crate::api::program_wrapper::process_token_vault_init::process_token_vault_init;

#[tokio::test]
async fn localnet_token_escrow_create_deposit_transfer_withdraw() -> Result<(), ProgramError> {
    let mut program_context: Box<dyn ProgramContext> =
        Box::new(create_program_test_context().await);

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();

    let authority1 = Keypair::new();
    let authority2 = Keypair::new();

    let source = Keypair::new();
    let destination = Keypair::new();

    // Fund payer
    program_context
        .process_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await?;

    // Create token mint
    let token_mint = Keypair::new();
    process_token_mint_init(
        &mut program_context,
        &payer,
        &token_mint,
        6,
        &token_mint.pubkey(),
    )
    .await?;

    // Airdrop token to our source wallet
    let source_token = process_associated_token_account_get_or_init(
        &mut program_context,
        &payer,
        &token_mint.pubkey(),
        &source.pubkey(),
    )
    .await?;
    process_token_mint_to(
        &mut program_context,
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
        &ephemeral_rollups_wrapper::id(),
    );
    let authority2_token_escrow_slot = 42;
    let authority2_token_escrow_pda = TokenEscrow::generate_pda(
        &authority2.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority2_token_escrow_slot,
        &ephemeral_rollups_wrapper::id(),
    );

    // Prepare being able to escrow token for this validator
    process_token_vault_init(
        &mut program_context,
        &payer,
        &validator,
        &token_mint.pubkey(),
    )
    .await?;

    // Create an escrow
    process_token_escrow_create(
        &mut program_context,
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
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority1_token_escrow_pda)
            .await?
            .amount
    );

    // Fund the escrow
    process_token_escrow_deposit(
        &mut program_context,
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
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority1_token_escrow_pda)
            .await?
            .amount
    );

    // Fund the escrow some more
    process_token_escrow_deposit(
        &mut program_context,
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
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority1_token_escrow_pda)
            .await?
            .amount
    );

    // Create a different escrow (but this one is unfunded)
    process_token_escrow_create(
        &mut program_context,
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
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority2_token_escrow_pda)
            .await?
            .amount
    );

    // Transfer some from 1->2
    process_token_escrow_transfer(
        &mut program_context,
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
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority1_token_escrow_pda)
            .await?
            .amount
    );
    assert_eq!(
        1_000_000,
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority2_token_escrow_pda)
            .await?
            .amount
    );

    // Transfer all remaining from 1->2
    process_token_escrow_transfer(
        &mut program_context,
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
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority1_token_escrow_pda)
            .await?
            .amount
    );
    assert_eq!(
        100_000_000,
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority2_token_escrow_pda)
            .await?
            .amount
    );

    // Transfer back most of it back 2->1
    process_token_escrow_transfer(
        &mut program_context,
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
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority1_token_escrow_pda)
            .await?
            .amount
    );
    assert_eq!(
        25_000_000,
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority2_token_escrow_pda)
            .await?
            .amount
    );

    // Withdraw everything after that
    let destination_token = process_associated_token_account_get_or_init(
        &mut program_context,
        &payer,
        &token_mint.pubkey(),
        &destination.pubkey(),
    )
    .await?;

    // There should be most of it in the first escrow
    process_token_escrow_withdraw(
        &mut program_context,
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
        &mut program_context,
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
        read_account_packed::<Account>(&mut program_context, &destination_token)
            .await?
            .amount
    );

    // Done
    Ok(())
}
