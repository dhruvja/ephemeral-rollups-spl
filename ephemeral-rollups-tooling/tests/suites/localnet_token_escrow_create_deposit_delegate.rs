use ephemeral_rollups_bridge::state::token_escrow::TokenEscrow;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::api::program_bridge::process_token_escrow_create::process_token_escrow_create;
use crate::api::program_bridge::process_token_escrow_delegate::process_token_escrow_delegate;
use crate::api::program_bridge::process_token_escrow_deposit::process_token_escrow_deposit;
use crate::api::program_bridge::process_token_vault_init::process_token_vault_init;
use crate::api::program_context::create_program_test_context::create_program_test_context;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;
use crate::api::program_context::read_account::read_account_borsh;
use crate::api::program_spl::process_associated_token_account_get_or_init::process_associated_token_account_get_or_init;
use crate::api::program_spl::process_token_mint_init::process_token_mint_init;
use crate::api::program_spl::process_token_mint_to::process_token_mint_to;

#[tokio::test]
async fn localnet_token_escrow_create_deposit_delegate() -> Result<(), ProgramError> {
    let mut program_context: Box<dyn ProgramContext> =
        Box::new(create_program_test_context().await);

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();
    let source = Keypair::new();
    let authority = Keypair::new();

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
    let source_usdc = process_associated_token_account_get_or_init(
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
        &source_usdc,
        100_000_000,
    )
    .await?;

    // Escrow account we will be creating
    let authority_token_escrow_index = 99;
    let authority_token_escrow_pda = TokenEscrow::generate_pda(
        &authority.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority_token_escrow_index,
        &ephemeral_rollups_bridge::id(),
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
        &authority.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority_token_escrow_index,
    )
    .await?;

    // No balance yet
    assert_eq!(
        0,
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority_token_escrow_pda)
            .await?
            .amount
    );

    // Fund the escrow
    process_token_escrow_deposit(
        &mut program_context,
        &payer,
        &source,
        &source_usdc,
        &authority.pubkey(),
        &validator,
        &token_mint.pubkey(),
        authority_token_escrow_index,
        10_000_000,
    )
    .await?;

    // New balance
    assert_eq!(
        10_000_000,
        read_account_borsh::<TokenEscrow>(&mut program_context, &authority_token_escrow_pda)
            .await?
            .amount
    );

    // Delegate the balance we just deposited
    process_token_escrow_delegate(
        &mut program_context,
        &payer,
        &authority,
        &validator,
        &token_mint.pubkey(),
        authority_token_escrow_index,
    )
    .await?;

    // Done
    Ok(())
}
