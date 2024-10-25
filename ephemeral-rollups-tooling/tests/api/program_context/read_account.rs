use borsh::BorshDeserialize;
use solana_sdk::account::Account;
use solana_sdk::program_pack::IsInitialized;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

pub async fn read_account(
    program_context: &mut Box<dyn ProgramContext>,
    address: &Pubkey,
) -> Result<Option<Account>, ProgramError> {
    program_context.get_account(address).await
}

pub async fn read_account_exists(
    program_context: &mut Box<dyn ProgramContext>,
    address: &Pubkey,
) -> Result<bool, ProgramError> {
    Ok(program_context.get_account(address).await?.is_some())
}

pub async fn read_account_lamports(
    program_context: &mut Box<dyn ProgramContext>,
    address: &Pubkey,
) -> Result<u64, ProgramError> {
    Ok(program_context
        .get_account(address)
        .await?
        .map(|account| account.lamports)
        .unwrap_or(0))
}

pub async fn read_account_data(
    program_context: &mut Box<dyn ProgramContext>,
    address: &Pubkey,
) -> Result<Vec<u8>, ProgramError> {
    let raw_account = program_context
        .get_account(address)
        .await?
        .ok_or(ProgramError::Custom("AccountDoesNotExist"))?;
    Ok(raw_account.data)
}

pub async fn read_account_packed<T: Pack + IsInitialized>(
    program_context: &mut Box<dyn ProgramContext>,
    address: &Pubkey,
) -> Result<T, ProgramError> {
    let raw_account_data = read_account_data(program_context, address).await?;
    let raw_account_slice: &[u8] = &raw_account_data;
    T::unpack(raw_account_slice).map_err(ProgramError::Program)
}

pub async fn read_account_borsh<T: BorshDeserialize>(
    program_context: &mut Box<dyn ProgramContext>,
    address: &Pubkey,
) -> Result<T, ProgramError> {
    let raw_account_data = read_account_data(program_context, address).await?;
    let raw_account_slice: &[u8] = &raw_account_data;
    T::try_from_slice(raw_account_slice).map_err(ProgramError::Io)
}
