use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

use ephemeral_rollups_wrapper::instruction::lamport_escrow_create;

pub async fn process_lamport_escrow_create(
    endpoint: &mut Endpoint,
    payer: &Keypair,
    authority: &Pubkey,
    validator: &Pubkey,
    slot: u64,
) -> Result<(), EndpointError> {
    let instruction =
        lamport_escrow_create::instruction(&payer.pubkey(), authority, validator, slot);
    endpoint.process_instruction(instruction, payer).await?;
    Ok(())
}
