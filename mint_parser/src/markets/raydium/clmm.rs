use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;

lazy_static! {
    pub static ref ID: Pubkey =
        Pubkey::from_str("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK").unwrap();
}

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    let token_mint_0_offset = 8 + 1 + 32 + 32; // 8 discriminator + bump + amm_config + pool_creator
    let token_mint_1_offset = token_mint_0_offset + 32; // + token_mint_0

    if account.data.len() < token_mint_1_offset + 32 {
        return Ok(vec![]);
    }

    let token_mint_0_bytes = &account.data[token_mint_0_offset..token_mint_0_offset + 32];
    let token_mint_1_bytes = &account.data[token_mint_1_offset..token_mint_1_offset + 32];

    let token_mint_0 = Pubkey::new_from_array(token_mint_0_bytes.try_into().unwrap());
    let token_mint_1 = Pubkey::new_from_array(token_mint_1_bytes.try_into().unwrap());

    Ok(vec![token_mint_0, token_mint_1])
}
