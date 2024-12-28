use ephemeral_rollups_wrapper::state::bubblegum_escrow::BubblegumEscrow;
use mpl_bubblegum::hash::hash_creators;
use mpl_bubblegum::hash::hash_metadata;
use mpl_bubblegum::types::Creator;
use mpl_bubblegum::types::LeafSchema;
use mpl_bubblegum::types::MetadataArgs;
use mpl_bubblegum::types::TokenProgramVersion;
use mpl_bubblegum::types::TokenStandard;
use mpl_bubblegum::utils::get_asset_id;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;
use spl_merkle_tree_reference::MerkleTree;
use spl_merkle_tree_reference::Node;

use crate::api::program_bubblegum::process_create_tree::process_create_tree;
use crate::api::program_bubblegum::process_mint::process_mint;
use crate::api::program_delegation::process_delegate_on_curve::process_delegate_on_curve;
use crate::api::program_delegation::wait_until_undelegation::wait_until_undelegation;
use crate::api::program_wrapper::process_bubblegum_escrow_delegate::process_bubblegum_escrow_delegate;
use crate::api::program_wrapper::process_bubblegum_escrow_deposit::process_bubblegum_escrow_deposit;
use crate::api::program_wrapper::process_bubblegum_escrow_transfer::process_bubblegum_escrow_transfer;
use crate::api::program_wrapper::process_bubblegum_escrow_undelegate::process_bubblegum_escrow_undelegate;
use crate::api::program_wrapper::process_bubblegum_escrow_withdraw::process_bubblegum_escrow_withdraw;

