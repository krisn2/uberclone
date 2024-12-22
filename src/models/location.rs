use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct Location {
    pub lat:f64,
    pub long:f64,
}