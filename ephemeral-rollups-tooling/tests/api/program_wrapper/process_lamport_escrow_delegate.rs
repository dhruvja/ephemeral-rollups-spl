use ephemeral_rollups_wrapper::instruction::lamport_escrow_delegate;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn process_lamport_escrow_delegate(
    toolbox_endpoint: &mut ToolboxEndpoint,
    payer: &Keypair,
    authority: &Keypair,
    validator: &Pubkey,
    slot: u64,
) -> Result<(), ToolboxEndpointError> {
    let instruction = lamport_escrow_delegate::instruction(
        &payer.pubkey(),
        &authority.pubkey(),
        validator,
        slot,
    );
    toolbox_endpoint
        .process_instruction_with_signers(instruction, payer, &[authority])
        .await?;
    Ok(())
}
