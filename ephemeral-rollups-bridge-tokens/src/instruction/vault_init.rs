use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::system_program;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::escrow_token_vault_seeds_generator;
use crate::state::escrow_token::EscrowTokenAccount;
use crate::util::create::create_pda;
use crate::util::ensure::{ensure_is_owned_by_program, ensure_is_pda, ensure_is_signer};

pub const TAG: [u8; 8] = [0x70, 0xfe, 0x66, 0x40, 0x47, 0x49, 0x16, 0x0e];

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    // Read instruction inputs
    let [payer, mint, validator_id, escrow_token_vault_pda, token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Verify that the escrow PDA is currently un-initialized
    ensure_is_owned_by_program(escrow_token_vault_pda, &system_program::ID)?;

    // Verify the seeds of the escrow PDA
    let escrow_token_vault_seeds = escrow_token_vault_seeds_generator!(mint.key, validator_id.key);
    ensure_is_pda(escrow_token_vault_pda, escrow_token_vault_seeds, program_id)?;

    // Initialize the escrow PDA
    create_pda(
        payer,
        escrow_token_vault_pda,
        escrow_token_vault_seeds,
        EscrowTokenAccount::space(),
        program_id,
        system_program,
    )?;

    // Write the spl token vault's content
    invoke(
        &spl_token::instruction::initialize_account3(
            token_program.key,
            escrow_token_vault_pda.key,
            mint.key,
            escrow_token_vault_pda.key,
        )?,
        &[
            escrow_token_vault_pda.clone(),
            escrow_token_vault_pda.clone(),
        ],
    );

    // Done
    Ok(())
}
