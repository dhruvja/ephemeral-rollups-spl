use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::state::token_escrow::TokenEscrow;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};
use crate::{token_escrow_seeds_generator, token_vault_seeds_generator};

pub const DISCRIMINANT: [u8; 8] = [0xda, 0xcf, 0x42, 0xdd, 0x24, 0x78, 0x76, 0x44];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    index: u64,
    amount: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [token_account, authority, validator_id, token_mint, token_escrow_pda, token_vault_pda] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the authority user is indeed the one initiating this IX
    ensure_is_signer(authority)?;

    // Verify that the program has proper control of the escrow PDA (and that it's been initialized)
    ensure_is_owned_by_program(token_escrow_pda, program_id)?;

    // Verify that the program has proper control of the vault PDA (and that it's been initialized)
    ensure_is_owned_by_program(token_vault_pda, program_id)?;

    // Verify the seeds of the escrow PDA
    let token_escrow_seeds =
        token_escrow_seeds_generator!(authority.key, validator_id.key, token_mint.key, args.index);
    ensure_is_pda(token_escrow_pda, token_escrow_seeds, program_id)?;

    // Verify the seeds of the vault PDA
    let token_vault_seeds = token_vault_seeds_generator!(validator_id.key, token_mint.key);
    ensure_is_pda(token_vault_pda, token_vault_seeds, program_id)?;

    // TODO - proceed to transfer from vault to token_account

    // Update the escrow amount
    let mut token_escrow_data = TokenEscrow::try_from_slice(&token_escrow_pda.data.borrow())?;
    token_escrow_data.amount -= args.amount;
    token_escrow_data.serialize(&mut &mut token_escrow_pda.try_borrow_mut_data()?.as_mut())?;

    // Done
    Ok(())
}
