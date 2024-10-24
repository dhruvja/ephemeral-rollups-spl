use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

pub fn ensure_is_signer(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

pub fn ensure_is_owned_by_program(
    account: &AccountInfo,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    if account.owner.ne(program_id) {
        return Err(ProgramError::InvalidAccountOwner);
    }
    Ok(())
}

pub fn ensure_is_pda(
    account: &AccountInfo,
    seeds: &[&[u8]],
    program_id: &Pubkey,
) -> Result<u8, ProgramError> {
    let pda = Pubkey::find_program_address(seeds, program_id);
    if account.key.ne(&pda.0) {
        return Err(ProgramError::InvalidSeeds);
    }
    Ok(pda.1)
}
