use redis::AsyncCommands;
use crate::models::location::Location;

pub async fn svae_session(
    redis_conn: &mut redis::aio::Connection,
    key: &str,
    value:&str,
)-> Result <(), redis::RedisError> {
    redis_conn.set(key, value).await?;
    Ok(())
}

pub async fn update_driver_location(
    redis_conn: &mut redis::aio::Connection,
    location: &Location,
    driver_id: &str,
) -> Result<(), redis::RedisError> {
    // let geo_key = format!("geo:{}", driver_id);
    // let location_str = format!("{} {}", location.latitude, location.longitude);
    // redis_conn.set(geo_key, location_str).await?;
    // Ok(())

    let geo_key = "driver_locations";
    redis_conn
        .geo_add(geo_key, (location.long, location.lat, driver_id))
        .await
}


pub async fn get_nearby_drivers(
    redis_conn: &mut redis::aio::Connection,
    lat: f64,
    long: f64,
    radius: f64,
)-> Result<Vec<String>, redis::RedisError> {
    let geo_key = "driver_locations";
    redis_conn
        .geo_radius(geo_key, long, lat, radius, redis::cmd::geo::GeoRadiusOptions::default())
        .await
}