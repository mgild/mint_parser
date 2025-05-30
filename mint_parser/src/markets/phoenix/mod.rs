use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;
use std::mem::size_of;

lazy_static! {
    pub static ref ID: Pubkey =
        Pubkey::from_str("PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY").unwrap();
}

#[repr(C)]
#[derive(Copy, Clone)]
struct MarketHeader {
    discriminant: [u8; 8],
    status: u64,
    market_size_params: u64,
    base_params: BaseParams,
    base_lot_size: u64,
    quote_lot_size: u64,
    tick_size_in_quote_lots_per_base_unit: u64,
    num_base_lots_per_base_unit: u64,
    base_mint: Pubkey,
    quote_mint: Pubkey,
    // ... other fields
}

#[repr(C)]
#[derive(Copy, Clone)]
struct BaseParams {
    base_decimals: u32,
    quote_decimals: u32,
}

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    let base_mint_offset = 48;  // discriminant(8) + status(8) + market_size_params(24) + base_params.decimals(4) + base_params.vault_bump(4)
    let quote_mint_offset = 128; // base_mint_offset + base_params(72) + base_lot_size(8) + quote_params.decimals(4) + quote_params.vault_bump(4)

    if account.data.len() < quote_mint_offset + 32 {
        return Ok(vec![]);
    }

    let base_mint_bytes = &account.data[base_mint_offset..base_mint_offset + 32];
    let quote_mint_bytes = &account.data[quote_mint_offset..quote_mint_offset + 32];

    let base_mint = Pubkey::new_from_array(base_mint_bytes.try_into().unwrap());
    let quote_mint = Pubkey::new_from_array(quote_mint_bytes.try_into().unwrap());

    Ok(vec![base_mint, quote_mint])
}
