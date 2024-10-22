use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::system_program;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::state::escrow_lamports::EscrowLamports;
use crate::util::create::create_pda;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0x1a, 0x92, 0xb7, 0x8b, 0x57, 0xad, 0x99, 0x02];

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

    // Verify that the escrow PDA is currently un-initialized
    ensure_is_owned_by_program(escrow_lamports_pda, &system_program::ID)?;

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

    // Initialize the escrow PDA
    create_pda(
        payer,
        escrow_lamports_pda,
        escrow_lamports_seeds,
        EscrowLamports::space(),
        program_id,
        system_program,
    )?;

    // Write the authority keys for the escrow
    let mut escrow_lamports =
        EscrowLamports::deserialize(&mut &**escrow_lamports_pda.data.borrow())?;

    escrow_lamports.user_funding = *user_funding.key;
    escrow_lamports.user_claimer = *user_claimer.key;

    escrow_lamports.serialize(&mut &mut escrow_lamports_pda.try_borrow_mut_data()?.as_mut())?;

    // Done
    Ok(())
}
