use crate::{data, handler};
use crate::data::cache::Cache;
use crate::data::server::Server;
use crate::handler::not_found::not_found;
use ndk::asset::AssetManager;
use rocket::{catchers, routes};
use rocket::config::LogLevel;
use rocket::data::{Limits, ToByteUnit};
use rocket::figment::Figment;
use std::sync::Arc;

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
    let server = rocket::custom(build_figment(srv))
        .attach(data::cors::CORS)
        .manage(Arc::new(Cache::new(ass)))
        .mount("/",
               routes![handler::file::file,handler::files::files,handler::html::file,handler::subtitle::subtitle])
        .register("/", catchers![not_found]);
    server.launch().await;
}