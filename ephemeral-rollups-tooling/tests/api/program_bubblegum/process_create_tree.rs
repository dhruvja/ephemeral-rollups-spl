use mpl_bubblegum::accounts::TreeConfig;
use mpl_bubblegum::instructions::CreateTreeConfigBuilder;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use solana_toolbox_endpoint::{Endpoint, EndpointError};
use spl_account_compression::state::CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1;
use spl_account_compression::ConcurrentMerkleTree;

pub async fn process_create_tree(
    endpoint: &mut Endpoint,
    payer: &Keypair,
    minter: &Keypair,
    tree: &Keypair,
    public: bool,
) -> Result<(), EndpointError> {
    let size = CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1 + size_of::<ConcurrentMerkleTree<6, 16>>();

    let create_account_instruction = create_account(
        &payer.pubkey(),
        &tree.pubkey(),
        endpoint.get_rent_minimum_balance(size).await?,
        size as u64,
        &spl_account_compression::ID,
    );

    let tree_config_pda = TreeConfig::find_pda(&tree.pubkey()).0;
    let create_config_instruction = CreateTreeConfigBuilder::new()
        .tree_config(tree_config_pda)
        .merkle_tree(tree.pubkey())
        .public(public)
        .payer(payer.pubkey())
        .tree_creator(minter.pubkey())
        .log_wrapper(spl_noop::ID)
        .max_depth(6)
        .max_buffer_size(16)
        .instruction();

    endpoint
        .process_instructions_with_signers(
            &[create_account_instruction, create_config_instruction],
            payer,
            &[tree, minter],
        )
        .await?;

    Ok(())
}
