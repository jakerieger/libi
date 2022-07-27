use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LibiConfig {
    pub compiler: String,
}