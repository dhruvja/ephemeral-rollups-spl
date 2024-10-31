use borsh::{BorshDeserialize, BorshSerialize};
use mpl_bubblegum::instructions::{TransferCpi, TransferCpiAccounts, TransferInstructionArgs};
use mpl_bubblegum::utils::get_asset_id;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::{msg, system_program};

use crate::bubblegum_escrow_seeds_generator;
use crate::state::bubblegum_escrow::BubblegumEscrow;
use crate::util::create::create_pda;
use crate::util::ensure::{
    ensure_is_owned_by_program, ensure_is_pda, ensure_is_program_id, ensure_is_signer,
};

pub const DISCRIMINANT: [u8; 8] = [0x34, 0x0b, 0x50, 0x67, 0x14, 0x31, 0x8e, 0x98];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub authority: Pubkey,
    pub validator: Pubkey,
    pub root_hash: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [payer, bubblegum_escrow_pda, tree, tree_config_pda, leaf_owner, leaf_delegate, bubblegum_program_id, compression_program_id, noop_program_id, system_program_id] =
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

    // Verify that the payer is allowed to pay for the rent fees
    ensure_is_signer(payer)?;

    // Verify that the escrow PDA is currently un-initialized
    ensure_is_owned_by_program(bubblegum_escrow_pda, &system_program::ID)?;

    // Which cNFT is being escrowed
    let asset = get_asset_id(&tree.key, args.nonce);

    // Verify the seeds of the escrow PDA
    let bubblegum_escrow_seeds = bubblegum_escrow_seeds_generator!(args.validator, asset);
    let bubblegum_escrow_bump =
        ensure_is_pda(bubblegum_escrow_pda, bubblegum_escrow_seeds, program_id)?;

    // Initialize the escrow PDA
    create_pda(
        payer,
        bubblegum_escrow_pda,
        bubblegum_escrow_seeds,
        bubblegum_escrow_bump,
        BubblegumEscrow::space(),
        program_id,
        system_program_id,
    )?;

    // Initialize the escrow data
    let bubblegum_escrow_data = BubblegumEscrow {
        discriminant: BubblegumEscrow::discriminant(),
        authority: args.authority,
    };
    bubblegum_escrow_data
        .serialize(&mut &mut bubblegum_escrow_pda.try_borrow_mut_data()?.as_mut())?;

    // Transfer ownership from user to the escrow (until redeemed)
    TransferCpi::new(
        bubblegum_program_id,
        TransferCpiAccounts {
            leaf_owner: (leaf_owner, true),
            leaf_delegate: (leaf_delegate, false),
            tree_config: tree_config_pda,
            merkle_tree: tree,
            new_leaf_owner: bubblegum_escrow_pda,
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
    .invoke()?;

    // Log outcome
    msg!("Ephemeral Rollups Wrapper: Deposited into a new BubblegumEscrow");
    msg!(" - authority: {}", args.authority);
    msg!(" - validator: {}", args.validator);
    msg!(" - asset: {}", asset);

    // Done
    Ok(())
}
