use borsh::{BorshDeserialize, BorshSerialize};
use ephemeral_rollups_sdk::cpi::delegate_account;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::state::token_escrow::TokenEscrow;
use crate::token_escrow_seeds_generator;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0xc6, 0xd6, 0x5c, 0x5f, 0xf8, 0xcc, 0xe0, 0x2c];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    index: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [payer, authority, validator_id, token_mint, token_escrow_pda, delegation_buffer, delegation_record, delegation_metadata, delegation_program, owner_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the authority user is indeed the one initiating this IX
    ensure_is_signer(authority)?;

    // Verify that the program has proper control of the PDA (and that it's been initialized)
    ensure_is_owned_by_program(token_escrow_pda, program_id)?;

    // Verify the seeds of the escrow PDA
    let token_escrow_seeds =
        token_escrow_seeds_generator!(authority.key, validator_id.key, token_mint.key, args.index);
    ensure_is_pda(token_escrow_pda, token_escrow_seeds, program_id)?;

    // Verify that the escrow PDA is properly initalized
    let token_escrow_data = TokenEscrow::try_from_slice(&token_escrow_pda.data.borrow())?;
    if !token_escrow_data.initialized {
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify that the owner_program account passed as parameter is valid
    if owner_program.key.ne(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Delegate the escrow, relinquish control on chain (it will become usable in the Ephem)
    delegate_account(
        payer,
        token_escrow_pda,
        owner_program,
        delegation_buffer,
        delegation_record,
        delegation_metadata,
        delegation_program,
        system_program,
        token_escrow_seeds,
        i64::MAX,
        u32::MAX,
    )?;

    // Done
    Ok(())
}
