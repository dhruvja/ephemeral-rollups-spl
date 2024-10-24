use borsh::{BorshDeserialize, BorshSerialize};
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::lamport_escrow_seeds_generator;
use crate::state::lamport_escrow::LamportEscrow;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0x1c, 0x69, 0x76, 0xee, 0x37, 0xb8, 0xab, 0x4d];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub index: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer, authority, validator, lamport_escrow_pda, magic_context_pda, magic_program_id] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the payer is allowed to pay for the rent fees
    ensure_is_signer(payer)?;

    // Verify that the authority user is indeed the one initiating this IX
    ensure_is_signer(authority)?;

    // Verify that the program has proper control of the PDA (and that it's been initialized)
    ensure_is_owned_by_program(lamport_escrow_pda, program_id)?;

    // Verify the seeds of the escrow PDA
    let lamport_escrow_seeds =
        lamport_escrow_seeds_generator!(authority.key, validator.key, args.index);
    ensure_is_pda(lamport_escrow_pda, lamport_escrow_seeds, program_id)?;

    // Verify that the escrow PDA is properly initalized
    let lamport_escrow_data = LamportEscrow::try_from_slice(&lamport_escrow_pda.data.borrow())?;
    if lamport_escrow_data.discriminant != LamportEscrow::discriminant() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Request undelegation inside the ER
    commit_and_undelegate_accounts(
        payer,
        vec![lamport_escrow_pda],
        magic_context_pda,
        magic_program_id,
    )?;

    // Done
    Ok(())
}
