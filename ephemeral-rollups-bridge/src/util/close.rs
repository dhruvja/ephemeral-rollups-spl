use solana_program::{
    account_info::AccountInfo,
    program::invoke_signed,
    program_error::ProgramError,
    system_instruction::{allocate, assign, transfer},
};

use crate::util::signer::signer_seeds;

pub fn close_pda<'info>(
    pda: &AccountInfo<'info>,
    pda_seeds: &[&[u8]],
    pda_bump: u8,
    spill: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
) -> Result<(), ProgramError> {
    // Generate the PDA's signer seeds
    let pda_bump_slice = &[pda_bump];
    let pda_signer_seeds = signer_seeds(pda_seeds, pda_bump_slice);
    // Dealloc all data
    invoke_signed(
        &allocate(pda.key, 0),
        &[pda.clone(), system_program.clone()],
        &[&pda_signer_seeds],
    )?;
    // Siphon all lamports
    invoke_signed(
        &transfer(pda.key, spill.key, pda.lamports()),
        &[pda.clone(), spill.clone(), system_program.clone()],
        &[&pda_signer_seeds],
    )?;
    // Reassign to system program
    invoke_signed(
        &assign(pda.key, system_program.key),
        &[pda.clone(), system_program.clone()],
        &[&pda_signer_seeds],
    )?;
    // Done
    Ok(())
}
