use ephemeral_rollups_wrapper::instruction::token_vault_init;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn process_token_vault_init(
    toolbox_endpoint: &mut ToolboxEndpoint,
    payer: &Keypair,
    validator: &Pubkey,
    token_mint: &Pubkey,
) -> Result<(), ToolboxEndpointError> {
    let instruction =
        token_vault_init::instruction(&payer.pubkey(), validator, token_mint);
    toolbox_endpoint.process_instruction(instruction, payer).await?;
    Ok(())
}
