use ephemeral_rollups_sdk::consts::DELEGATION_PROGRAM_ID;
use solana_toolbox_endpoint::ToolboxEndpoint;

pub async fn create_localnet_toolbox_endpoint() -> ToolboxEndpoint {
    ToolboxEndpoint::new_program_test_with_preloaded_programs(&[
        (DELEGATION_PROGRAM_ID, "./binaries/dlp"),
        (mpl_bubblegum::ID, "./binaries/bubblegum"),
        (spl_noop::ID, "./binaries/noop"),
        (spl_account_compression::ID, "./binaries/compression"),
        (
            ephemeral_rollups_wrapper::ID,
            "../target/deploy/ephemeral_rollups_wrapper",
        ),
    ])
    .await
}
