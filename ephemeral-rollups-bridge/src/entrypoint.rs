use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::{
    escrow_lamports_claim, escrow_lamports_close, escrow_lamports_create, escrow_lamports_delegate,
    escrow_lamports_undelegate,
};

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (tag, data) = data.split_at(8);
    let tag_array: [u8; 8] = tag
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match tag_array {
        escrow_lamports_create::DISCRIMINANT => {
            escrow_lamports_create::process(program_id, accounts, data)
        }
        escrow_lamports_delegate::DISCRIMINANT => {
            escrow_lamports_delegate::process(program_id, accounts, data)
        }
        escrow_lamports_claim::DISCRIMINANT => {
            escrow_lamports_claim::process(program_id, accounts, data)
        }
        escrow_lamports_undelegate::DISCRIMINANT => {
            escrow_lamports_undelegate::process(program_id, accounts, data)
        }
        escrow_lamports_close::DISCRIMINANT => {
            escrow_lamports_close::process(program_id, accounts, data)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);
