use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::{msg, system_program};

use crate::lamport_escrow_seeds_generator;
use crate::state::lamport_escrow::LamportEscrow;
use crate::util::create::create_pda;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const DISCRIMINANT: [u8; 8] = [0x1a, 0x92, 0xb7, 0x8b, 0x57, 0xad, 0x99, 0x02];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub authority: Pubkey,
    pub validator: Pubkey,
    pub slot: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [payer, lamport_escrow_pda, system_program_id] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the payer is allowed to pay for the rent fees
    ensure_is_signer(payer)?;

    // Verify that the escrow PDA is currently un-initialized
    ensure_is_owned_by_program(lamport_escrow_pda, &system_program::ID)?;

    // Verify the seeds of the escrow PDA
    let lamport_escrow_seeds =
        lamport_escrow_seeds_generator!(args.authority, args.validator, args.slot);
    let lamport_escrow_bump = ensure_is_pda(lamport_escrow_pda, lamport_escrow_seeds, program_id)?;

    // Initialize the escrow PDA
    create_pda(
        payer,
        lamport_escrow_pda,
        lamport_escrow_seeds,
        lamport_escrow_bump,
        LamportEscrow::space(),
        program_id,
        system_program_id,
    )?;

    // Initialize the escrow data
    let lamport_escrow_data = LamportEscrow {
        discriminant: LamportEscrow::discriminant(),
    };
    lamport_escrow_data.serialize(&mut &mut lamport_escrow_pda.try_borrow_mut_data()?.as_mut())?;

    // Log outcome
    msg!("Ephemeral Rollups Wrap: Created a new LamportEscrow");
    msg!(" - authority: {} ({})", args.authority, args.slot);

    // Done
    Ok(())
}
