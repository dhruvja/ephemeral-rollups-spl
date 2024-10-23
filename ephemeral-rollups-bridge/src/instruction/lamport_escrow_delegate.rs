use borsh::{BorshDeserialize, BorshSerialize};
use ephemeral_rollups_sdk::cpi::delegate_account;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::lamport_escrow_seeds_generator;
use crate::state::lamport_escrow::LamportEscrow;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0x98, 0xe4, 0x41, 0xd1, 0x81, 0xb6, 0xc9, 0x3b];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub index: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [payer, authority, validator, lamport_escrow_pda, delegation_buffer_pda, delegation_record_pda, delegation_metadata_pda, delegation_program_id, owner_program_id, system_program_id] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

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

    // Verify that the owner_program_id account passed as parameter is valid
    if owner_program_id.key.ne(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Delegate the escrow, relinquish control on chain (it will become claimable in the Ephem)
    delegate_account(
        payer,
        lamport_escrow_pda,
        owner_program_id,
        delegation_buffer_pda,
        delegation_record_pda,
        delegation_metadata_pda,
        delegation_program_id,
        system_program_id,
        lamport_escrow_seeds,
        i64::MAX,
        u32::MAX,
    )?;

    // Done
    Ok(())
}
