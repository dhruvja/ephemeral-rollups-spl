use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;

pub async fn create_program_test_context() -> ProgramTestContext {
    let mut program_test = ProgramTest::default();

    program_test.prefer_bpf(true);

    program_test.add_program(
        "../target/deploy/ephemeral_rollups_bridge",
        ephemeral_rollups_bridge::id(),
        None,
    );

    program_test.start_with_context().await
}
