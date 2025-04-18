use serde::Deserialize;


#[derive(Deserialize)]
pub struct BusConfig {
    pub socket_path: String,
}