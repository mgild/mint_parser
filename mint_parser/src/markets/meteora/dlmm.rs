use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;

lazy_static! {
    pub static ref ID: Pubkey =
        Pubkey::from_str("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo").unwrap();
}

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    let token_x_mint_offset = 8 + 73;
    let token_y_mint_offset = token_x_mint_offset + 32;

    if account.data.len() < token_y_mint_offset + 32 {
        return Ok(vec![]);
    }

    let token_x_mint_bytes = &account.data[token_x_mint_offset..token_x_mint_offset + 32];
    let token_y_mint_bytes = &account.data[token_y_mint_offset..token_y_mint_offset + 32];

    let token_x_mint = Pubkey::new_from_array(token_x_mint_bytes.try_into().unwrap());
    let token_y_mint = Pubkey::new_from_array(token_y_mint_bytes.try_into().unwrap());

    Ok(vec![token_x_mint, token_y_mint])
}

