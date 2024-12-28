use ephemeral_rollups_wrapper::instruction::token_escrow_transfer;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn process_token_escrow_transfer(
    toolbox_endpoint: &mut ToolboxEndpoint,
    payer: &Keypair,
    source_authority: &Keypair,
    destination_authority: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    source_slot: u64,
    destination_slot: u64,
    amount: u64,
) -> Result<(), ToolboxEndpointError> {
    let instruction = token_escrow_transfer::instruction(
        &source_authority.pubkey(),
        destination_authority,
        validator,
        token_mint,
        source_slot,
        destination_slot,
        amount,
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
