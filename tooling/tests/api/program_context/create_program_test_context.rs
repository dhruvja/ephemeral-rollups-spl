use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;

pub async fn create_program_test_context() -> ProgramTestContext {
    // Program test struct will need to be aware of all its programs it will use
    let mut program_test = ProgramTest::default();

    // For some reason we need to set this flag to true in order for the binaries files to be loaded as programs
    program_test.prefer_bpf(true);

    /*
    program_test.add_program(
        "./tooling/tests/api/program_context/binaries/",
        system_program::id(),
        None,
    );
     */

    // Done, generate the ProgramTestContext
    program_test.start_with_context().await
}
