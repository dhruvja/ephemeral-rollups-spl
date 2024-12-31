use borsh::BorshSerialize;
use ephemeral_rollups_sdk::consts::BUFFER;
use ephemeral_rollups_sdk::consts::DELEGATION_PROGRAM_ID;
use ephemeral_rollups_sdk::pda::delegation_metadata_pda_from_pubkey;
use ephemeral_rollups_sdk::pda::delegation_record_pda_from_pubkey;
use ephemeral_rollups_sdk::types::DelegateAccountArgs;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointError;

pub async fn process_delegate_on_curve(
    toolbox_endpoint: &mut ToolboxEndpoint,
    payer: &Keypair,
    account: &Keypair,
    lamports: u64,
) -> Result<(), ToolboxEndpointError> {
    let rent_minimum_balance =
        toolbox_endpoint.get_sysvar_rent().await?.minimum_balance(0);
    toolbox_endpoint
        .process_system_create(
            payer,
            account,
            rent_minimum_balance + lamports,
            0,
            &DELEGATION_PROGRAM_ID,
        )
        .await?;

    let pubkey = account.pubkey();
    let system_program_id = system_program::ID;

    let delegation_buffer_pda = Pubkey::find_program_address(
        &[BUFFER, &pubkey.to_bytes()],
        &system_program_id,
    )
    .0;

    let delegation_record_pda = delegation_record_pda_from_pubkey(&pubkey);
    let delegation_metadata_pda = delegation_metadata_pda_from_pubkey(&pubkey);

    let args = DelegateAccountArgs {
        valid_until: i64::MAX,
        commit_frequency_ms: u32::MAX,
        seeds: vec![],
    };

    let mut data = 0_u64.to_le_bytes().to_vec();
    args.serialize(&mut data).unwrap();

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

    toolbox_endpoint
        .process_instruction_with_signers(instruction, payer, &[account])
        .await?;

    Ok(())
}
