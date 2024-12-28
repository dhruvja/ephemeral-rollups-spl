use ephemeral_rollups_wrapper::instruction::lamport_escrow_claim;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn process_lamport_escrow_claim(
    toolbox_endpoint: &mut ToolboxEndpoint,
    payer: &Keypair,
    authority: &Keypair,
    destination: &Pubkey,
    validator: &Pubkey,
    slot: u64,
    lamports: u64,
) -> Result<(), ToolboxEndpointError> {
    let instruction = lamport_escrow_claim::instruction(
        &authority.pubkey(),
        destination,
        validator,
        slot,
        lamports,
    );
    toolbox_endpoint
        .process_instruction_with_signers(instruction, payer, &[authority])
        .await?;
    Ok(())
}
