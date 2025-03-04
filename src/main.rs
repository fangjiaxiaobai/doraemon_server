mod biz;
mod db;
use crate::biz::vod::VodQuery;
use crate::db::{config::DatabaseConfig, pool::DbPool};
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let f = test_create_vod();
    f.await;
}


async fn test_create_vod() {
    let pool = test_pool().await;

    // let handle:Vec<_> = (0..1)
    //     .map(|i| {
    //         let pool = pool.clone();
    //         tokio::spawn(
    //             async move { VodQuery::paginate(&pool,0 , 10).await },
    //         )
    //     })
    //     .collect();
    let result = VodQuery::paginate(&pool,1 , 10).await;

    // let results = futures::future::join_all(handle).await;

    println!("results: {:?}", result);
}

async fn test_pool() -> DbPool {
    let config = DatabaseConfig::default();
    let (url, max, min, timeout) = config.to_pool_config();
    DbPool::new(&url, max, min, timeout)
        .await
        .expect("Failed to create pool")
}
