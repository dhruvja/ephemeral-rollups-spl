use borsh::{BorshDeserialize, BorshSerialize};
use mpl_bubblegum::utils::get_asset_id;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::bubblegum_escrow_seeds_generator;
use crate::state::bubblegum_escrow::BubblegumEscrow;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0x85, 0xd7, 0x3a, 0x53, 0x9f, 0xda, 0xfa, 0x5c];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub destination_authority: Pubkey,
    pub validator: Pubkey,
    pub nonce: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [source_authority, bubblegum_escrow_pda, tree] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the authority user is indeed the one initiating this IX
    ensure_is_signer(source_authority)?;

    // Verify that the program has proper control of the escrow PDA (and that it's been initialized)
    ensure_is_owned_by_program(bubblegum_escrow_pda, program_id)?;

    // Which cNFT is being escrowed
    let asset = get_asset_id(&tree.key, args.nonce);

    // Verify the seeds of the escrow PDA
    let bubblegum_escrow_seeds = bubblegum_escrow_seeds_generator!(args.validator, asset);
    ensure_is_pda(bubblegum_escrow_pda, bubblegum_escrow_seeds, program_id)?;

    // Update the escrow authority
    let mut bubblegum_escrow_data =
        BubblegumEscrow::try_from_slice(&bubblegum_escrow_pda.data.borrow())?;
    if bubblegum_escrow_data.discriminant != BubblegumEscrow::discriminant() {
        return Err(ProgramError::InvalidAccountData);
    }
    if bubblegum_escrow_data.authority != *source_authority.key {
        return Err(ProgramError::InvalidAccountOwner);
    }
    bubblegum_escrow_data.authority = args.destination_authority;
    bubblegum_escrow_data
        .serialize(&mut &mut bubblegum_escrow_pda.try_borrow_mut_data()?.as_mut())?;

    // Log outcome
    msg!("Ephemeral Rollups Wrapper: Transfered a BubblegumEscrow");
    msg!(" - source_authority: {}", source_authority.key);
    msg!(" - destination_authority: {}", args.destination_authority);
    msg!(" - validator: {}", args.validator);
    msg!(" - asset: {}", asset);

    // Done
    Ok(())
}
