use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use spl_token::instruction::transfer;

use crate::state::token_escrow::TokenEscrow;
use crate::token_escrow_seeds_generator;
use crate::token_vault_seeds_generator;
use crate::util::ensure::ensure_is_owned_by_program;
use crate::util::ensure::ensure_is_pda;
use crate::util::ensure::ensure_is_program_id;
use crate::util::ensure::ensure_is_signer;
use crate::util::signer::signer_seeds;

pub const DISCRIMINANT: [u8; 8] =
    [0xDA, 0xCF, 0x42, 0xDD, 0x24, 0x78, 0x76, 0x44];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub validator: Pubkey,
    pub token_mint: Pubkey,
    pub slot: u64,
    pub amount: u64,
}

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Read instruction inputs
    let [authority, destination_token_account, token_escrow_pda, token_vault_pda, token_program_id] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify the programs
    ensure_is_program_id(token_program_id, &spl_token::ID)?;

    // Verify that the authority user is indeed the one initiating this IX
    ensure_is_signer(authority)?;

    // Verify that the program has proper control of the escrow PDA (and that
    // it's been initialized)
    ensure_is_owned_by_program(token_escrow_pda, program_id)?;

    // Verify that the vault has been initialized properly
    ensure_is_owned_by_program(token_vault_pda, token_program_id.key)?;

    // Verify the seeds of the escrow PDA
    let token_escrow_seeds = token_escrow_seeds_generator!(
        authority.key,
        args.validator,
        args.token_mint,
        args.slot
    );
    ensure_is_pda(token_escrow_pda, token_escrow_seeds, program_id)?;

    // Verify the seeds of the vault PDA
    let token_vault_seeds =
        token_vault_seeds_generator!(args.validator, args.token_mint);
    let token_vault_bump =
        ensure_is_pda(token_vault_pda, token_vault_seeds, program_id)?;

    // Update the escrow amount (panic if not enough amount available)
    let mut token_escrow_data =
        TokenEscrow::try_from_slice(&token_escrow_pda.data.borrow())?;
    if token_escrow_data.discriminant != TokenEscrow::discriminant() {
        return Err(ProgramError::InvalidAccountData);
    }
    token_escrow_data.amount =
        token_escrow_data.amount.checked_sub(args.amount).unwrap();
    token_escrow_data.serialize(
        &mut &mut token_escrow_pda.try_borrow_mut_data()?.as_mut(),
    )?;

    // Proceed to transfer from token_vault_pda to destination_token_account (if
    // everything else succeeded)
    invoke_signed(
        &transfer(
            token_program_id.key,
            token_vault_pda.key,
            destination_token_account.key,
            token_vault_pda.key,
            &[],
            args.amount,
        )?,
        &[
            token_vault_pda.clone(),
            destination_token_account.clone(),
            token_vault_pda.clone(),
        ],
        &[&signer_seeds(token_vault_seeds, &[token_vault_bump])],
    )?;

    // Log outcome
    msg!("Ephemeral Rollups Wrapper: Withdrew from TokenEscrow");
    msg!(" - authority: {} (slot: {})", authority.key, args.slot);
    msg!(" - validator: {}", args.validator);
    msg!(" - token_mint: {}", args.token_mint);
    msg!(" - destination_token_account: {}", destination_token_account.key);
    msg!(
        " - amount: {} (remaining: {})",
        args.amount,
        token_escrow_data.amount
    );

    // Done
    Ok(())
}
