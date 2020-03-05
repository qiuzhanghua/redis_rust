use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let redis_url = dotenv::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(redis_url)?;
    let pool = r2d2::Pool::builder().max_size(4).build(manager)?;
    let mut conn = pool.get()?;
    let s = conn.set::<&str, i32, String>("my_key", 32)?;
    println!("{:?}", s);  // s == "OK"
    let v = conn.get::<&str, i32>("my_key")?;
    println!("{:?}", v);
    Ok(())
}
