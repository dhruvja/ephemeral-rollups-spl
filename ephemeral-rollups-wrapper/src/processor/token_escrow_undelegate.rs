use borsh::{BorshDeserialize, BorshSerialize};
use ephemeral_rollups_sdk::consts::MAGIC_PROGRAM_ID;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::state::token_escrow::TokenEscrow;
use crate::token_escrow_seeds_generator;
use crate::util::ensure::{
    ensure_is_owned_by_program, ensure_is_pda, ensure_is_program_id, ensure_is_signer,
};

pub const DISCRIMINANT: [u8; 8] = [0x4b, 0x9c, 0x96, 0x18, 0xdf, 0x98, 0x31, 0x24];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub validator: Pubkey,
    pub token_mint: Pubkey,
    pub slot: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer, authority, token_escrow_pda, magic_context_pda, magic_program_id] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify the programs
    ensure_is_program_id(magic_program_id, &MAGIC_PROGRAM_ID)?;

    // Verify that the payer is allowed to pay for the rent fees
    ensure_is_signer(payer)?;

    // Verify that the authority user is indeed the one initiating this IX
    ensure_is_signer(authority)?;

    // Verify that the program has proper control of the PDA (and that it's been initialized)
    ensure_is_owned_by_program(token_escrow_pda, program_id)?;

    // Verify the seeds of the escrow PDA
    let token_escrow_seeds =
        token_escrow_seeds_generator!(authority.key, args.validator, args.token_mint, args.slot);
    ensure_is_pda(token_escrow_pda, token_escrow_seeds, program_id)?;

    // Verify that the escrow PDA is properly initalized
    let token_escrow_data = TokenEscrow::try_from_slice(&token_escrow_pda.data.borrow())?;
    if token_escrow_data.discriminant != TokenEscrow::discriminant() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Request undelegation inside the ER
    commit_and_undelegate_accounts(
        payer,
        vec![token_escrow_pda],
        magic_context_pda,
        magic_program_id,
    )?;

    // Done
    Ok(())
}
