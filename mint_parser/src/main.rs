use anyhow::Result;
use anyhow::Context;
use solana_sdk::pubkey::Pubkey;
use serde::Deserialize;
use serde::Serialize;
use clap::Parser;
use std::str::FromStr;
use solana_client::nonblocking::rpc_client::RpcClient;

pub mod markets;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pubkey to look up market mints for
    #[arg(short, long)]
    pubkey: String,
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let pubkey = Pubkey::try_from(args.pubkey.as_str())?;

    let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let market_data = client.get_account(&pubkey).await
        .context("Failed to fetch account data for the provided pubkey")?;
    let mints = markets::parse_mints(&market_data).await
        .context("Failed to parse mints from market data")?;
    println!("Market data for pubkey: {:?}", mints);

    Ok(())
}
