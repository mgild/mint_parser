use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;

lazy_static! {
    pub static ref ID: Pubkey =
        Pubkey::from_str("SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe").unwrap();
}

const X_MINT_OFFSET: usize = 2664;
const Y_MINT_OFFSET: usize = 2696;

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    if account.data.len() < 2800 { // SolFi accounts are 2800 bytes
        return Ok(vec![]);
    }

    // Extract x_mint and y_mint from known offsets
    let x_mint_bytes = &account.data[X_MINT_OFFSET..X_MINT_OFFSET + 32];
    let y_mint_bytes = &account.data[Y_MINT_OFFSET..Y_MINT_OFFSET + 32];

    let x_mint = Pubkey::new_from_array(x_mint_bytes.try_into().unwrap());
    let y_mint = Pubkey::new_from_array(y_mint_bytes.try_into().unwrap());

    Ok(vec![x_mint, y_mint])
}
