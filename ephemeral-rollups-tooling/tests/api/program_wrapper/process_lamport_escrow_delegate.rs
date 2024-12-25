use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

use ephemeral_rollups_wrapper::instruction::lamport_escrow_delegate;

pub async fn process_lamport_escrow_delegate(
    endpoint: &mut Endpoint,
    payer: &Keypair,
    authority: &Keypair,
    validator: &Pubkey,
    slot: u64,
) -> Result<(), EndpointError> {
    let instruction =
        lamport_escrow_delegate::instruction(&payer.pubkey(), &authority.pubkey(), validator, slot);
    endpoint
        .process_instruction_with_signers(instruction, payer, &[authority])
        .await?;
    Ok(())
}
