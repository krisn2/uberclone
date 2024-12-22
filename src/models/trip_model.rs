use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trip{
    pub trip_id:Uuid,
    pub driver_id: Option<Uuid>,
    pub user_id:Uuid,
    pub pickup: String,
    pub destination: String,
    pub status: String,
    pub start_time:Option<String>,
    pub end_time:Option<String>,
}