use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;
use bytemuck;

lazy_static! {
    pub static ref ID: Pubkey =
        Pubkey::from_str("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA").unwrap();
}

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    let base_mint_offset = 43; // 8 + 1 + 2 + 32 (discriminator + pool_bump + index + creator)
    let quote_mint_offset = 75; // base_mint_offset + 32

    let base_mint_bytes = &account.data[base_mint_offset..base_mint_offset + 32];
    let quote_mint_bytes = &account.data[quote_mint_offset..quote_mint_offset + 32];

    let base_mint = Pubkey::new_from_array(base_mint_bytes.try_into().unwrap());
    let quote_mint = Pubkey::new_from_array(quote_mint_bytes.try_into().unwrap());

    Ok(vec![base_mint, quote_mint])
}
