use borsh::BorshDeserialize;
use ephemeral_rollups_sdk::cpi::undelegate_account;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::util::ensure::ensure_is_program_id;

pub fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Read instruction inputs
    let [delegated_account, delegation_buffer, payer, system_program_id] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let pda_seeds = <Vec<Vec<u8>>>::try_from_slice(data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Verify the programs
    ensure_is_program_id(system_program_id, &system_program::ID)?;

    // Allow undelegating to ourselves on our behalf
    undelegate_account(
        delegated_account,
        &crate::ID,
        delegation_buffer,
        payer,
        system_program_id,
        pda_seeds,
    )?;

    // Done
    Ok(())
}
