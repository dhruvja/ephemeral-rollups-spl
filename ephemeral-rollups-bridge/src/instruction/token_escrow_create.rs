use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::system_program;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::state::token_escrow::TokenEscrow;
use crate::token_escrow_seeds_generator;
use crate::util::create::create_pda;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda};

pub const DISCRIMINANT: [u8; 8] = [0xfe, 0x25, 0x5a, 0x94, 0x2e, 0x8e, 0x50, 0xac];

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Args {
    pub index: u64,
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [payer, authority, validator, token_mint, token_escrow_pda, system_program_id] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let args = Args::try_from_slice(data)?;

    // Verify that the escrow PDA is currently un-initialized
    ensure_is_owned_by_program(token_escrow_pda, &system_program::ID)?;

    // Verify the seeds of the escrow PDA
    let token_escrow_seeds =
        token_escrow_seeds_generator!(authority.key, validator.key, token_mint.key, args.index);
    ensure_is_pda(token_escrow_pda, token_escrow_seeds, program_id)?;

    // Initialize the escrow PDA
    create_pda(
        payer,
        token_escrow_pda,
        token_escrow_seeds,
        TokenEscrow::space(),
        program_id,
        system_program_id,
    )?;

    // Initialize the escrow data
    let token_escrow_data = TokenEscrow {
        discriminant: TokenEscrow::discriminant(),
        amount: 0,
    };
    token_escrow_data.serialize(&mut &mut token_escrow_pda.try_borrow_mut_data()?.as_mut())?;

    // Done
    Ok(())
}
