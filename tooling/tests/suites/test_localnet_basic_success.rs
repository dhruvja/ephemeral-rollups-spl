use ephemeral_rollups_bridge::state::lamport_escrow::LamportEscrow;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::api::program_bridge::process_lamport_escrow_claim::process_lamport_escrow_claim;
use crate::api::program_bridge::process_lamport_escrow_create::process_lamport_escrow_create;
use crate::api::program_context::create_program_test_context::create_program_test_context;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;
use crate::api::program_context::read_account;
use crate::api::program_spl::process_system_transfer::process_system_transfer;
use crate::suites::test_localnet_basic_success::read_account::read_account;

#[tokio::test]
async fn test_localnet_basic_success() -> Result<(), ProgramError> {
    let mut program_context: Box<dyn ProgramContext> =
        Box::new(create_program_test_context().await);

    // Important keys
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();
    let authority = Keypair::new();

    let lamport_escrow_pda = LamportEscrow::generate_pda(
        &authority.pubkey(),
        &validator,
        42,
        &ephemeral_rollups_bridge::id(),
    );

    // Fund payer
    program_context
        .process_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await?;

    eprintln!(
        "Stage1: {:?}",
        read_account(&mut program_context, &lamport_escrow_pda).await?
    );

    // Create an escrow for the
    process_lamport_escrow_create(
        &mut program_context,
        &payer,
        &authority.pubkey(),
        &validator,
        42,
    )
    .await?;

    eprintln!(
        "Stage2: {:?}",
        read_account(&mut program_context, &lamport_escrow_pda).await?
    );

    process_system_transfer(
        &mut program_context,
        &payer,
        &payer,
        &lamport_escrow_pda,
        999,
    )
    .await?;

    eprintln!(
        "Stage3: {:?}",
        read_account(&mut program_context, &lamport_escrow_pda).await?
    );

    process_lamport_escrow_claim(
        &mut program_context,
        &payer,
        &authority,
        &validator,
        42,
        100,
    )
    .await?;

    /*
    // Mints decimals
    let usdc_mint_decimals = 6;

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

    // Important amounts
    let liquid_usdc_amount = ui_amount_to_amount(50_000_000., usdc_mint_decimals);

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
    */

    // Done
    Ok(())
}
