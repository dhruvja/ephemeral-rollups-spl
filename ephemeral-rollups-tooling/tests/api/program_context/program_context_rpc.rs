use std::time::{Duration, Instant};

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::sysvar::Sysvar;
use solana_sdk::transaction::Transaction;

use async_trait::async_trait;
use tokio::time::sleep;

use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;

#[async_trait]
impl ProgramContext for RpcClient {
    async fn get_latest_blockhash(&mut self) -> Result<Hash, ProgramError> {
        RpcClient::get_latest_blockhash(self)
            .await
            .map_err(ProgramError::Client)
    }

    async fn get_rent_minimum_balance(&mut self, space: usize) -> Result<u64, ProgramError> {
        self.get_minimum_balance_for_rent_exemption(space)
            .await
            .map_err(ProgramError::Client)
    }

    async fn get_clock(&mut self) -> Result<Clock, ProgramError> {
        Clock::get().map_err(ProgramError::Program)
    }

    async fn get_account(&mut self, address: &Pubkey) -> Result<Option<Account>, ProgramError> {
        let response = self
            .get_account_with_commitment(address, CommitmentConfig::processed())
            .await
            .map_err(ProgramError::Client)?;
        Ok(response.value)
    }

    async fn process_transaction(&mut self, transaction: Transaction) -> Result<(), ProgramError> {
        let signature = self
            .send_transaction(&transaction)
            .await
            .map_err(ProgramError::Client)?;
        println!("process_transaction signature: {:?}", signature);
        let start = Instant::now();
        loop {
            let confirmed = self
                .confirm_transaction(&signature)
                .await
                .map_err(ProgramError::Client)?;
            if confirmed {
                break;
            }
            let duration = start.elapsed();
            println!(
                "failed to confirm transaction (elapsed: {} ms)",
                duration.as_millis()
            );
            if duration > Duration::from_secs(10) {
                return Err(ProgramError::Custom("Timeout on awaiting confirmation"));
            }
            sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }

    async fn process_airdrop(&mut self, _to: &Pubkey, _lamports: u64) -> Result<(), ProgramError> {
        Err(ProgramError::Custom("Airdrop not supported"))
    }

    async fn move_clock_forward(
        &mut self,
        _unix_timestamp_delta: u64,
        _slot_delta: u64,
    ) -> Result<(), ProgramError> {
        Err(ProgramError::Custom("Clock forward not supported"))
    }
}
