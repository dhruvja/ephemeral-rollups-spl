use borsh::BorshDeserialize;
use ephemeral_rollups_sdk::cpi::undelegate_account;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [delegated_account, delegation_buffer, payer, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let pda_seeds =
        <Vec<Vec<u8>>>::try_from_slice(data).map_err(|_| ProgramError::InvalidInstructionData)?;

    // Allow undelegating on our behalf
    undelegate_account(
        delegated_account,
        &crate::id(),
        delegation_buffer,
        payer,
        system_program,
        pda_seeds,
    )?;

    // Done
    Ok(())
}
