use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::api::program_bridge::process_lamport_escrow_create::process_lamport_escrow_create;
use crate::api::program_bridge::process_lamport_escrow_delegate::process_lamport_escrow_delegate;
use crate::api::program_context::create_program_test_context::create_program_test_context;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

#[tokio::test]
async fn test_localnet_lamport_escrow_create_delegate() -> Result<(), ProgramError> {
    let mut program_context: Box<dyn ProgramContext> =
        Box::new(create_program_test_context().await);

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();
    let authority = Keypair::new();

    // Lamport escrow account we will be using
    let lamport_escrow_index = 42;

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
        lamport_escrow_index,
    )
    .await?;

    // Delegate it immediately
    process_lamport_escrow_delegate(
        &mut program_context,
        &payer,
        &authority,
        &validator,
        lamport_escrow_index,
    )
    .await?;

    // Done
    Ok(())
}
