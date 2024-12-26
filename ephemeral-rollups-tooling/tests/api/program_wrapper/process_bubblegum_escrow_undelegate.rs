use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

use ephemeral_rollups_wrapper::instruction::bubblegum_escrow_undelegate;

pub async fn process_bubblegum_escrow_undelegate(
    endpoint: &mut Endpoint,
    payer: &Keypair,
    authority: &Keypair,
    validator: &Pubkey,
    tree: &Pubkey,
    nonce: u64,
) -> Result<(), EndpointError> {
    let instruction = bubblegum_escrow_undelegate::instruction(
        &payer.pubkey(),
        &authority.pubkey(),
        validator,
        tree,
        nonce,
    );
    endpoint
        .process_instruction_with_signers(instruction, payer, &[authority])
        .await?;
    Ok(())
}
