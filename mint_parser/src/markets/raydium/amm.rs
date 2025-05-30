use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use solana_sdk::account::Account;

lazy_static! {
    pub static ref ID: Pubkey =
        Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap();
}

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    let coin_vault_mint_offset = 368 + 32; // exact field offset in AmmInfo struct
    let pc_vault_mint_offset = 400 + 32;   // exact field offset in AmmInfo struct

    if account.data.len() < pc_vault_mint_offset + 32 {
        return Ok(vec![]);
    }

    let coin_vault_mint_bytes = &account.data[coin_vault_mint_offset..coin_vault_mint_offset + 32];
    let pc_vault_mint_bytes = &account.data[pc_vault_mint_offset..pc_vault_mint_offset + 32];

    let coin_vault_mint = Pubkey::new_from_array(coin_vault_mint_bytes.try_into().unwrap());
    let pc_vault_mint = Pubkey::new_from_array(pc_vault_mint_bytes.try_into().unwrap());

    Ok(vec![coin_vault_mint, pc_vault_mint])
}
