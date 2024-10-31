use solana_program::{
    account_info::AccountInfo,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{allocate, assign, transfer},
    sysvar::Sysvar,
};

use crate::util::signer::signer_seeds;

pub fn create_pda<'info>(
    payer: &AccountInfo<'info>,
    pda: &AccountInfo<'info>,
    pda_seeds: &[&[u8]],
    pda_bump: u8,
    data_len: usize,
    owner: &Pubkey,
    system_program: &AccountInfo<'info>,
) -> Result<(), ProgramError> {
    // Generate the PDA's signer seeds
    let pda_bump_slice = &[pda_bump];
    let pda_signer_seeds = signer_seeds(pda_seeds, pda_bump_slice);
    // Transfer sufficient lamports for rent exemption
    let rent_exempt_missing_amount = Rent::get()?
        .minimum_balance(data_len)
        .saturating_sub(pda.lamports());
    if rent_exempt_missing_amount.gt(&0) {
        invoke(
            &transfer(payer.key, pda.key, rent_exempt_missing_amount),
            &[payer.clone(), pda.clone(), system_program.clone()],
        )?;
    }
    // Allocate enough space
    let space = u64::try_from(data_len).map_err(|_| ProgramError::ArithmeticOverflow)?;
    invoke_signed(
        &allocate(pda.key, space),
        &[pda.clone(), system_program.clone()],
        &[&pda_signer_seeds],
    )?;
    // Assign new program as the owner
    invoke_signed(
        &assign(pda.key, owner),
        &[pda.clone(), system_program.clone()],
        &[&pda_signer_seeds],
    )?;
    // Done
    Ok(())
}
