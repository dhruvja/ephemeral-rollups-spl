#[derive(Debug)]
pub enum ProgramError {
    BanksClient(solana_program_test::BanksClientError),
    Client(solana_client::client_error::ClientError),
    Signature(String),
    Program(solana_sdk::program_error::ProgramError),
    Elapsed(tokio::time::error::Elapsed),
    Io(std::io::Error),
    Custom(&'static str),
}
