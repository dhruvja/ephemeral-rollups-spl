use mpl_bubblegum::accounts::TreeConfig;
use mpl_bubblegum::instructions::MintV1Builder;
use mpl_bubblegum::types::MetadataArgs;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn process_mint(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    minter: &Keypair,
    tree: &Pubkey,
    owner: &Pubkey,
    metadata: &MetadataArgs,
) -> Result<(), ProgramError> {
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
    process_instruction_with_signer(program_context, mint_instruction, payer, minter).await
}
