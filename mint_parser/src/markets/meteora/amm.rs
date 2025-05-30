use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;
use anchor_lang::AccountDeserialize;

lazy_static! {
    pub static ref ID: Pubkey = Pubkey::from_str("Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB").unwrap();
}

// Use the actual Pool struct from dynamic_amm
// This matches the trader implementation
use dynamic_amm::state::Pool;

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    let pool = Pool::try_deserialize(&mut &account.data[..])?;
    Ok(vec![pool.token_a_mint, pool.token_b_mint])
}


