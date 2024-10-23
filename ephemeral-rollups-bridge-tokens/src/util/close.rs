use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke_signed,
    system_instruction::{allocate, assign, transfer},
};

pub fn close_pda<'info>(
    pda: &AccountInfo<'info>,
    pda_seeds: &[&[u8]],
    spill: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
) -> ProgramResult {
    // Dealloc all data
    invoke_signed(
        &allocate(pda.key, 0),
        &[pda.clone(), system_program.clone()],
        &[pda_seeds],
    )?;
    // Siphon all lamports
    invoke_signed(
        &transfer(pda.key, spill.key, pda.lamports()),
        &[pda.clone(), spill.clone(), system_program.clone()],
        &[pda_seeds],
    )?;
    // Reassign to system program
    invoke_signed(
        &assign(pda.key, system_program.key),
        &[pda.clone(), system_program.clone()],
        &[pda_seeds],
    )?;
    // Done
    Ok(())
}
