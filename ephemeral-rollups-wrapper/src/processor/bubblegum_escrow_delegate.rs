use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use ephemeral_rollups_sdk::consts::DELEGATION_PROGRAM_ID;
use ephemeral_rollups_sdk::cpi::delegate_account;
use mpl_bubblegum::utils::get_asset_id;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::bubblegum_escrow_seeds_generator;
use crate::state::bubblegum_escrow::BubblegumEscrow;
use crate::util::ensure::ensure_is_owned_by_program;
use crate::util::ensure::ensure_is_pda;
use crate::util::ensure::ensure_is_program_id;
use crate::util::ensure::ensure_is_signer;

pub const DISCRIMINANT: [u8; 8] =
    [0xC6, 0x1B, 0x66, 0xB2, 0x82, 0xEC, 0xF1, 0x5A];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub validator: Pubkey,
    pub tree: Pubkey,
    pub nonce: u64,
}

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Read instruction inputs
    let [payer, authority, bubblegum_escrow_pda, delegation_buffer_pda, delegation_record_pda, delegation_metadata_pda, delegation_program_id, owner_program_id, system_program_id] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify the programs
    ensure_is_program_id(delegation_program_id, &DELEGATION_PROGRAM_ID)?;
    ensure_is_program_id(owner_program_id, program_id)?;
    ensure_is_program_id(system_program_id, &system_program::ID)?;

    // Verify that the payer is allowed to pay for the rent fees
    ensure_is_signer(payer)?;

    // Verify that the authority user is indeed the one initiating this IX
    ensure_is_signer(authority)?;

    // Verify that the program has proper control of the PDA (and that it's been
    // initialized)
    ensure_is_owned_by_program(bubblegum_escrow_pda, program_id)?;

    // Which cNFT is being escrowed
    let asset = get_asset_id(&args.tree, args.nonce);

    // Verify the seeds of the escrow PDA
    let bubblegum_escrow_seeds =
        bubblegum_escrow_seeds_generator!(args.validator, asset);
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

    // Delegate the escrow, relinquish control on chain (it will become usable
    // in the Ephem)
    delegate_account(
        payer,
        bubblegum_escrow_pda,
        owner_program_id,
        delegation_buffer_pda,
        delegation_record_pda,
        delegation_metadata_pda,
        delegation_program_id,
        system_program_id,
        bubblegum_escrow_seeds,
        i64::MAX,
        u32::MAX,
    )?;

    // Log outcome
    msg!("Ephemeral Rollups Wrapper: Delegated a BubblegumEscrow");
    msg!(" - authority: {}", authority.key);
    msg!(" - validator: {}", args.validator);
    msg!(" - asset: {}", asset);

    // Done
    Ok(())
}
