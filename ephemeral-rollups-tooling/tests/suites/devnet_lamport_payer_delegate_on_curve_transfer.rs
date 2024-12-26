use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

use crate::api::program_delegation::process_delegate_on_curve::process_delegate_on_curve;

#[tokio::test]
async fn devnet_lamport_payer_delegate_on_curve_transfer() -> Result<(), EndpointError> {
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

    // Ephemeral dummy payer, delegate it to be used in the ER
    let payer_ephem1 = Keypair::new();
    process_delegate_on_curve(&mut endpoint_chain, &payer_chain, &payer_ephem1, 1_000_000).await?;

    // Ephemeral dummy payer, delegate it to be used in the ER
    let payer_ephem2 = Keypair::new();
    process_delegate_on_curve(&mut endpoint_chain, &payer_chain, &payer_ephem2, 3_000_000).await?;

    // Transfer lamports between the payers in the ER
    endpoint_ephem
        .process_system_transfer(
            &payer_ephem2,
            &payer_ephem2,
            &payer_ephem1.pubkey(),
            500_000,
        )
        .await?;

    // Account base rent needs to be taken into account
    let rent_minimum_balance = endpoint_chain.get_rent_minimum_balance(0).await?;

    // Check the balances
    assert_eq!(
        rent_minimum_balance + 1_500_000,
        endpoint_ephem
            .get_account_lamports(&payer_ephem1.pubkey())
            .await?
    );
    assert_eq!(
        rent_minimum_balance + 2_500_000,
        endpoint_ephem
            .get_account_lamports(&payer_ephem2.pubkey())
            .await?
    );

    // Done
    Ok(())
}
