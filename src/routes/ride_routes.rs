use actix_web::web;
use crate::controllers::ride_controller::request_ride;
use crate::ws_handler::ride_ws_handler::ride_ws_handler;

pub fn ride_routes(
    cfg: &mut web::ServiceConfig
) {
    cfg
        .service(web::resource("/api/ride-req").route(web::post().to(request_ride)))
        .service(web::resource("/api/rider-ws").route(web::get().to(ride_ws_handler)));
}