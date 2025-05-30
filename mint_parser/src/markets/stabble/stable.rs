use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;
use anchor_lang::AccountDeserialize;

lazy_static! {
    pub static ref ID: Pubkey = Pubkey::from_str("swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ").unwrap();
}

// Use the actual Pool struct from stabble_stable_swap
// This matches the trader implementation
use stabble_stable_swap::pool::Pool;

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    let pool = Pool::try_deserialize(&mut &account.data[..])?;

    // Extract token mints from the pool tokens - matches trader implementation
    let mints: Vec<Pubkey> = pool.tokens.iter().map(|t| t.mint).collect();
    Ok(mints)
}
