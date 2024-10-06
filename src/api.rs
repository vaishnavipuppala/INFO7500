use rocket::{get, routes, State};
use sqlx::{Pool, Postgres, Row};
use serde::Serialize;
use rocket::serde::json::Json;

#[derive(Serialize)]
struct BlockMetrics {
    block_height: i32,
    market_price: f64,
}

#[get("/block_metrics")]
async fn get_block_metrics(pool: &State<Pool<Postgres>>) -> Option<Json<BlockMetrics>> {
    let result = sqlx::query(
        "SELECT block_height, market_price FROM metrics ORDER BY id DESC LIMIT 1"
    )
    .fetch_one(pool.inner())
    .await;

    match result {
        Ok(record) => {
            let block_height: i32 = match record.try_get("block_height") {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Error extracting block_height: {:?}", e);
                    return None;
                }
            };

            let market_price: f64 = match record.try_get("market_price") {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Error extracting market_price: {:?}", e);
                    return None;
                }
            };

            Some(Json(BlockMetrics {
                block_height,
                market_price,
            }))
        },
        Err(e) => {
            eprintln!("Error fetching block metrics: {:?}", e);
            None
        },
    }
}

pub fn start_server(pool: Pool<Postgres>) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(pool)
        .mount("/", routes![get_block_metrics])
}