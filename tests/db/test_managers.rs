use scraper_test::db::managers::BaseCompanyManager;
use scraper_test::db::managers::Company;
use scraper_test::db::managers::CompanyManager;
use sqlx::migrate;
use sqlx::pool::PoolOptions;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::Pool;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use std::process::Command;

// use scraper_test::db::managers::ForvalterManager;
// use scraper_test::db::managers::Forvalter;

// const DB_URL: &str = "sqlite::memory:";
const DB_URL: &str = "sqlite://db.sqlite";
const FILENAME: &str = "db.sqlite";
// const FILENAME: &str = ":memory:";

async fn setup() -> SqlitePool {
   

    // Run migrations
//    let result =  Command::new("sqlx")
//     .args(&["database", "create", "--database-url", DB_URL])
//     .output()
//     .expect("Failed to run migrations");

//     println!("Result: {:?}", result);

//     let result = Command::new("sqlx")
//         .args(&["migrate", "run", "--database-url", DB_URL])
//         .output()
//         .expect("Failed to run migrations");

//     println!("Result: {:?}", result);

    let options = SqliteConnectOptions::new().filename(FILENAME).create_if_missing(false);
    let pool = SqlitePool::connect_with(options).await.unwrap();
    // migrate!("migrations").run(&pool).await.unwrap();

    pool
}

#[tokio::test]
async fn test_verify() {
    let pool = setup().await;

    let manager = CompanyManager::new(pool);
    let company = Company { id: 0, name: "ACME Concrete".into() };
    let created = manager.create(company).await.unwrap();

    assert !(created.id == 0);
    assert !(created.name == "ACME Concrete".to_string());

    let retrieved = manager.get(created.id).await.unwrap();
    assert_eq!(created, retrieved);


    let deleted = manager.delete(retrieved.id).await.unwrap();
    assert!(deleted);
}

// #[tokio::test]
// async fn test_verify(){
//     let pool : Pool<Sqlite> = PoolOptions::new().max_connections(5).connect(DB_URL).await.unwrap();

//     let manager = ForvalterManager::new(pool);
//     let forvalter = Forvalter {id:0,name:"Anders Tandberg-Johansen".into(),added:Utc::now(), company: "ACME Concrete".into() };
//     let created = manager.create(forvalter).await.unwrap();
// }