use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::system_program;

pub fn close_pda<'a, 'info>(
    pda: &'a AccountInfo<'info>,
    spill: &'a AccountInfo<'info>,
) -> ProgramResult {
    **spill.lamports.borrow_mut() =
        spill.lamports().checked_add(pda.lamports()).unwrap();
    **pda.lamports.borrow_mut() = 0;
    pda.assign(&system_program::ID);
    pda.realloc(0, false).map_err(Into::into)
}
