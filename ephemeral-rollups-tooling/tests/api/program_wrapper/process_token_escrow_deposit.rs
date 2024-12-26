use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

use ephemeral_rollups_wrapper::instruction::token_escrow_deposit;

pub async fn process_token_escrow_deposit(
    endpoint: &mut Endpoint,
    payer: &Keypair,
    source_authority: &Keypair,
    source_token_account: &Pubkey,
    authority: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    slot: u64,
    amount: u64,
) -> Result<(), EndpointError> {
    let instruction = token_escrow_deposit::instruction(
        &source_authority.pubkey(),
        source_token_account,
        authority,
        validator,
        token_mint,
        slot,
        amount,
    );
    endpoint
        .process_instruction_with_signers(instruction, payer, &[source_authority])
        .await?;
    Ok(())
}
