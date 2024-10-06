use tokio::time::{sleep, Duration};
use sqlx::Pool;
use sqlx::Postgres;
use crate::bitcoin;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct PriceResponse {
    bitcoin: PriceData,
}

#[derive(Deserialize)]
struct PriceData {
    usd: f64,
}

async fn fetch_market_price() -> Result<f64, reqwest::Error> {
    let response = reqwest::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
        .await?
        .json::<PriceResponse>()
        .await?;
    
    Ok(response.bitcoin.usd)
}

pub async fn start_ingestion(pool: Pool<Postgres>) {
    let client = match bitcoin::get_client() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create Bitcoin RPC client: {:?}", e);
            return;
        }
    };

    loop {
        match bitcoin::fetch_block_height(&client) {
            Ok(block_height) => {
                if let Ok(market_price) = fetch_market_price().await {
                    if let Err(e) = crate::db::insert_metrics(&pool, block_height, market_price).await {
                        eprintln!("Failed to insert metrics: {:?}", e);
                    }
                } else {
                    eprintln!("Failed to fetch market price");
                }
            },
            Err(e) => eprintln!("Failed to fetch block height: {:?}", e),
        }

        sleep(Duration::from_secs(30)).await;
    }
}