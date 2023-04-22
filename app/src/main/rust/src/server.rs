use std::sync::Arc;
use ndk::asset::AssetManager;
use rocket::catchers;
use rocket::data::{Limits, ToByteUnit};
use rocket::figment::providers::Toml;
use rocket::config::LogLevel;

#[tokio::main]
pub async fn run_server(srv: Server,ass: AssetManager) {
    let limits = Limits::default()
        .limit("json", 3.mebibytes())
        .limit("string", 3.mebibytes())
        .limit("data-form", 5.gibibytes())
        .limit("file", 5.gibibytes());
    let toml = Toml::string(r#"
    [default.databases]
    notes = { url = "/storage/emulated/0/notes.db", pool_size = 1 }
     "#).nested();
    let figment = Figment::from(rocket::Config::default())
        .merge((rocket::Config::ADDRESS, srv.host))
        .merge((rocket::Config::PORT, srv.port))
        .merge((rocket::Config::TEMP_DIR, srv.temp_dir))
        .merge((rocket::Config::LIMITS, limits))
        .merge((rocket::Config::LOG_LEVEL, LogLevel::Critical))
        .merge(toml);
    let mut server = rocket::custom(figment)
        .manage(Arc::new(Cache::new(ass)))
        .register("/", catchers![error::not_found]);
    ;
    server.launch().await;
}