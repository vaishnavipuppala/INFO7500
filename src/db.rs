// use sqlx::{migrate::MigrateDatabase, Error, Pool, Postgres};

// pub async fn init_db(database_url: &str) -> Result<Pool<Postgres>, Error> {
//      if !Postgres::database_exists(database_url).await.unwrap_or(false) {
//         Postgres::create_database(database_url).await?;
//     } else {
//         println!("Database already exists");
//     }
//     let pool = Pool::<Postgres>::connect(database_url).await?;

//     create_tables(&pool).await?;

//     Ok(pool)
// }
use sqlx::{migrate::MigrateDatabase, Error, Pool, Postgres};

pub async fn init_db(database_url: &str) -> Result<Pool<Postgres>, Error> {
    let database_name = "bitcoin_explorer";
    let base_url = database_url.split('/').take(3).collect::<Vec<&str>>().join("/");
    let full_url = format!("{}/{}", base_url, database_name);

    if !Postgres::database_exists(&full_url).await.unwrap_or(false) {
        println!("Database does not exist. Creating...");
        match Postgres::create_database(&full_url).await {
            Ok(_) => println!("Database created successfully"),
            Err(e) => eprintln!("Error creating database: {:?}", e),
        }
    } else {
        println!("Database already exists");
    }

    println!("Connecting to database...");
    let pool = Pool::<Postgres>::connect(&full_url).await?;
    println!("Connected to database successfully");

    create_tables(&pool).await?;

    Ok(pool)
}

async fn create_tables(pool: &Pool<Postgres>) -> Result<(), Error> {
    match sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS metrics (
            id SERIAL PRIMARY KEY,
            block_height INTEGER NOT NULL,
            market_price FLOAT8,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(pool)
    .await {
        Ok(_) => println!("metrics table created or already exists."),
        Err(e) => eprintln!("Failed to create metrics table: {:?}", e),
    };

    Ok(())
}

pub async fn insert_metrics(pool: &Pool<Postgres>, block_height: i64, market_price: f64) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO metrics (block_height, market_price) VALUES ($1, $2)"
    )
    .bind(block_height)
    .bind(market_price)
    .execute(pool)
    .await?;
    Ok(())
}