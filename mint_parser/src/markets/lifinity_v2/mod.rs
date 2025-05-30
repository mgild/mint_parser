use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;

lazy_static! {
    pub static ref ID: Pubkey =
        Pubkey::from_str("2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c").unwrap();
}

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    // Based on IDL struct layout: Lifinity V2 uses 8-byte discriminator + Amm struct
    // tokenAMint is at offset 229, tokenBMint is at offset 261 within the account data
    
    let token_a_mint_offset = 229; // 8 bytes discriminator + struct fields up to tokenAMint
    let token_b_mint_offset = 261; // token_a_mint_offset + 32 bytes
    
    if account.data.len() < token_b_mint_offset + 32 {
        return Ok(vec![]);
    }
    
    let token_a_mint_bytes = &account.data[token_a_mint_offset..token_a_mint_offset + 32];
    let token_b_mint_bytes = &account.data[token_b_mint_offset..token_b_mint_offset + 32];
    
    let token_a_mint = Pubkey::new_from_array(token_a_mint_bytes.try_into().unwrap());
    let token_b_mint = Pubkey::new_from_array(token_b_mint_bytes.try_into().unwrap());
    
    Ok(vec![token_a_mint, token_b_mint])
}
