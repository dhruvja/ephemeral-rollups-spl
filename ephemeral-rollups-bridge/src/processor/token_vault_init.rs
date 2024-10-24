use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::{msg, system_program};
use spl_token::instruction::initialize_account3;
use spl_token::state::Account;

use crate::token_vault_seeds_generator;
use crate::util::create::create_pda;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0x70, 0xfe, 0x66, 0x40, 0x47, 0x49, 0x16, 0x0e];

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [payer, validator, token_mint, token_vault_pda, token_program_id, system_program_id] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Verify that the payer is allowed to pay for the rent fees
    ensure_is_signer(payer)?;

    // Verify that the vault PDA is currently un-initialized
    ensure_is_owned_by_program(token_vault_pda, &system_program::ID)?;

    // Verify the seeds of the vault PDA
    let token_vault_seeds = token_vault_seeds_generator!(validator.key, token_mint.key);
    let token_vault_bump = ensure_is_pda(token_vault_pda, token_vault_seeds, program_id)?;

    // Initialize the vault PDA
    create_pda(
        payer,
        token_vault_pda,
        token_vault_seeds,
        token_vault_bump,
        Account::LEN,
        token_program_id.key,
        system_program_id,
    )?;

    // Write the spl token vault's content
    invoke(
        &initialize_account3(
            token_program_id.key,
            token_vault_pda.key,
            token_mint.key,
            token_vault_pda.key,
        )?,
        &[token_vault_pda.clone(), token_mint.clone()],
    )?;

    // Log outcome
    msg!("Ephemeral Rollups Bridge: Created a new vault for token mint");
    msg!(" - validator: {}", validator.key);
    msg!(" - token_mint: {}", token_mint.key);

    // Done
    Ok(())
}
