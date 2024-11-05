use anchor_lang::prelude::AccountMeta;
use anchor_lang::AnchorSerialize;
use ephemeral_rollups_sdk::consts::{BUFFER, DELEGATION_PROGRAM_ID};
use ephemeral_rollups_sdk::pda::{
    delegation_metadata_pda_from_pubkey, delegation_record_pda_from_pubkey,
};
use ephemeral_rollups_sdk::types::DelegateAccountArgs;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;

use crate::api::program_context::process_instruction::process_instruction_with_signer;
use crate::api::program_context::program_context_trait::ProgramContext;
use crate::api::program_context::program_error::ProgramError;
use crate::api::program_spl::process_system_create::process_system_create;

pub async fn process_delegate_on_curve(
    program_context: &mut Box<dyn ProgramContext>,
    payer: &Keypair,
    account: &Keypair,
    lamports: u64,
) -> Result<(), ProgramError> {
    let rent_minimum_balance = program_context.get_rent_minimum_balance(0).await?;
    process_system_create(
        program_context,
        payer,
        account,
        rent_minimum_balance + lamports,
        0,
        &DELEGATION_PROGRAM_ID,
    )
    .await?;

    let pubkey = account.pubkey();
    let system_program_id = system_program::ID;

    let delegation_buffer_pda =
        Pubkey::find_program_address(&[BUFFER, &pubkey.to_bytes()], &system_program_id).0;

    let delegation_record_pda = delegation_record_pda_from_pubkey(&pubkey);
    let delegation_metadata_pda = delegation_metadata_pda_from_pubkey(&pubkey);

    let args = DelegateAccountArgs {
        valid_until: i64::MAX,
        commit_frequency_ms: u32::MAX,
        seeds: vec![],
    };

    let mut data = 0_u64.to_le_bytes().to_vec();
    data.extend_from_slice(&args.try_to_vec().unwrap());

    let instruction = Instruction {
        program_id: DELEGATION_PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(account.pubkey(), true),
            AccountMeta::new_readonly(system_program_id, false),
            AccountMeta::new(delegation_buffer_pda, false),
            AccountMeta::new(delegation_record_pda, false),
            AccountMeta::new(delegation_metadata_pda, false),
            AccountMeta::new_readonly(system_program_id, false),
        ],
        data,
    };

    process_instruction_with_signer(program_context, instruction, payer, account).await
}
