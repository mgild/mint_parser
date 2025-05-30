pub mod lifinity_v2;
pub mod meteora;
pub mod obric;
pub mod orca;
pub mod phoenix;
pub mod pump_amm;
pub mod raydium;
pub mod solfi;
pub mod stabble;
use crate::Pubkey;

use anyhow::Result;
use solana_sdk::account::Account;
use solana_client::nonblocking::rpc_client::RpcClient;

pub async fn parse_mints(account: &Account) -> Result<Vec<Pubkey>> {
    if account.owner == *lifinity_v2::ID {
        return lifinity_v2::parse_mints(account).await;
    }
    if account.owner == *meteora::dlmm::ID {
        return meteora::dlmm::parse_mints(account).await;
    }
    if account.owner == *meteora::amm::ID {
        return meteora::amm::parse_mints(account).await;
    }
    if account.owner == *phoenix::ID {
        return phoenix::parse_mints(account).await;
    }
    if account.owner == *pump_amm::ID {
        return pump_amm::parse_mints(account).await;
    }
    if account.owner == *raydium::amm::ID {
        return raydium::amm::parse_mints(account).await;
    }
    if account.owner == *raydium::clmm::ID {
        return raydium::clmm::parse_mints(account).await;
    }
    if account.owner == *orca::ID {
        return orca::parse_mints(account).await;
    }
    if account.owner == *obric::ID {
        return obric::parse_mints(account).await;
    }
    if account.owner == *stabble::stable::ID {
        return stabble::stable::parse_mints(account).await;
    }
    if account.owner == *stabble::weighted::ID {
        return stabble::weighted::parse_mints(account).await;
    }
    if account.owner == *solfi::ID {
        return solfi::parse_mints(account).await;
    }
    Ok(vec![])
}
