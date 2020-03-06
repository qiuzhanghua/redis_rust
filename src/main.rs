use r2d2_redis::redis::{geo, Commands, ConnectionLike};
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::f64;
use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let redis_url = dotenv::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(redis_url)?;
    let pool = r2d2::Pool::builder().max_size(4).build(manager)?;
    let mut conn = pool.get()?;
    conn.is_open();

    let s = conn.set_multiple::<&str, i32, bool>(&[("my_key", 99), ("my_key2", 111)])?;
    println!("{:?}", s);
    // let s = conn.set::<&str, i32, String>("my_key", 32)?;
    // println!("{:?}", s);  // s == "OK"
    // let v = conn.get::<&str, i32>("my_key")?;
    // println!("{:?}", v);
    println!("delete {} items.", conn.del::<&str, i32>("my_key")?);

    // let added: isize = conn.geo_add(
    //     "gis",
    //     &[
    //         (geo::Coord::lon_lat("13.361389", "38.115556"), "Palermo"),
    //         (geo::Coord::lon_lat("15.087269", "37.502669"), "Catania"),
    //         (geo::Coord::lon_lat("13.5833332", "37.316667"), "Agrigento"),
    //     ],
    // )?;
    // println!("[geo_add] Added {} members.", added);

    let position: Vec<geo::Coord<f64>> = conn.geo_pos("gis", "Palermo")?;
    println!("[geo_pos] Position for Palermo: {:?}", position);

    // Search members near (13.5, 37.75)

    let options = geo::RadiusOptions::default()
        .order(geo::RadiusOrder::Asc)
        .with_dist()
        .limit(2);
    let items: Vec<geo::RadiusSearchResult> =
        conn.geo_radius("gis", 13.5, 37.75, 150.0, geo::Unit::Kilometers, options)?;

    for item in items {
        println!(
            "[geo_radius] {}, dist = {} Km",
            item.name,
            item.dist.unwrap_or(f64::NAN)
        );
    }

    let (s, r) = unbounded();
    s.send("Hello, world!");

    assert_eq!(r.recv(), Ok("Hello, world!"));

    let d = ChannelDrop::R(r);

    let (sender_command, receiver_command) = unbounded();
    sender_command.send(d);

    let command = receiver_command.recv().unwrap();
    match command {
        ChannelDrop::R(r) => {
            drop(r);
        }
        ChannelDrop::S(s) => {
            drop(s)
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum ChannelDrop<T> {
    R(Receiver<T>),
    S(Sender<T>),
}

