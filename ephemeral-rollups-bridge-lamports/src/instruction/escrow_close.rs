use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::rent::Rent;
use solana_program::system_instruction::transfer;
use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::escrow_seeds_generator;
use crate::state::escrow::Escrow;
use crate::util::close::close_pda;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const TAG: [u8; 8] = [0xcd, 0xde, 0x5a, 0xf0, 0x3b, 0x67, 0x97, 0xc0];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    index: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [user_funding, user_claimer, validator_id, escrow_pda, spill, system_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the funding user is indeed the one initiating this IX
    ensure_is_signer(user_funding)?;

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

    // Send all the remaining lamports back to the funding user
    let minimum_lamports = Rent::get()?.minimum_balance(Escrow::space());
    let remaining_lamports = escrow_pda.lamports().saturating_sub(minimum_lamports);
    invoke_signed(
        &transfer(escrow_pda.key, user_funding.key, remaining_lamports),
        &[escrow_pda.clone(), user_funding.clone()],
        &[escrow_seeds],
    )?;

    // Close the PDA
    close_pda(escrow_pda, escrow_seeds, spill, system_program)?;

    // Done
    Ok(())
}
