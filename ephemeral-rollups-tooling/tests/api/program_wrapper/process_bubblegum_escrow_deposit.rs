use ephemeral_rollups_wrapper::instruction::bubblegum_escrow_deposit;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn process_bubblegum_escrow_deposit(
    toolbox_endpoint: &mut ToolboxEndpoint,
    payer: &Keypair,
    authority: &Pubkey,
    validator: &Pubkey,
    tree: &Pubkey,
    leaf_owner: &Keypair,
    leaf_delegate: &Pubkey,
    root_hash: &[u8; 32],
    data_hash: &[u8; 32],
    creator_hash: &[u8; 32],
    nonce: u64,
    index: u32,
) -> Result<(), ToolboxEndpointError> {
    let instruction = bubblegum_escrow_deposit::instruction(
        &payer.pubkey(),
        authority,
        validator,
        tree,
        &leaf_owner.pubkey(),
        leaf_delegate,
        root_hash,
        data_hash,
        creator_hash,
        nonce,
        index,
    );
    toolbox_endpoint
        .process_instruction_with_signers(instruction, payer, &[leaf_owner])
        .await?;
    Ok(())
}
