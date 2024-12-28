use ephemeral_rollups_wrapper::instruction::bubblegum_escrow_undelegate;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn process_bubblegum_escrow_undelegate(
    toolbox_endpoint: &mut ToolboxEndpoint,
    payer: &Keypair,
    authority: &Keypair,
    validator: &Pubkey,
    tree: &Pubkey,
    nonce: u64,
) -> Result<(), ToolboxEndpointError> {
    let instruction = bubblegum_escrow_undelegate::instruction(
        &payer.pubkey(),
        &authority.pubkey(),
        validator,
        tree,
        nonce,
    );
    toolbox_endpoint
        .process_instruction_with_signers(instruction, payer, &[authority])
        .await?;
    Ok(())
}
