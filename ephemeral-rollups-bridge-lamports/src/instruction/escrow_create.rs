use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::system_program;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::escrow_seeds_generator;
use crate::state::escrow::Escrow;
use crate::util::create::create_pda;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const TAG: [u8; 8] = [0x1a, 0x92, 0xb7, 0x8b, 0x57, 0xad, 0x99, 0x02];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    index: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [payer, user_funding, user_claimer, validator_id, escrow_pda, system_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the funding user is indeed the one initiating this IX
    ensure_is_signer(user_funding)?;

    // Verify that the escrow PDA is currently un-initialized
    ensure_is_owned_by_program(escrow_pda, &system_program::ID)?;

    // Verify the seeds of the escrow PDA
    let escrow_seeds = escrow_seeds_generator!(
        user_funding.key,
        user_claimer.key,
        validator_id.key,
        args.index
    );
    ensure_is_pda(escrow_pda, escrow_seeds, program_id)?;

    // Initialize the escrow PDA
    create_pda(
        payer,
        escrow_pda,
        escrow_seeds,
        Escrow::space(),
        program_id,
        system_program,
    )?;

    // Write the authority keys in the escrow account
    let escrow = Escrow {};
    escrow.serialize(&mut &mut escrow_pda.try_borrow_mut_data()?.as_mut())?;

    // Done
    Ok(())
}
