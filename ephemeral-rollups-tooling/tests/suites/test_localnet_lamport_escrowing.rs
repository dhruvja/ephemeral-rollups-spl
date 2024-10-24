use ephemeral_rollups_bridge::state::lamport_escrow::LamportEscrow;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::api::program_bridge::process_lamport_escrow_claim::process_lamport_escrow_claim;
use crate::api::program_bridge::process_lamport_escrow_create::process_lamport_escrow_create;
use crate::api::program_context::create_program_test_context::create_program_test_context;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;
use crate::api::program_context::read_account::read_account_lamports;
use crate::api::program_spl::process_system_transfer::process_system_transfer;

#[tokio::test]
async fn test_localnet_lamport_escrowing() -> Result<(), ProgramError> {
    let mut program_context: Box<dyn ProgramContext> =
        Box::new(create_program_test_context().await);

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();
    let authority = Keypair::new();
    let destination = Keypair::new();

    // Lamport escrow account we will be using
    let lamport_escrow_index = 42;
    let lamport_escrow_pda = LamportEscrow::generate_pda(
        &authority.pubkey(),
        &validator,
        lamport_escrow_index,
        &ephemeral_rollups_bridge::id(),
    );
    let lamport_escrow_rent = program_context
        .get_rent_minimum_balance(LamportEscrow::space())
        .await?;

    // Fund payer
    program_context
        .process_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await?;

    // Before anything happened
    assert_eq!(
        0,
        read_account_lamports(&mut program_context, &lamport_escrow_pda).await?
    );
    assert_eq!(
        0,
        read_account_lamports(&mut program_context, &authority.pubkey()).await?
    );
    assert_eq!(
        0,
        read_account_lamports(&mut program_context, &destination.pubkey()).await?
    );

    // Create a new lamport escrow
    process_lamport_escrow_create(
        &mut program_context,
        &payer,
        &authority.pubkey(),
        &validator,
        lamport_escrow_index,
    )
    .await?;

    // After the escrow is created
    assert_eq!(
        lamport_escrow_rent,
        read_account_lamports(&mut program_context, &lamport_escrow_pda).await?
    );
    assert_eq!(
        0,
        read_account_lamports(&mut program_context, &authority.pubkey()).await?
    );
    assert_eq!(
        0,
        read_account_lamports(&mut program_context, &destination.pubkey()).await?
    );

    // Send some lamports to the escrow from somewhere
    process_system_transfer(
        &mut program_context,
        &payer,
        &payer,
        &lamport_escrow_pda,
        10 * LAMPORTS_PER_SOL,
    )
    .await?;

    // After the escrow is funded
    assert_eq!(
        lamport_escrow_rent + 10 * LAMPORTS_PER_SOL,
        read_account_lamports(&mut program_context, &lamport_escrow_pda).await?
    );
    assert_eq!(
        0,
        read_account_lamports(&mut program_context, &authority.pubkey()).await?
    );
    assert_eq!(
        0,
        read_account_lamports(&mut program_context, &destination.pubkey()).await?
    );

    // Claim some funds from the escrow
    process_lamport_escrow_claim(
        &mut program_context,
        &payer,
        &authority,
        &destination.pubkey(),
        &validator,
        lamport_escrow_index,
        1 * LAMPORTS_PER_SOL,
    )
    .await?;

    // After the escrow has had some funds claimed
    assert_eq!(
        lamport_escrow_rent + 9 * LAMPORTS_PER_SOL,
        read_account_lamports(&mut program_context, &lamport_escrow_pda).await?
    );
    assert_eq!(
        0,
        read_account_lamports(&mut program_context, &authority.pubkey()).await?
    );
    assert_eq!(
        1 * LAMPORTS_PER_SOL,
        read_account_lamports(&mut program_context, &destination.pubkey()).await?
    );

    // Claim the rest of the funds
    process_lamport_escrow_claim(
        &mut program_context,
        &payer,
        &authority,
        &destination.pubkey(),
        &validator,
        lamport_escrow_index,
        9 * LAMPORTS_PER_SOL,
    )
    .await?;

    // After the escrow has had all funds claimed
    assert_eq!(
        lamport_escrow_rent,
        read_account_lamports(&mut program_context, &lamport_escrow_pda).await?
    );
    assert_eq!(
        0,
        read_account_lamports(&mut program_context, &authority.pubkey()).await?
    );
    assert_eq!(
        10 * LAMPORTS_PER_SOL,
        read_account_lamports(&mut program_context, &destination.pubkey()).await?
    );

    // Claiming zero should be an acceptable no-op
    process_lamport_escrow_claim(
        &mut program_context,
        &payer,
        &authority,
        &destination.pubkey(),
        &validator,
        lamport_escrow_index,
        0,
    )
    .await?;

    // After the escrow has had all funds claimed
    assert_eq!(
        lamport_escrow_rent,
        read_account_lamports(&mut program_context, &lamport_escrow_pda).await?
    );
    assert_eq!(
        0,
        read_account_lamports(&mut program_context, &authority.pubkey()).await?
    );
    assert_eq!(
        10 * LAMPORTS_PER_SOL,
        read_account_lamports(&mut program_context, &destination.pubkey()).await?
    );

    // Make sure we can't withdraw anything past the rent exemption
    assert!(process_lamport_escrow_claim(
        &mut program_context,
        &payer,
        &authority,
        &destination.pubkey(),
        &validator,
        lamport_escrow_index,
        1,
    )
    .await
    .is_err());

    // Done
    Ok(())
}
