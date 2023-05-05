use serde_json::Value;
use upstash_redis_rs::{commands::set::Expire, Command, Redis};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let url = env!("UPSTASH_REDIS_REST_URL");
    let token = env!("UPSTASH_REDIS_REST_TOKEN");

    let redis = Redis::new(url, token).unwrap();

    let sv: Value = redis
        .set("author", "kafka")?
        .get()?
        .expire(Expire::EX(30))?
        .send()
        .await?;
    // let gv: Value = redis.get("xx")?.send().await?;

    println!("set: {}", sv);
    // println!("get: {}", gv);

    Ok(())
}
