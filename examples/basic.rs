use serde_json::Value;
use upstash_redis_rs::{Command, Redis};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let url = env!("UPSTASH_REDIS_REST_URL");
    let token = env!("UPSTASH_REDIS_REST_TOKEN");

    let redis = Redis::new(url, token).unwrap();

    // let sv: Value = redis.set("author", "kafka")?.get()?.send().await?;
    // let gv: Value = redis.get("author")?.send().await?;
    let dv: Value = redis.del("author")?.send().await?;

    // println!("set: {}", sv);
    // println!("get: {}", gv);
    println!("get: {}", dv);

    Ok(())
}
