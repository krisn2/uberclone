use actix_web::{HttpRequest, HttpResponse, web};
use actix_ws::{handle, Session};
use futures_util::StreamExt;
use redis::aio::Connection;
use serde_json::Value;
use crate::models::{trip_model::Trip, location::Location};
use crate::services::redis_service::{update_driver_location, save_session};

pub async fn ride_ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    redis_conn: web::Data<Connection>,
) -> Result<HttpResponse, actix_web::Error> {
    let (res, mut session, stream) = handle(&req, stream)?;

    let mut stream = stream.map(|item| match item {
        Ok(msg) => Ok(msg.to_text().unwrap_or("").to_string()),
        Err(_) => Ok("".to_string()),
    });

    while let Some(Ok(text)) = stream.next().await {
        let msg: Value = serde_json::from_str(&text).unwrap_or_default();

        match msg["type"].as_str() {
            Some("update_location") => {
                let location = serde_json::from_value::<Location>(msg["data"].clone()).unwrap();
                let driver_id = msg["driver_id"].as_str().unwrap();

                update_driver_location(&mut redis_conn.get_ref().clone(), driver_id, location).await.unwrap();
                session.text("Location updated").await?;
            }
            Some("request_ride") => {
                let trip = serde_json::from_value::<Trip>(msg["data"].clone()).unwrap();
                save_session(&mut redis_conn.get_ref().clone(), &trip.trip_id.to_string(), &text).await.unwrap();
                session.text("Ride requested").await?;
            }
            _ => session.text("Unknown command").await?,
        }
    }

    Ok(res)
}
