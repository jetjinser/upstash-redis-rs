use upstash_redis_rs::{Command, Redis};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let url = env!("UPSTASH_REDIS_REST_URL");
    let token = env!("UPSTASH_REDIS_REST_TOKEN");

    let redis = Redis::new(url, token).unwrap();

    // let sv = redis.set("author", "kafka")?.get()?.send().await?;
    let gv = redis.get("author")?.send().await?;
    // let dv = redis.del("author")?.send().await?;

    // println!("{:#?}", sv);
    println!("{:#?}", gv);
    // println!("{:#?}", dv);

    Ok(())
}
