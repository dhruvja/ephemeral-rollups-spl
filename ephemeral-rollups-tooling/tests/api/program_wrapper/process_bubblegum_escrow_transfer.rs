use ephemeral_rollups_wrapper::instruction::bubblegum_escrow_transfer;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn process_bubblegum_escrow_transfer(
    toolbox_endpoint: &mut ToolboxEndpoint,
    payer: &Keypair,
    source_authority: &Keypair,
    destination_authority: &Pubkey,
    validator: &Pubkey,
    tree: &Pubkey,
    nonce: u64,
) -> Result<(), ToolboxEndpointError> {
    let instruction = bubblegum_escrow_transfer::instruction(
        &source_authority.pubkey(),
        destination_authority,
        validator,
        tree,
        nonce,
    );
    toolbox_endpoint
        .process_instruction_with_signers(
            instruction,
            payer,
            &[source_authority],
        )
        .await?;
    Ok(())
}
