use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::rent::Rent;
use solana_program::system_instruction::transfer;
use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::lamport_escrow_seeds_generator;
use crate::state::lamport_escrow::LamportEscrow;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0x62, 0x2b, 0x40, 0xa9, 0xc1, 0xe1, 0x1d, 0x72];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    index: u64,
    lamports: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [authority, validator_id, lamport_escrow_pda] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the authority user is indeed the one initiating this IX
    ensure_is_signer(authority)?;

    // Verify that the program has proper control of the PDA (and that it's been initialized)
    ensure_is_owned_by_program(lamport_escrow_pda, program_id)?;

    // Verify the seeds of the escrow PDA
    let lamport_escrow_seeds =
        lamport_escrow_seeds_generator!(authority.key, validator_id.key, args.index);
    ensure_is_pda(lamport_escrow_pda, lamport_escrow_seeds, program_id)?;

    // Verify that the escrow PDA is properly initalized
    let lamport_escrow_data = LamportEscrow::try_from_slice(&lamport_escrow_pda.data.borrow())?;
    if !lamport_escrow_data.initialized {
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify that the escrow PDA has a sufficient amount of available lamports to claim
    let minimum_lamports = Rent::get()?.minimum_balance(LamportEscrow::space());
    let claimable_lamports = lamport_escrow_pda
        .lamports()
        .saturating_sub(minimum_lamports);
    if args.lamports > claimable_lamports {
        return Err(ProgramError::InsufficientFunds);
    }

    // Send the lamports to the authority account
    invoke_signed(
        &transfer(lamport_escrow_pda.key, authority.key, args.lamports),
        &[lamport_escrow_pda.clone(), authority.clone()],
        &[lamport_escrow_seeds],
    )?;

    // Done
    Ok(())
}
