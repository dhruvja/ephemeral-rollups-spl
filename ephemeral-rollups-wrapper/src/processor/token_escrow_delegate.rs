use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use ephemeral_rollups_sdk::consts::DELEGATION_PROGRAM_ID;
use ephemeral_rollups_sdk::cpi::delegate_account;
use ephemeral_rollups_sdk::cpi::DelegateAccounts;
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::state::token_escrow::TokenEscrow;
use crate::token_escrow_seeds_generator;
use crate::util::ensure::ensure_is_owned_by_program;
use crate::util::ensure::ensure_is_pda;
use crate::util::ensure::ensure_is_program_id;
use crate::util::ensure::ensure_is_signer;

pub const DISCRIMINANT: [u8; 8] =
    [0xC6, 0xD6, 0x5C, 0x5F, 0xF8, 0xCC, 0xE0, 0x2C];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub validator: Pubkey,
    pub token_mint: Pubkey,
    pub slot: u64,
}

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Read instruction inputs
    let [payer, authority, token_escrow_pda, delegation_buffer_pda, delegation_record_pda, delegation_metadata_pda, delegation_program_id, owner_program_id, system_program_id] =
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
    ensure_is_owned_by_program(token_escrow_pda, program_id)?;

    // Verify the seeds of the escrow PDA
    let token_escrow_seeds = token_escrow_seeds_generator!(
        authority.key,
        args.validator,
        args.token_mint,
        args.slot
    );
    ensure_is_pda(token_escrow_pda, token_escrow_seeds, program_id)?;

    // Verify that the escrow PDA is properly initalized
    let token_escrow_data =
        TokenEscrow::try_from_slice(&token_escrow_pda.data.borrow())?;
    if token_escrow_data.discriminant != TokenEscrow::discriminant() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Delegate the escrow, relinquish control on chain (it will become usable
    // in the Ephem)
    let accounts = DelegateAccounts {
        payer,
        pda: token_escrow_pda,
        owner_program: owner_program_id,
        buffer: delegation_buffer_pda,
        delegation_record: delegation_record_pda,
        delegation_metadata: delegation_metadata_pda,
        delegation_program: delegation_program_id,
        system_program: system_program_id,
    };
    // Delegate the escrow, relinquish control on chain (it will become
    // claimable in the Ephem)
    delegate_account(
        accounts,
        token_escrow_seeds, 
        DelegateConfig::default(), 
    )?;

    // Log outcome
    msg!("Ephemeral Rollups Wrapper: Delegated a TokenEscrow");
    msg!(" - authority: {} (slot: {})", authority.key, args.slot);
    msg!(" - validator: {}", args.validator);
    msg!(" - token_mint: {}", args.token_mint);
    msg!(" - amount: {}", token_escrow_data.amount);

    // Done
    Ok(())
}