#[tokio::test]
async fn devnet_bubblegum_escrow_deposit_delegate_undelegate(
) -> Result<(), ToolboxEndpointError> {
    let mut toolbox_endpoint_chain =
        ToolboxEndpoint::new_rpc_with_url_and_commitment(
            "https://api.devnet.solana.com".to_string(),
            CommitmentConfig::confirmed(),
        );
    let mut toolbox_endpoint_ephem =
        ToolboxEndpoint::new_rpc_with_url_and_commitment(
            "https://devnet.magicblock.app".to_string(),
            CommitmentConfig::confirmed(),
        );

    // Devnet dummy payer: Payi9ovX2Tbe69XuUdgav5qS3sVnNAn2dN8BZoAQwyq
    let payer_chain = Keypair::from_bytes(&[
        243, 85, 166, 238, 237, 2, 46, 208, 68, 40, 98, 2, 148, 117, 134, 238,
        144, 223, 165, 108, 203, 120, 96, 89, 172, 223, 98, 26, 162, 92, 234,
        167, 5, 201, 50, 82, 10, 153, 196, 60, 132, 31, 123, 66, 63, 113, 122,
        83, 145, 102, 200, 15, 46, 50, 207, 1, 6, 109, 0, 216, 225, 247, 70,
        96,
    ])
    .unwrap();

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let bubblegum_minter = Keypair::new();
    let bubblegum_tree = Keypair::new();

    let authority1 = Keypair::new();
    let authority2 = Keypair::new();

    let chain_input = Keypair::new();
    let chain_output = Pubkey::new_unique();

    // Create the bubblegum tree
    process_create_tree(
        &mut toolbox_endpoint_chain,
        &payer_chain,
        &bubblegum_minter,
        &bubblegum_tree,
        false,
    )
    .await?;

    // We create a local tree, so that we can keep track of the hashes involved
    // without reading the ledger
    let mut bubblegum_proof =
        MerkleTree::new(vec![Node::default(); 1 << 6].as_slice());

    // Define an NFT
    let bubblegum_nft_metadata = MetadataArgs {
        name: String::from("Bubblegum NFT"),
        uri: String::from("https://bubblegum.nft"),
        symbol: String::from("bNFT"),
        creators: vec![Creator {
            address: bubblegum_minter.pubkey(),
            share: 100,
            verified: false,
        }],
        edition_nonce: None,
        is_mutable: true,
        primary_sale_happened: true,
        seller_fee_basis_points: 500,
        token_program_version: TokenProgramVersion::Original,
        token_standard: Some(TokenStandard::NonFungible),
        collection: None,
        uses: None,
    };

    // Mint the new nft to the "chain_input"
    process_mint(
        &mut toolbox_endpoint_chain,
        &payer_chain,
        &bubblegum_minter,
        &bubblegum_tree.pubkey(),
        &chain_input.pubkey(),
        &bubblegum_nft_metadata,
    )
    .await?;

    // After minting, we expect to know the information about the asset
    let bubblegum_nft_nonce = 0;
    let bubblegum_nft_index = 0;
    let bubblegum_nft_data_hash =
        hash_metadata(&bubblegum_nft_metadata).unwrap();
    let bubblegum_nft_creator_hash =
        hash_creators(&bubblegum_nft_metadata.creators);
    let bubblegum_nft_asset_id =
        get_asset_id(&bubblegum_tree.pubkey(), bubblegum_nft_nonce);

    // After mint, we update the local proof for later use
    bubblegum_proof.add_leaf(
        LeafSchema::V1 {
            id: bubblegum_nft_asset_id,
            owner: chain_input.pubkey(),
            delegate: chain_input.pubkey(),
            nonce: bubblegum_nft_nonce,
            data_hash: bubblegum_nft_data_hash,
            creator_hash: bubblegum_nft_creator_hash,
        }
        .hash(),
        bubblegum_nft_nonce as usize,
    );

    // Create a new bubblegum escrow (owned by authority1)
    process_bubblegum_escrow_deposit(
        &mut toolbox_endpoint_chain,
        &payer_chain,
        &authority1.pubkey(),
        &validator,
        &bubblegum_tree.pubkey(),
        &chain_input,
        &chain_input.pubkey(),
        &bubblegum_proof.get_root(),
        &bubblegum_nft_data_hash,
        &bubblegum_nft_creator_hash,
        bubblegum_nft_nonce,
        bubblegum_nft_index,
    )
    .await?;

    // Escrow will be the new owner
    let bubblegum_escrow_pda = BubblegumEscrow::generate_pda(
        &validator,
        &bubblegum_nft_asset_id,
        &ephemeral_rollups_wrapper::ID,
    );

    // The authority1 must now be the escrow authority
    assert_eq!(
        authority1.pubkey(),
        toolbox_endpoint_chain
            .get_account_data_borsh_deserialized::<BubblegumEscrow>(
                &bubblegum_escrow_pda
            )
            .await?
            .unwrap()
            .authority
    );

    // After escrow, we update the local proof for later use
    bubblegum_proof.add_leaf(
        LeafSchema::V1 {
            id: bubblegum_nft_asset_id,
            owner: bubblegum_escrow_pda,
            delegate: bubblegum_escrow_pda,
            nonce: bubblegum_nft_nonce,
            data_hash: bubblegum_nft_data_hash,
            creator_hash: bubblegum_nft_creator_hash,
        }
        .hash(),
        bubblegum_nft_nonce as usize,
    );

    // Delegate the escrow
    process_bubblegum_escrow_delegate(
        &mut toolbox_endpoint_chain,
        &payer_chain,
        &authority1,
        &validator,
        &bubblegum_tree.pubkey(),
        bubblegum_nft_nonce,
    )
    .await?;

    // Ephemeral dummy payer, delegate it to be used in the ER
    let payer_ephem = Keypair::new();
    process_delegate_on_curve(
        &mut toolbox_endpoint_chain,
        &payer_chain,
        &payer_ephem,
        1_000_000,
    )
    .await?;

    // Transfer the ownership to authority2 from inside the ER
    process_bubblegum_escrow_transfer(
        &mut toolbox_endpoint_ephem,
        &payer_ephem,
        &authority1,
        &authority2.pubkey(),
        &validator,
        &bubblegum_tree.pubkey(),
        bubblegum_nft_nonce,
    )
    .await?;

    // The authority2 must now be the escrow authority
    assert_eq!(
        authority2.pubkey(),
        toolbox_endpoint_ephem
            .get_account_data_borsh_deserialized::<BubblegumEscrow>(
                &bubblegum_escrow_pda
            )
            .await?
            .unwrap()
            .authority
    );

    // Undelegate back to chain
    process_bubblegum_escrow_undelegate(
        &mut toolbox_endpoint_ephem,
        &payer_ephem,
        &authority2,
        &validator,
        &bubblegum_tree.pubkey(),
        bubblegum_nft_nonce,
    )
    .await?;

    // Wait for undelegation to succeed
    wait_until_undelegation(&mut toolbox_endpoint_chain, &bubblegum_escrow_pda)
        .await?;

    // Withdraw the cNFT from the escrow back to "chain_output"
    process_bubblegum_escrow_withdraw(
        &mut toolbox_endpoint_chain,
        &payer_chain,
        &authority2,
        &chain_output,
        &validator,
        &chain_output,
        &bubblegum_tree.pubkey(),
        &bubblegum_proof.get_root(),
        &bubblegum_nft_data_hash,
        &bubblegum_nft_creator_hash,
        bubblegum_nft_nonce,
        bubblegum_nft_index,
    )
    .await?;

    // The escrow must have been destroyed
    assert!(
        !toolbox_endpoint_chain
            .get_account_exists(&bubblegum_escrow_pda)
            .await?
    );

    // Done
    Ok(())
}
