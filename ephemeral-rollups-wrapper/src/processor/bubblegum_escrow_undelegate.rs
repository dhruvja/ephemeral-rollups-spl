use borsh::{BorshDeserialize, BorshSerialize};
use ephemeral_rollups_sdk::consts::MAGIC_PROGRAM_ID;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;
use mpl_bubblegum::utils::get_asset_id;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::bubblegum_escrow_seeds_generator;
use crate::state::bubblegum_escrow::BubblegumEscrow;
use crate::util::ensure::{
    ensure_is_owned_by_program, ensure_is_pda, ensure_is_program_id, ensure_is_signer,
};

pub const DISCRIMINANT: [u8; 8] = [0xaa, 0x98, 0xa4, 0x02, 0xa7, 0xee, 0x30, 0x93];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub validator: Pubkey,
    pub tree: Pubkey,
    pub nonce: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer, authority, bubblegum_escrow_pda, magic_context_pda, magic_program_id] = accounts
    else {
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
    ensure_is_owned_by_program(bubblegum_escrow_pda, program_id)?;

    // Which cNFT is being escrowed
    let asset = get_asset_id(&args.tree, args.nonce);

    // Verify the seeds of the escrow PDA
    let bubblegum_escrow_seeds = bubblegum_escrow_seeds_generator!(args.validator, asset);
    ensure_is_pda(bubblegum_escrow_pda, bubblegum_escrow_seeds, program_id)?;

    // Verify that the escrow PDA is properly initalized
    let bubblegum_escrow_data =
        BubblegumEscrow::try_from_slice(&bubblegum_escrow_pda.data.borrow())?;
    if bubblegum_escrow_data.discriminant != BubblegumEscrow::discriminant() {
        return Err(ProgramError::InvalidAccountData);
    }
    if bubblegum_escrow_data.authority != *authority.key {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Request undelegation inside the ER
    commit_and_undelegate_accounts(
        payer,
        vec![bubblegum_escrow_pda],
        magic_context_pda,
        magic_program_id,
    )?;

    // Log outcome
    msg!("Ephemeral Rollups Wrapper: Requested undelegation of a BubblegumEscrow");
    msg!(" - authority: {}", authority.key);
    msg!(" - validator: {}", args.validator);
    msg!(" - asset: {}", asset);

    // Done
    Ok(())
}
