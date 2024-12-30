use ephemeral_rollups_wrapper::state::lamport_escrow::LamportEscrow;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpointError;

use crate::api::create_localnet_toolbox_endpoint::create_localnet_toolbox_endpoint;
use crate::api::program_wrapper::process_lamport_escrow_create::process_lamport_escrow_create;
use crate::api::program_wrapper::process_lamport_escrow_delegate::process_lamport_escrow_delegate;

#[tokio::test]
async fn localnet_lamport_escrow_create_fund_delegate(
) -> Result<(), ToolboxEndpointError> {
    let mut toolbox_endpoint = create_localnet_toolbox_endpoint().await;

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
    let authority_lamport_escrow_rent = toolbox_endpoint
        .get_sysvar_rent()
        .await?
        .minimum_balance(LamportEscrow::space());

    // Fund payer
    toolbox_endpoint
        .process_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await?;

    // Create a new lamport escrow
    process_lamport_escrow_create(
        &mut toolbox_endpoint,
        &payer,
        &authority.pubkey(),
        &validator,
        authority_lamport_escrow_slot,
    )
    .await?;

    // Escrow should be ready
    assert_eq!(
        authority_lamport_escrow_rent,
        toolbox_endpoint
            .get_account_lamports(&authority_lamport_escrow_pda)
            .await?
    );

    // Send some lamports to the escrow from somewhere
    toolbox_endpoint
        .process_system_transfer(
            &payer,
            &payer,
            &authority_lamport_escrow_pda,
            10 * LAMPORTS_PER_SOL,
        )
        .await?;

    // Escrow should be funded
    assert_eq!(
        authority_lamport_escrow_rent + 10 * LAMPORTS_PER_SOL,
        toolbox_endpoint
            .get_account_lamports(&authority_lamport_escrow_pda)
            .await?
    );

    // Delegate it immediately
    process_lamport_escrow_delegate(
        &mut toolbox_endpoint,
        &payer,
        &authority,
        &validator,
        authority_lamport_escrow_slot,
    )
    .await?;

    // Done
    Ok(())
}
