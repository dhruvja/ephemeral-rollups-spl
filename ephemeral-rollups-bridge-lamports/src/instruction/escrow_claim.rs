use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::rent::Rent;
use solana_program::system_instruction::transfer;
use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::escrow_seeds_generator;
use crate::state::escrow::Escrow;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const TAG: [u8; 8] = [0x62, 0x2b, 0x40, 0xa9, 0xc1, 0xe1, 0x1d, 0x72];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    index: u64,
    lamports: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [user_funding, user_claimer, validator_id, escrow_pda] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the claimer user is indeed the one initiating this IX
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

    // Verify that the escrow PDA has a sufficient amount of available lamports to claim
    let minimum_lamports = Rent::get()?.minimum_balance(Escrow::space());
    let claimable_lamports = escrow_pda.lamports().saturating_sub(minimum_lamports);

    if args.lamports > claimable_lamports {
        return Err(ProgramError::InsufficientFunds);
    }

    // Send the lamports to the claimer account
    invoke_signed(
        &transfer(escrow_pda.key, user_claimer.key, args.lamports),
        &[escrow_pda.clone(), user_claimer.clone()],
        &[escrow_seeds],
    )?;

    // Done
    Ok(())
}
