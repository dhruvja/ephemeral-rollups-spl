use ephemeral_rollups_wrapper::state::lamport_escrow::LamportEscrow;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::api::endpoint::create_endpoint_program_test::create_program_test_context;


use crate::api::endpoint::read_account::read_account_lamports;
use crate::api::program_spl::process_system_transfer::process_system_transfer;
use crate::api::program_wrapper::process_lamport_escrow_create::process_lamport_escrow_create;
use crate::api::program_wrapper::process_lamport_escrow_delegate::process_lamport_escrow_delegate;

#[tokio::test]
async fn localnet_lamport_escrow_create_fund_delegate() -> Result<(), EndpointError> {
    let mut program_context: Box<dyn ProgramContext> =
        Box::new(create_program_test_context().await);

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();
    let authority = Keypair::new();

    // Lamport escrow account we will be using
    let authority_lamport_escrow_slot = 42;
    let authority_lamport_escrow_pda = LamportEscrow::generate_pda(
        &authority.pubkey(),
        &validator,
        authority_lamport_escrow_slot,
        &ephemeral_rollups_wrapper::ID,
    );
    let authority_lamport_escrow_rent = program_context
        .get_rent_minimum_balance(LamportEscrow::space())
        .await?;

    // Fund payer
    program_context
        .process_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await?;

    // Create a new lamport escrow
    process_lamport_escrow_create(
        &mut program_context,
        &payer,
        &authority.pubkey(),
        &validator,
        authority_lamport_escrow_slot,
    )
    .await?;

    // Escrow should be ready
    assert_eq!(
        authority_lamport_escrow_rent,
        read_account_lamports(&mut program_context, &authority_lamport_escrow_pda).await?
    );

    // Send some lamports to the escrow from somewhere
    process_system_transfer(
        &mut program_context,
        &payer,
        &payer,
        &authority_lamport_escrow_pda,
        10 * LAMPORTS_PER_SOL,
    )
    .await?;

    // Escrow should be funded
    assert_eq!(
        authority_lamport_escrow_rent + 10 * LAMPORTS_PER_SOL,
        read_account_lamports(&mut program_context, &authority_lamport_escrow_pda).await?
    );

    // Delegate it immediately
    process_lamport_escrow_delegate(
        &mut program_context,
        &payer,
        &authority,
        &validator,
        authority_lamport_escrow_slot,
    )
    .await?;

    // Done
    Ok(())
}
