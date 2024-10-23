use borsh::{BorshDeserialize, BorshSerialize};
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::escrow_seeds_generator;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const TAG: [u8; 8] = [0x1c, 0x69, 0x76, 0xee, 0x37, 0xb8, 0xab, 0x4d];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    index: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer, user_funding, user_claimer, validator_id, escrow_pda, magic_context, magic_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the funding user is indeed the one initiating this IX
    ensure_is_signer(user_claimer)?;

    // Verify that the program has proper control of the PDA (and that it's been initialized)
    ensure_is_owned_by_program(escrow_pda, program_id)?;

    // Verify the seeds of the escrow PDA
    let escrow_seeds = escrow_seeds_generator!(
        user_funding.key,
        user_claimer.key,
        validator_id.key,
        args.index
    );
    ensure_is_pda(escrow_pda, escrow_seeds, program_id)?;

    // Request undelegation inside the ER
    commit_and_undelegate_accounts(payer, vec![escrow_pda], magic_context, magic_program)?;

    // Done
    Ok(())
}
