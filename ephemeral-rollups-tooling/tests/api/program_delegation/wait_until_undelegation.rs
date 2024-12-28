use std::time::Duration;
use std::time::Instant;

use ephemeral_rollups_sdk::consts::DELEGATION_PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn wait_until_undelegation(
    toolbox_endpoint: &mut ToolboxEndpoint,
    account: &Pubkey,
) -> Result<(), ToolboxEndpointError> {
    let start = Instant::now();
    loop {
        if toolbox_endpoint.get_account_owner(account).await?
            != DELEGATION_PROGRAM_ID
        {
            break; // Alternatively we could look into the logs of the account
        }
        if start.elapsed() > Duration::from_secs(10) {
            return Err(ToolboxEndpointError::Custom("Undelegation timeout"));
        }
    }
    Ok(())
}
