use mpl_bubblegum::accounts::TreeConfig;
use mpl_bubblegum::instructions::MintV1Builder;
use mpl_bubblegum::types::MetadataArgs;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

pub async fn process_mint(
    endpoint: &mut Endpoint,
    payer: &Keypair,
    minter: &Keypair,
    tree: &Pubkey,
    owner: &Pubkey,
    metadata: &MetadataArgs,
) -> Result<(), EndpointError> {
    let tree_config_pda = TreeConfig::find_pda(tree).0;
    let mint_instruction = MintV1Builder::new()
        .leaf_delegate(*owner)
        .leaf_owner(*owner)
        .merkle_tree(*tree)
        .payer(payer.pubkey())
        .tree_config(tree_config_pda)
        .tree_creator_or_delegate(minter.pubkey())
        .metadata(metadata.clone())
        .instruction();
    endpoint
        .process_instruction_with_signers(mint_instruction, payer, &[minter])
        .await?;
    Ok(())
}
