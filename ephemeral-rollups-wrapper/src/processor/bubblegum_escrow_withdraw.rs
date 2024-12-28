use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use mpl_bubblegum::instructions::TransferCpi;
use mpl_bubblegum::instructions::TransferCpiAccounts;
use mpl_bubblegum::instructions::TransferInstructionArgs;
use mpl_bubblegum::utils::get_asset_id;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::bubblegum_escrow_seeds_generator;
use crate::state::bubblegum_escrow::BubblegumEscrow;
use crate::util::close::close_pda;
use crate::util::ensure::ensure_is_owned_by_program;
use crate::util::ensure::ensure_is_pda;
use crate::util::ensure::ensure_is_program_id;
use crate::util::ensure::ensure_is_signer;
use crate::util::signer::signer_seeds;

pub const DISCRIMINANT: [u8; 8] =
    [0xA7, 0x91, 0x66, 0x54, 0xDF, 0xBA, 0xFB, 0x67];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub validator: Pubkey,
    pub root_hash: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Read instruction inputs
    let [authority, new_leaf_owner, bubblegum_escrow_pda, spill, tree, tree_config_pda, bubblegum_program_id, compression_program_id, noop_program_id, system_program_id] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify the programs
    ensure_is_program_id(bubblegum_program_id, &mpl_bubblegum::ID)?;
    ensure_is_program_id(compression_program_id, &spl_account_compression::ID)?;
    ensure_is_program_id(noop_program_id, &spl_noop::ID)?;
    ensure_is_program_id(system_program_id, &system_program::ID)?;

    // Verify that the authority user is indeed the one initiating this IX
    ensure_is_signer(authority)?;

    // Verify that the program has proper control of the escrow PDA (and that
    // it's been initialized)
    ensure_is_owned_by_program(bubblegum_escrow_pda, program_id)?;

    // Which cNFT is being escrowed
    let asset = get_asset_id(tree.key, args.nonce);

    // Verify the seeds of the escrow PDA
    let bubblegum_escrow_seeds =
        bubblegum_escrow_seeds_generator!(args.validator, asset);
    let bubblegum_escrow_bump = ensure_is_pda(
        bubblegum_escrow_pda,
        bubblegum_escrow_seeds,
        program_id,
    )?;

    // Verify that the escrow PDA is properly initalized
    let bubblegum_escrow_data =
        BubblegumEscrow::try_from_slice(&bubblegum_escrow_pda.data.borrow())?;
    if bubblegum_escrow_data.discriminant != BubblegumEscrow::discriminant() {
        return Err(ProgramError::InvalidAccountData);
    }
    if bubblegum_escrow_data.authority != *authority.key {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Transfer ownership from escrow back to user
    TransferCpi::new(
        bubblegum_program_id,
        TransferCpiAccounts {
            leaf_owner: (bubblegum_escrow_pda, true),
            leaf_delegate: (bubblegum_escrow_pda, false),
            tree_config: tree_config_pda,
            merkle_tree: tree,
            new_leaf_owner,
            log_wrapper: noop_program_id,
            compression_program: compression_program_id,
            system_program: system_program_id,
        },
        TransferInstructionArgs {
            root: args.root_hash,
            data_hash: args.data_hash,
            creator_hash: args.creator_hash,
            nonce: args.nonce,
            index: args.index,
        },
    )
    .invoke_signed(&[&signer_seeds(
        bubblegum_escrow_seeds,
        &[bubblegum_escrow_bump],
    )])?;

    // Close the escrow PDA
    close_pda(bubblegum_escrow_pda, spill)?;

    // Log outcome
    msg!("Ephemeral Rollups Wrapper: Withdraw from a BubblegumEscrow");
    msg!(" - authority: {}", authority.key);
    msg!(" - validator: {}", args.validator);
    msg!(" - asset: {}", asset);
    msg!(" - new_leaf_owner: {}", new_leaf_owner.key);

    // Done
    Ok(())
}
