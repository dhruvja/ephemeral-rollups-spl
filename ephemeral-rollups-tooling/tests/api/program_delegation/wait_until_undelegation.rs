use std::time::{Duration, Instant};

use ephemeral_rollups_sdk::consts::DELEGATION_PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::{Endpoint, EndpointError};

pub async fn wait_until_undelegation(
    endpoint: &mut Endpoint,
    account: &Pubkey,
) -> Result<(), EndpointError> {
    let start = Instant::now();
    loop {
        let account_owner = endpoint
            .get_account_owner(&account)
            .await?
            .unwrap_or_default();
        if account_owner != DELEGATION_PROGRAM_ID {
            break; // Alternatively we could look into the logs of the account
        }
        if start.elapsed() > Duration::from_secs(10) {
            return Err(EndpointError::Custom("Undelegation timeout"));
        }
    }
    Ok(())
}
