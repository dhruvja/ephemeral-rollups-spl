use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::processor::token_vault_init;
use crate::state::token_vault::token_vault_generate_pda;

pub fn instruction(
    payer: &Pubkey,
    validator: &Pubkey,
    token_mint: &Pubkey,
) -> Instruction {
    let program_id = crate::ID;
    let token_vault_pda =
        token_vault_generate_pda(validator, token_mint, &program_id);

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*validator, false),
        AccountMeta::new_readonly(*token_mint, false),
        AccountMeta::new(token_vault_pda, false),
        AccountMeta::new_readonly(spl_token::ID, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let mut data = Vec::new();
    data.extend_from_slice(&token_vault_init::DISCRIMINANT);

    Instruction { program_id, accounts, data }
}
