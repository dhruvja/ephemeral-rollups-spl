use ephemeral_rollups_wrapper::state::bubblegum_escrow::BubblegumEscrow;
use mpl_bubblegum::hash::hash_creators;
use mpl_bubblegum::hash::hash_metadata;
use mpl_bubblegum::types::Creator;
use mpl_bubblegum::types::LeafSchema;
use mpl_bubblegum::types::MetadataArgs;
use mpl_bubblegum::types::TokenProgramVersion;
use mpl_bubblegum::types::TokenStandard;
use mpl_bubblegum::utils::get_asset_id;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpointError;
use spl_merkle_tree_reference::MerkleTree;
use spl_merkle_tree_reference::Node;

use crate::api::create_localnet_toolbox_endpoint::create_localnet_toolbox_endpoint;
use crate::api::program_bubblegum::process_create_tree::process_create_tree;
use crate::api::program_bubblegum::process_mint::process_mint;
use crate::api::program_wrapper::process_bubblegum_escrow_deposit::process_bubblegum_escrow_deposit;
use crate::api::program_wrapper::process_bubblegum_escrow_transfer::process_bubblegum_escrow_transfer;
use crate::api::program_wrapper::process_bubblegum_escrow_withdraw::process_bubblegum_escrow_withdraw;

#[tokio::test]
async fn localnet_bubblegum_escrow_deposit_transfer_withdraw(
) -> Result<(), ToolboxEndpointError> {
    let mut toolbox_endpoint = create_localnet_toolbox_endpoint().await;

    // Important keys used in the test
    let validator = Pubkey::new_unique();

    let payer = Keypair::new();

    let bubblegum_minter = Keypair::new();
    let bubblegum_tree = Keypair::new();

    let authority1 = Keypair::new();
    let authority2 = Keypair::new();

    let source = Keypair::new();
    let destination = Keypair::new();

    // Fund payer
    toolbox_endpoint
        .process_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await?;

    // Create the bubblegum tree
    process_create_tree(
        &mut toolbox_endpoint,
        &payer,
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

    // Mint the new nft to the "source"
    process_mint(
        &mut toolbox_endpoint,
        &payer,
        &bubblegum_minter,
        &bubblegum_tree.pubkey(),
        &source.pubkey(),
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
            owner: source.pubkey(),
            delegate: source.pubkey(),
            nonce: bubblegum_nft_nonce,
            data_hash: bubblegum_nft_data_hash,
            creator_hash: bubblegum_nft_creator_hash,
        }
        .hash(),
        bubblegum_nft_nonce as usize,
    );

    // Create a new bubblegum escrow (owned by authority1)
    process_bubblegum_escrow_deposit(
        &mut toolbox_endpoint,
        &payer,
        &authority1.pubkey(),
        &validator,
        &bubblegum_tree.pubkey(),
        &source,
        &source.pubkey(),
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
        toolbox_endpoint
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

    // Transfer the ownership to authority2
    process_bubblegum_escrow_transfer(
        &mut toolbox_endpoint,
        &payer,
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
        toolbox_endpoint
            .get_account_data_borsh_deserialized::<BubblegumEscrow>(
                &bubblegum_escrow_pda
            )
            .await?
            .unwrap()
            .authority
    );

    // Withdraw the cNFT from the escrow back to "destination"
    process_bubblegum_escrow_withdraw(
        &mut toolbox_endpoint,
        &payer,
        &authority2,
        &destination.pubkey(),
        &validator,
        &destination.pubkey(),
        &bubblegum_tree.pubkey(),
        &bubblegum_proof.get_root(),
        &bubblegum_nft_data_hash,
        &bubblegum_nft_creator_hash,
        bubblegum_nft_nonce,
        bubblegum_nft_index,
    )
    .await?;

    // The escrow must have been destroyed
    assert!(!toolbox_endpoint.get_account_exists(&bubblegum_escrow_pda).await?);

    // Done
    Ok(())
}
