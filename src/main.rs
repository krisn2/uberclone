use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, rt};
use actix_ws::{handle, AggregatedMessage, ProtocolError, Session};
use futures_util::StreamExt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

struct Location {
    lat :f64,
    long: f64,
}

type Users = Arc<Mutex<HashMap<Uuid, Arc<Mutex<Session>>>>>;
type CurrentLocation = Arc<Mutex<HashMap<Uuid>, Arc<Mutex<Location>>>>;
struct AppState {
    users: Users,
    locations : CurrentLocation,

}


async fn update_location_ws(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let (res, session, stream) = handle(&req, stream)?;
    let user_id = Uuid::new_v4();
    let session = Arc::new(Mutex::new(session));
    data.users.lock().unwrap().insert(user_id, session.clone());

    session
        .lock()
        .unwrap()
        .text(format!("Your id is : {} ", user_id))
        .await
        .unwrap();

    let stream = Box::pin(stream.aggregate_continuations().max_continuation_size(2_usize.pow(20)));

    let data_clone = data.clone();
    rt::spawn(handle_update_location(user_id, stream, data_clone));

    Ok(res)
}

async fn handle_update_location(
    user_id: Uuid,
    mut stream: impl StreamExt<Item = Result<AggregatedMessage, ProtocolError>> + Unpin,
    data: web::Data<AppState>,
) {
    while let Some(location) = stream.next().await {
        match location {
            Ok(AggregatedMessage::Text(text)) => {
                if let Some((driver_id, message)) = parse_location(&text).await {
                    let clients = data.users.lock().unwrap();
                    if let Some(driver_session) = clients.get(&driver_id) {
                        if let Err(e) = driver_session.lock().unwrap().text(message).await {
                            eprintln!("Failed to send message: {}", e);
                        }
                    } else {
                        eprintln!("Driver not found: {}", driver_id);
                    }
                } else {
                    eprintln!("Failed to parse message: {}", text);
                }
            }
            Ok(AggregatedMessage::Close(_)) => {
                data.users.lock().unwrap().remove(&user_id);
                break;
            }
            _ => {}
        }
    }
}
async fn parse_location(input: &str) -> Option<(Uuid, String)> {
    let parts: Vec<&str> = input.split(", location:").collect();

    if parts.len() == 2 {
        let driver_id_part = parts[0].trim_start_matches("to:").trim();
        let location_data = parts[1].trim();

        if let Ok(driver_id) = Uuid::parse_str(driver_id_part) {
            return Some((driver_id, location_data.to_string()));
        }
    }
    None
}

async fn update_driver_location(location:Location, appstate:AppState, driver: Uuid){
    let mut locations = appstate.locations.lock().unwrap();
    locations.insert(driver,location);
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let users: Users = Arc::new(Mutex::new(HashMap::new()));
    let data = web::Data::new(AppState { users });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("api/live-location", web::get().to(update_location_ws))
            .route("api/trip", web::get().to())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
