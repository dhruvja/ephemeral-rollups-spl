use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::{
    escrow_claim, escrow_close, escrow_create, escrow_delegate, escrow_undelegate,
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
        escrow_create::TAG => escrow_create::process(program_id, accounts, data),
        escrow_delegate::TAG => escrow_delegate::process(program_id, accounts, data),
        escrow_claim::TAG => escrow_claim::process(program_id, accounts, data),
        escrow_undelegate::TAG => escrow_undelegate::process(program_id, accounts, data),
        escrow_close::TAG => escrow_close::process(program_id, accounts, data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);
