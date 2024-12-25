use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

use ephemeral_rollups_wrapper::instruction::token_escrow_transfer;

pub async fn process_token_escrow_transfer(
    endpoint: &mut Endpoint,
    payer: &Keypair,
    source_authority: &Keypair,
    destination_authority: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    source_slot: u64,
    destination_slot: u64,
    amount: u64,
) -> Result<(), EndpointError> {
    let instruction = token_escrow_transfer::instruction(
        &source_authority.pubkey(),
        destination_authority,
        validator,
        token_mint,
        source_slot,
        destination_slot,
        amount,
    );
    endpoint
        .process_instruction_with_signers(instruction, payer, &[source_authority])
        .await?;
    Ok(())
}
