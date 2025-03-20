use ephemeral_rollups_sdk::consts::EXTERNAL_UNDELEGATE_DISCRIMINATOR;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::processor::apply_undelegation;
// use crate::processor::bubblegum_escrow_delegate;
// use crate::processor::bubblegum_escrow_deposit;
// use crate::processor::bubblegum_escrow_transfer;
// use crate::processor::bubblegum_escrow_undelegate;
// use crate::processor::bubblegum_escrow_withdraw;
use crate::processor::lamport_escrow_claim;
use crate::processor::lamport_escrow_create;
use crate::processor::lamport_escrow_delegate;
use crate::processor::lamport_escrow_undelegate;
use crate::processor::token_escrow_create;
use crate::processor::token_escrow_delegate;
use crate::processor::token_escrow_deposit;
use crate::processor::token_escrow_transfer;
use crate::processor::token_escrow_undelegate;
use crate::processor::token_escrow_withdraw;
use crate::processor::token_vault_init;

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::ID) {
        return Err(ProgramError::IncorrectProgramId);
    }

    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (tag, data) = data.split_at(8);
    let tag_array: [u8; 8] =
        tag.try_into().map_err(|_| ProgramError::InvalidInstructionData)?;

    match tag_array {
        // bubblegum_escrow_deposit::DISCRIMINANT => {
        //     bubblegum_escrow_deposit::process(program_id, accounts, data)
        // },
        // bubblegum_escrow_delegate::DISCRIMINANT => {
        //     bubblegum_escrow_delegate::process(program_id, accounts, data)
        // },
        // bubblegum_escrow_transfer::DISCRIMINANT => {
        //     bubblegum_escrow_transfer::process(program_id, accounts, data)
        // },
        // bubblegum_escrow_undelegate::DISCRIMINANT => {
        //     bubblegum_escrow_undelegate::process(program_id, accounts, data)
        // },
        // bubblegum_escrow_withdraw::DISCRIMINANT => {
        //     bubblegum_escrow_withdraw::process(program_id, accounts, data)
        // },
        lamport_escrow_create::DISCRIMINANT => {
            lamport_escrow_create::process(program_id, accounts, data)
        },
        lamport_escrow_delegate::DISCRIMINANT => {
            lamport_escrow_delegate::process(program_id, accounts, data)
        },
        lamport_escrow_claim::DISCRIMINANT => {
            lamport_escrow_claim::process(program_id, accounts, data)
        },
        lamport_escrow_undelegate::DISCRIMINANT => {
            lamport_escrow_undelegate::process(program_id, accounts, data)
        },
        token_escrow_create::DISCRIMINANT => {
            token_escrow_create::process(program_id, accounts, data)
        },
        token_escrow_delegate::DISCRIMINANT => {
            token_escrow_delegate::process(program_id, accounts, data)
        },
        token_escrow_deposit::DISCRIMINANT => {
            token_escrow_deposit::process(program_id, accounts, data)
        },
        token_escrow_transfer::DISCRIMINANT => {
            token_escrow_transfer::process(program_id, accounts, data)
        },
        token_escrow_undelegate::DISCRIMINANT => {
            token_escrow_undelegate::process(program_id, accounts, data)
        },
        token_escrow_withdraw::DISCRIMINANT => {
            token_escrow_withdraw::process(program_id, accounts, data)
        },
        token_vault_init::DISCRIMINANT => {
            token_vault_init::process(program_id, accounts, data)
        },
        EXTERNAL_UNDELEGATE_DISCRIMINATOR => {
            apply_undelegation::process(program_id, accounts, data)
        },
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);
