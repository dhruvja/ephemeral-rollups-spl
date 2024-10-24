use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use spl_token::instruction::transfer;

use crate::state::token_escrow::TokenEscrow;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};
use crate::util::seeds::seeds_signer_for_pda;
use crate::{token_escrow_seeds_generator, token_vault_seeds_generator};

pub const DISCRIMINANT: [u8; 8] = [0xda, 0xcf, 0x42, 0xdd, 0x24, 0x78, 0x76, 0x44];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub index: u64,
    pub amount: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [destination_account, authority, validator, token_mint, token_escrow_pda, token_vault_pda, token_program] =
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
        token_escrow_seeds_generator!(authority.key, validator.key, token_mint.key, args.index);
    ensure_is_pda(token_escrow_pda, token_escrow_seeds, program_id)?;

    // Verify the seeds of the vault PDA
    let token_vault_seeds = token_vault_seeds_generator!(validator.key, token_mint.key);
    let token_vault_bump = ensure_is_pda(token_vault_pda, token_vault_seeds, program_id)?;

    // Update the escrow amount (panic if not enough amount available)
    let mut token_escrow_data = TokenEscrow::try_from_slice(&token_escrow_pda.data.borrow())?;
    if token_escrow_data.discriminant != TokenEscrow::discriminant() {
        return Err(ProgramError::InvalidAccountData);
    }
    token_escrow_data.amount = token_escrow_data.amount.checked_sub(args.amount).unwrap();
    token_escrow_data.serialize(&mut &mut token_escrow_pda.try_borrow_mut_data()?.as_mut())?;

    // Proceed to transfer from vault to destination_account (if everything else succeeded)
    invoke_signed(
        &transfer(
            token_program.key,
            token_vault_pda.key,
            destination_account.key,
            token_vault_pda.key,
            &[],
            args.amount,
        )?,
        &[
            token_vault_pda.clone(),
            destination_account.clone(),
            token_vault_pda.clone(),
        ],
        &[&seeds_signer_for_pda(
            token_vault_seeds,
            &[token_vault_bump],
        )],
    )?;

    // Done
    Ok(())
}
