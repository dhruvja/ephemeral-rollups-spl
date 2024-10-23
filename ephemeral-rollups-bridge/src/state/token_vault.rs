pub const TOKEN_VAULT_SEEDS_PREFIX: &[u8] = b"token_vault";

#[macro_export]
macro_rules! token_vault_seeds_generator {
    ($validator: expr, $token_mint: expr) => {
        &[
            crate::state::token_vault::TOKEN_VAULT_SEEDS_PREFIX,
            &$validator.to_bytes(),
            &$token_mint.to_bytes(),
        ]
    };
}
