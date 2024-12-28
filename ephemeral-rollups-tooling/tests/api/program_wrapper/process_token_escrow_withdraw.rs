use ephemeral_rollups_wrapper::instruction::token_escrow_withdraw;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn process_token_escrow_withdraw(
    toolbox_endpoint: &mut ToolboxEndpoint,
    payer: &Keypair,
    authority: &Keypair,
    destination_token_account: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
    slot: u64,
    amount: u64,
) -> Result<(), ToolboxEndpointError> {
    let instruction = token_escrow_withdraw::instruction(
        &authority.pubkey(),
        destination_token_account,
        validator,
        token_mint,
        slot,
        amount,
    );
    toolbox_endpoint
        .process_instruction_with_signers(instruction, payer, &[authority])
        .await?;
    Ok(())
}
