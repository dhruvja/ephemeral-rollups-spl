use solana_program::pubkey::Pubkey;

use crate::token_vault_seeds_generator;

pub fn token_vault_generate_pda(
    validator: &Pubkey,
    token_mint: &Pubkey,
    program_id: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        token_vault_seeds_generator!(validator, token_mint),
        program_id,
    )
    .0
}

pub const TOKEN_VAULT_SEEDS_PREFIX: &[u8] = b"token_vault";

#[macro_export]
macro_rules! token_vault_seeds_generator {
    ($validator: expr, $token_mint: expr) => {
        &[
            $crate::state::token_vault::TOKEN_VAULT_SEEDS_PREFIX,
            &$validator.to_bytes(),
            &$token_mint.to_bytes(),
        ]
    };
}
