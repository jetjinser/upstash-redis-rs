use upstash_redis_rs::{Command, Redis};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let url = env!("UPSTASH_REDIS_REST_URL");
    let token = env!("UPSTASH_REDIS_REST_TOKEN");

    let redis = Redis::new(url, token).unwrap();

    let v = redis.hgetall("author")?.send().await?;

    println!("{:#?}", v);

    Ok(())
}
