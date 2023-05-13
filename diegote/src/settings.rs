use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub port: u16,
}
