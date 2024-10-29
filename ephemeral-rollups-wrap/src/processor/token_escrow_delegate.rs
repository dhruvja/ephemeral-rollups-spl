use borsh::{BorshDeserialize, BorshSerialize};
use ephemeral_rollups_sdk::cpi::delegate_account;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::state::token_escrow::TokenEscrow;
use crate::token_escrow_seeds_generator;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0xc6, 0xd6, 0x5c, 0x5f, 0xf8, 0xcc, 0xe0, 0x2c];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub validator: Pubkey,
    pub token_mint: Pubkey,
    pub number: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [payer, authority, token_escrow_pda, delegation_buffer_pda, delegation_record_pda, delegation_metadata_pda, delegation_program_id, owner_program_id, system_program_id] =
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
    ensure_is_owned_by_program(token_escrow_pda, program_id)?;

    // Verify the seeds of the escrow PDA
    let token_escrow_seeds =
        token_escrow_seeds_generator!(authority.key, args.validator, args.token_mint, args.number);
    ensure_is_pda(token_escrow_pda, token_escrow_seeds, program_id)?;

    // Verify that the escrow PDA is properly initalized
    let token_escrow_data = TokenEscrow::try_from_slice(&token_escrow_pda.data.borrow())?;
    if token_escrow_data.discriminant != TokenEscrow::discriminant() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify that the owner_program_id account passed as parameter is valid
    if owner_program_id.key.ne(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Delegate the escrow, relinquish control on chain (it will become usable in the Ephem)
    delegate_account(
        payer,
        token_escrow_pda,
        owner_program_id,
        delegation_buffer_pda,
        delegation_record_pda,
        delegation_metadata_pda,
        delegation_program_id,
        system_program_id,
        token_escrow_seeds,
        i64::MAX,
        u32::MAX,
    )?;

    // Log outcome
    msg!("Ephemeral Rollups Wrap: Delegated a TokenEscrow");
    msg!(" - authority: {} ({})", authority.key, args.number);
    msg!(" - validator: {}", args.validator);
    msg!(" - token_mint: {}", args.token_mint);
    msg!(" - amount: {}", token_escrow_data.amount);

    // Done
    Ok(())
}
