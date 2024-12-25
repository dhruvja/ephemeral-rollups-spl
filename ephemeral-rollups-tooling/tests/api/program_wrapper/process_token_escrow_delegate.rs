use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

use ephemeral_rollups_wrapper::instruction::token_escrow_delegate;

pub async fn process_token_escrow_delegate(
    endpoint: &mut Endpoint,
    payer: &Keypair,
    authority: &Keypair,
    validator: &Pubkey,
    token_mint: &Pubkey,
    slot: u64,
) -> Result<(), EndpointError> {
    let instruction = token_escrow_delegate::instruction(
        &payer.pubkey(),
        &authority.pubkey(),
        validator,
        token_mint,
        slot,
    );
    endpoint
        .process_instruction_with_signers(instruction, payer, &[authority])
        .await?;
    Ok(())
}
