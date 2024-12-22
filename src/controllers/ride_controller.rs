use actix_web::{HttpRequest, HttpResponse, web};
use redis::aio::Connection;
use uuid::Uuid;
use crate::models::trip_model::Trip;
use crate::services::redis_service::get_nearby_drivers;

pub async fn request_ride(
    req: web::Json<Trip>,
    redis_conn:web::Data<Connection>,
) -> HttpResponse {
    let trip = req.into_inner();

    let nearby_drivers = get_nearby_drivers(
        &mut redis_conn.get_ref().clone(),
        37.773972, // Example pickup latitude
        -122.431297, // Example pickup longitude
        5.0, // Radius in kilometers
    )
    .await;

    match nearby_drivers {
        Ok(drivers) => {
            if drivers.is_empty() {
                return HttpResponse::NotFound().finish();
            } 
            HttpResponse::Ok().json(drivers)
        }

        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}