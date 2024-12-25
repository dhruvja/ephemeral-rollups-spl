use ephemeral_rollups_wrapper::state::lamport_escrow::LamportEscrow;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

use crate::api::program_delegation::process_delegate_on_curve::process_delegate_on_curve;
use crate::api::program_delegation::wait_until_undelegation::wait_until_undelegation;
use crate::api::program_wrapper::process_lamport_escrow_claim::process_lamport_escrow_claim;
use crate::api::program_wrapper::process_lamport_escrow_create::process_lamport_escrow_create;
use crate::api::program_wrapper::process_lamport_escrow_delegate::process_lamport_escrow_delegate;
use crate::api::program_wrapper::process_lamport_escrow_undelegate::process_lamport_escrow_undelegate;

#[tokio::test]
async fn devnet_lamport_escrow_create_fund_delegate_undelegate() -> Result<(), EndpointError> {
    let mut endpoint_chain = Endpoint::from(RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    ));
    let mut endpoint_ephem = Endpoint::from(RpcClient::new_with_commitment(
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
    .map_err(|e| EndpointError::Signature(e.to_string()))?;

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let authority1 = Keypair::new();
    let authority2 = Keypair::new();

    let chain_output = Pubkey::new_unique();

    // Lamport escrow account we will be using
    let authority1_lamport_escrow_slot = 41;
    let authority1_lamport_escrow_pda = LamportEscrow::generate_pda(
        &authority1.pubkey(),
        &validator,
        authority1_lamport_escrow_slot,
        &ephemeral_rollups_wrapper::ID,
    );
    let authority2_lamport_escrow_slot = 77;
    let authority2_lamport_escrow_pda = LamportEscrow::generate_pda(
        &authority2.pubkey(),
        &validator,
        authority2_lamport_escrow_slot,
        &ephemeral_rollups_wrapper::ID,
    );

    let lamport_escrow_rent = endpoint_chain
        .get_rent_minimum_balance(LamportEscrow::space())
        .await?;

    // Create a new lamport escrow
    process_lamport_escrow_create(
        &mut endpoint_chain,
        &payer_chain,
        &authority1.pubkey(),
        &validator,
        authority1_lamport_escrow_slot,
    )
    .await?;

    // Send some lamports to the escrow from somewhere
    endpoint_chain
        .process_system_transfer(
            &payer_chain,
            &payer_chain,
            &authority1_lamport_escrow_pda,
            1_000_000,
        )
        .await?;

    // Delegate it immediately
    process_lamport_escrow_delegate(
        &mut endpoint_chain,
        &payer_chain,
        &authority1,
        &validator,
        authority1_lamport_escrow_slot,
    )
    .await?;

    // Create another escrow
    process_lamport_escrow_create(
        &mut endpoint_chain,
        &payer_chain,
        &authority2.pubkey(),
        &validator,
        authority2_lamport_escrow_slot,
    )
    .await?;

    // Delegate it too
    process_lamport_escrow_delegate(
        &mut endpoint_chain,
        &payer_chain,
        &authority2,
        &validator,
        authority2_lamport_escrow_slot,
    )
    .await?;

    // Ephemeral dummy payer, delegate it to be used in the ER
    let payer_ephem = Keypair::new();
    process_delegate_on_curve(&mut endpoint_chain, &payer_chain, &payer_ephem, 1_000_000).await?;

    // TODO - this should work properly, but doesn't, yet
    // Claim some funds from the escrow toward the other one (from inside the ER)
    process_lamport_escrow_claim(
        &mut endpoint_ephem,
        &payer_ephem,
        &authority1,
        &authority2_lamport_escrow_pda, // other escrow is the receiver
        &validator,
        authority1_lamport_escrow_slot,
        400_000,
    )
    .await?;

    // Check that the lamports have appeared in the ER
    assert_eq!(
        lamport_escrow_rent + 600_000,
        endpoint_ephem
            .get_account_lamports(&authority1_lamport_escrow_pda)
            .await?
    );
    assert_eq!(
        400_000,
        endpoint_ephem
            .get_account_lamports(&authority2_lamport_escrow_pda)
            .await?
    );

    // Move some funds back to where it came from
    process_lamport_escrow_claim(
        &mut endpoint_ephem,
        &payer_ephem,
        &authority2,
        &authority1_lamport_escrow_pda, // other escrow is the receiver
        &validator,
        authority2_lamport_escrow_slot,
        100_000,
    )
    .await?;

    // Check that the lamports have moved in the ER
    assert_eq!(
        lamport_escrow_rent + 700_000,
        endpoint_ephem
            .get_account_lamports(&authority1_lamport_escrow_pda)
            .await?
    );
    assert_eq!(
        300_000,
        endpoint_ephem
            .get_account_lamports(&authority2_lamport_escrow_pda)
            .await?
    );

    // Undelegate everything
    process_lamport_escrow_undelegate(
        &mut endpoint_ephem,
        &payer_ephem,
        &authority1,
        &validator,
        authority1_lamport_escrow_slot,
    )
    .await?;
    process_lamport_escrow_undelegate(
        &mut endpoint_ephem,
        &payer_ephem,
        &authority2,
        &validator,
        authority2_lamport_escrow_slot,
    )
    .await?;

    // Wait for undelegation to succeed
    wait_until_undelegation(&mut endpoint_chain, &authority1_lamport_escrow_pda).await?;
    wait_until_undelegation(&mut endpoint_chain, &authority2_lamport_escrow_pda).await?;

    // For fun, we should be able to claim lamports back on chain now
    process_lamport_escrow_claim(
        &mut endpoint_chain,
        &payer_chain,
        &authority1,
        &chain_output,
        &validator,
        authority1_lamport_escrow_slot,
        700_000,
    )
    .await?;
    process_lamport_escrow_claim(
        &mut endpoint_chain,
        &payer_chain,
        &authority2,
        &chain_output,
        &validator,
        authority2_lamport_escrow_slot,
        300_000,
    )
    .await?;

    // Done
    Ok(())
}
