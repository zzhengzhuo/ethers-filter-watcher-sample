use std::sync::Arc;

use ethers::{
    providers::{Http, Middleware, Provider, StreamExt},
    types::{Filter, H160, H256},
};

const USDC_ADDRESS: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";

#[tokio::main]
async fn main() {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:8545").unwrap();
    let client = Arc::new(provider);
    let token_topics = [H256::from(USDC_ADDRESS.parse::<H160>().unwrap())];
    let filter = Filter::new()
        .topic1(token_topics.to_vec())
        .topic2(token_topics.to_vec());

    let watcher = client.watch(&filter).await.unwrap();
    watcher
        .enumerate()
        .for_each(|(i, v)| async move {
            dbg!("i: {i}, log: {v}");
        })
        .await;
}
