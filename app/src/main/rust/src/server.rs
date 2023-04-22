use std::sync::Arc;
use ndk::asset::AssetManager;
use rocket::{catchers, routes};
use rocket::data::{Limits, ToByteUnit};
use rocket::config::LogLevel;
use crate::data::server::Server;
use crate::handler;
use rocket::figment::Figment;
use crate::data::cache::Cache;
use crate::handler::not_found::not_found;

fn build_limits() -> Limits {
    Limits::default()
        .limit("json", 3.mebibytes())
        .limit("string", 3.mebibytes())
        .limit("data-form", 5.gibibytes())
        .limit("file", 5.gibibytes())
}

fn build_figment(srv: Server) -> Figment {
    Figment::from(rocket::Config::default())
        .merge((rocket::Config::ADDRESS, srv.host))
        .merge((rocket::Config::PORT, srv.port))
        .merge((rocket::Config::TEMP_DIR, srv.temp_dir))
        .merge((rocket::Config::LIMITS, build_limits()))
        .merge((rocket::Config::LOG_LEVEL, LogLevel::Critical))
}

#[tokio::main]
pub async fn run_server(srv: Server, ass: AssetManager) {
    let  server = rocket::custom(build_figment(srv))
        .manage(Arc::new(Cache::new(ass)))
        .mount("/",
               routes![handler::file::file,handler::files::files,handler::html::file])
        .register("/", catchers![not_found]);
    server.launch().await;
}