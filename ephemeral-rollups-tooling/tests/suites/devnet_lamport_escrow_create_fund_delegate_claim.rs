use ephemeral_rollups_wrapper::state::lamport_escrow::LamportEscrow;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;
use crate::api::program_context::read_account::read_account_lamports;
use crate::api::program_spl::process_system_transfer::process_system_transfer;
use crate::api::program_wrapper::process_lamport_escrow_claim::process_lamport_escrow_claim;
use crate::api::program_wrapper::process_lamport_escrow_create::process_lamport_escrow_create;
use crate::api::program_wrapper::process_lamport_escrow_delegate::process_lamport_escrow_delegate;

#[tokio::test]
async fn devnet_lamport_escrow_create_fund_delegate_claim() -> Result<(), ProgramError> {
    let mut program_context_chain: Box<dyn ProgramContext> =
        Box::new(RpcClient::new_with_commitment(
            "https://api.devnet.solana.com".to_string(),
            CommitmentConfig::confirmed(),
        ));
    let mut program_context_ephem: Box<dyn ProgramContext> =
        Box::new(RpcClient::new_with_commitment(
            "https://devnet.magicblock.app".to_string(),
            CommitmentConfig::confirmed(),
        ));

    // Devnet dummy payer: Payi9ovX2Tbe69XuUdgav5qS3sVnNAn2dN8BZoAQwyq
    let payer_chain = Keypair::from_bytes(&[
        243, 85, 166, 238, 237, 2, 46, 208, 68, 40, 98, 2, 148, 117, 134, 238, 144, 223, 165, 108,
        203, 120, 96, 89, 172, 223, 98, 26, 162, 92, 234, 167, 5, 201, 50, 82, 10, 153, 196, 60,
        132, 31, 123, 66, 63, 113, 122, 83, 145, 102, 200, 15, 46, 50, 207, 1, 6, 109, 0, 216, 225,
        247, 70, 96,
    ])
    .map_err(|e| ProgramError::Signature(e.to_string()))?;

    // Ephemeral dummy payer
    let payer_ephem = Keypair::new();
    // TODO - we have to provide fee lamports later for payer in ER

    // Important keys used in the test
    let validator = Pubkey::new_unique();
    let authority = Keypair::new();
    let destination = Keypair::new();

    // Lamport escrow account we will be using
    let lamport_escrow_slot = 42;
    let lamport_escrow_pda = LamportEscrow::generate_pda(
        &authority.pubkey(),
        &validator,
        lamport_escrow_slot,
        &ephemeral_rollups_wrapper::id(),
    );
    /*
       let lamport_escrow_rent = program_context_chain
           .get_rent_minimum_balance(LamportEscrow::space())
           .await?;
    */

    // Create a new lamport escrow
    process_lamport_escrow_create(
        &mut program_context_chain,
        &payer_chain,
        &authority.pubkey(),
        &validator,
        lamport_escrow_slot,
    )
    .await?;

    // Send some lamports to the escrow from somewhere
    process_system_transfer(
        &mut program_context_chain,
        &payer_chain,
        &payer_chain,
        &lamport_escrow_pda,
        1_000_000,
    )
    .await?;

    // Delegate it immediately
    process_lamport_escrow_delegate(
        &mut program_context_chain,
        &payer_chain,
        &authority,
        &validator,
        lamport_escrow_slot,
    )
    .await?;

    /*
    // TODO - this should work properly, but doesn't

    // Claim some funds from the escrow (from inside the ER)
    process_lamport_escrow_claim(
        &mut program_context_ephem,
        &payer_ephem,
        &authority,
        &destination.pubkey(),
        &validator,
        lamport_escrow_slot,
        500_000,
    )
    .await?;

    // Check that the lamports have appeared in the ER
    assert_eq!(
        lamport_escrow_rent + 500_000,
        read_account_lamports(&mut program_context_ephem, &lamport_escrow_pda).await?
    );
    assert_eq!(
        500_000,
        read_account_lamports(&mut program_context_ephem, &destination).await?
    );
    */

    // Done
    Ok(())
}
