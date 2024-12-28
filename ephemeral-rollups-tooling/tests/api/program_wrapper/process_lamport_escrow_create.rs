use ephemeral_rollups_wrapper::instruction::lamport_escrow_create;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn process_lamport_escrow_create(
    toolbox_endpoint: &mut ToolboxEndpoint,
    payer: &Keypair,
    authority: &Pubkey,
    validator: &Pubkey,
    slot: u64,
) -> Result<(), ToolboxEndpointError> {
    let instruction = lamport_escrow_create::instruction(
        &payer.pubkey(),
        authority,
        validator,
        slot,
    );
    toolbox_endpoint.process_instruction(instruction, payer).await?;
    Ok(())
}
