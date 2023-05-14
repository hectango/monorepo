use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub port: u16,
    pub cloudflare_api_key: String,
    pub cloudflare_account_id: String,
    pub db_url: String,
}
