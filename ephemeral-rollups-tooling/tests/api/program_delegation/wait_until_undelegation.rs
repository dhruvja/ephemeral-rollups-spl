use std::time::{Duration, Instant};

use ephemeral_rollups_sdk::consts::DELEGATION_PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;

use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;
use crate::api::program_context::read_account::read_account_owner;

pub async fn wait_until_undelegation(
    program_context: &mut Box<dyn ProgramContext>,
    account: &Pubkey,
) -> Result<(), ProgramError> {
    let start = Instant::now();
    loop {
        let account_owner = read_account_owner(program_context, &account)
            .await?
            .unwrap_or_default();
        if account_owner != DELEGATION_PROGRAM_ID {
            break; // Alternatively we could look into the logs of the account
        }
        if start.elapsed() > Duration::from_secs(10) {
            return Err(ProgramError::Custom("Undelegation timeout"));
        }
    }
    Ok(())
}
