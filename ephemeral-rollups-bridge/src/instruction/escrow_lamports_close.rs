use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::rent::Rent;
use solana_program::system_instruction::transfer;
use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::state::escrow_lamports::EscrowLamports;
use crate::util::close::close_pda;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0xcd, 0xde, 0x5a, 0xf0, 0x3b, 0x67, 0x97, 0xc0];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    index: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer, user_funding, user_claimer, validator_id, escrow_lamports_pda, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the funding user is indeed the one initiating this IX
    ensure_is_signer(payer)?;
    ensure_is_signer(user_funding)?;

    // Verify that the program has proper control of the PDA (and that it's been initialized)
    ensure_is_owned_by_program(escrow_lamports_pda, program_id)?;

    // Verify the seeds of the escrow PDA
    let escrow_lamports_seeds = &[
        // TODO - write seeds generator macro
        EscrowLamports::SEEDS_PREFIX,
        &user_funding.key.to_bytes(),
        &user_claimer.key.to_bytes(),
        &validator_id.key.to_bytes(),
        &args.index.to_le_bytes(),
    ];
    ensure_is_pda(escrow_lamports_pda, escrow_lamports_seeds, program_id)?;

    // Verify that the funding user is the authority for this escrow PDA
    let escrow_lamports =
        EscrowLamports::try_from_slice(&mut &**escrow_lamports_pda.data.borrow())?;
    if user_funding.key.ne(&escrow_lamports.user_funding) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Send all the remaining lamports back to the funding user
    let minimum_lamports = Rent::get()?.minimum_balance(EscrowLamports::space());
    let remaining_lamports = escrow_lamports_pda
        .lamports()
        .saturating_sub(minimum_lamports);
    invoke_signed(
        &transfer(
            escrow_lamports_pda.key,
            user_funding.key,
            remaining_lamports,
        ),
        &[escrow_lamports_pda.clone(), user_funding.clone()],
        &[escrow_lamports_seeds],
    )?;

    // Close the PDA
    close_pda(
        payer,
        escrow_lamports_pda,
        escrow_lamports_seeds,
        system_program,
    )?;

    // Done
    Ok(())
}
