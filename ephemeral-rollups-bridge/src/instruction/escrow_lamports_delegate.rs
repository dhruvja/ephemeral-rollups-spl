use borsh::{BorshDeserialize, BorshSerialize};
use ephemeral_rollups_sdk::cpi::delegate_account;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::state::escrow_lamports::EscrowLamports;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0x98, 0xe4, 0x41, 0xd1, 0x81, 0xb6, 0xc9, 0x3b];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    index: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer, user_funding, user_claimer, validator_id, escrow_lamports_pda, delegation_buffer, delegation_record, delegation_metadata, delegation_program, owner_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the funding user is indeed the one initiating this IX
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

    // Verify that the owner_program account passed as parameter is valid
    if owner_program.key.ne(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Delegate the escrow, relinquish control on chain (it will become claimable in the Ephem)
    delegate_account(
        payer,
        escrow_lamports_pda,
        owner_program,
        delegation_buffer,
        delegation_record,
        delegation_metadata,
        delegation_program,
        system_program,
        escrow_lamports_seeds,
        i64::MAX,
        u32::MAX,
    )?;

    Ok(())
}
