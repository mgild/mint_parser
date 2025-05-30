use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;
use borsh::BorshDeserialize;

lazy_static! {
    pub static ref ID: Pubkey =
        Pubkey::from_str("obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y").unwrap();
}

#[derive(BorshDeserialize)]
struct SSTradingPair {
    discriminator: [u8; 8],
    // Add other fields as needed, but we need mint_x and mint_y
    mint_x: Pubkey,
    mint_y: Pubkey,
    // ... other fields would be here
}

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    let mint_x_offset = 8 + 194; // 8 bytes discriminator + exact field offset
    let mint_y_offset = 8 + 226; // 8 bytes discriminator + exact field offset

    if account.data.len() < mint_y_offset + 32 {
        return Ok(vec![]);
    }

    let mint_x_bytes = &account.data[mint_x_offset..mint_x_offset + 32];
    let mint_y_bytes = &account.data[mint_y_offset..mint_y_offset + 32];

    let mint_x = Pubkey::new_from_array(mint_x_bytes.try_into().unwrap());
    let mint_y = Pubkey::new_from_array(mint_y_bytes.try_into().unwrap());

    Ok(vec![mint_x, mint_y])
}
