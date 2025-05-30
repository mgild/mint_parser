use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;
use bytemuck;

lazy_static! {
    pub static ref ID: Pubkey =
        Pubkey::from_str("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc").unwrap();
}

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    if account.data.len() < 653 {
        return Ok(vec![]);
    }

    let token_mint_a_bytes = &account.data[101..133];
    let token_mint_b_bytes = &account.data[181..213];

    let token_mint_a = Pubkey::new_from_array(token_mint_a_bytes.try_into().unwrap());
    let token_mint_b = Pubkey::new_from_array(token_mint_b_bytes.try_into().unwrap());

    Ok(vec![token_mint_a, token_mint_b])
}
