use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::Transaction;

use async_trait::async_trait;

use crate::api::program_context::program_error::ProgramError;

#[async_trait]
pub trait ProgramContext {
    async fn get_latest_blockhash(&mut self) -> Result<Hash, ProgramError>;

    async fn get_rent_minimum_balance(&mut self, space: usize) -> Result<u64, ProgramError>;

    async fn get_clock(&mut self) -> Result<Clock, ProgramError>;

    async fn get_account(&mut self, address: &Pubkey) -> Result<Option<Account>, ProgramError>;

    async fn process_transaction(&mut self, transaction: Transaction) -> Result<(), ProgramError>;

    async fn process_airdrop(&mut self, to: &Pubkey, lamports: u64) -> Result<(), ProgramError>;

    async fn move_clock_forward(
        &mut self,
        unix_timestamp_delta: u64,
        slot_delta: u64,
    ) -> Result<(), ProgramError>;
}
