use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

use ephemeral_rollups_wrapper::instruction::token_vault_init;

pub async fn process_token_vault_init(
    endpoint: &mut Endpoint,
    payer: &Keypair,
    validator: &Pubkey,
    token_mint: &Pubkey,
) -> Result<(), EndpointError> {
    let instruction = token_vault_init::instruction(&payer.pubkey(), validator, token_mint);
    endpoint.process_instruction(instruction, payer).await?;
    Ok(())
}
