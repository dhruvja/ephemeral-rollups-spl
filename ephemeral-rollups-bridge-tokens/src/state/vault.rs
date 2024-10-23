pub const VAULT_SEEDS_PREFIX: &[u8] = b"vault";

#[macro_export]
macro_rules! escrow_token_vault_seeds_generator {
    ($mint: expr, $validator_id: expr) => {
        &[
            crate::state::escrow_token::VAULT_SEEDS_PREFIX,
            &$mint.to_bytes(),
            &$validator_id.to_bytes(),
        ]
    };
}
