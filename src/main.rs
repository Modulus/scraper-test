pub mod collectors;


use collectors::dnb_teknologi::DnbTeknologiCollector;
use collectors::base::StringDataCollector;
use sqlx::pool::PoolOptions;
use sqlx::{Pool, Sqlite};

const DB_URL: &str = "sqlite://db.sqlite";


#[tokio::main]
async fn main() {
    let collector = DnbTeknologiCollector::new();
    let forvaltere = collector.collect().await.unwrap();
    println!("{:?}", forvaltere);   

    let pool : Pool<Sqlite> = PoolOptions::new().max_connections(5).connect(DB_URL).await.unwrap();

}
