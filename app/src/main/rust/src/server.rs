use crate::data::cache::Cache;
use crate::data::server::Server;
use crate::handler::not_found::not_found;
use crate::{data, handler};
use log::log;
use ndk::asset::AssetManager;
use rocket::config::LogLevel;
use rocket::data::{Limits, ToByteUnit};
use rocket::figment::Figment;
use rocket::{catchers, routes};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

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

pub struct Database(pub Arc<Mutex<Connection>>);

fn initialize_database(conn: &Connection) {
    match conn.execute(
        r#"CREATE TABLE IF NOT EXISTS "video" (
	"id"	INTEGER NOT NULL UNIQUE,
	"uri"	TEXT NOT NULL,
	"title"	TEXT,
    "subtitle"	TEXT,
	"file"	TEXT,
	"image"	TEXT,
	"source_type"	INTEGER,
    "views"	INTEGER,
	"hidden"	INTEGER,
	"create_at"	INTEGER,
	"update_at"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT)
)"#,
        [],
    ) {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error {}", err);
        }
    };
    // conn.execute(
    //     r#"CREATE UNIQUE INDEX IF NOT EXISTS "uri_idx_unique" ON "video" ("uri" ASC);"#,
    //     [],
    // );
//     r#"ALTER TABLE video RENAME TO temp_video;

// CREATE TABLE "video" ("id" INTEGER NOT NULL UNIQUE,"uri" TEXT NOT NULL,"title" TEXT,"subtitle" TEXT,"file" TEXT,"image" TEXT,"source_type" INTEGER,"views" INTEGER,"hidden" INTEGER,"create_at" INTEGER,"update_at" INTEGER,PRIMARY KEY("id" AUTOINCREMENT));

// INSERT INTO video(id,uri,title,subtitle,file,image,source_type,hidden,create_at,update_at) SELECT id,uri,title,subtitle,file,image,source_type,hidden,create_at,update_at FROM temp_video;

// DROP TABLE temp_video;"#.split("\n")
// .into_iter()
// .for_each(|l|{
// if l.is_empty() {
// return;
// }
// conn.execute(
//     l,
//     [],
// );
// });
// conn.execute_batch(r#"BEGIN;
// delete from video where id in (select id from (SELECT id, COUNT(*) c FROM video GROUP BY uri HAVING c > 1));
// CREATE UNIQUE INDEX IF NOT EXISTS "uri_idx_unique" ON "video" ("uri" ASC);
// END;"#);
}

#[tokio::main]
pub async fn run_server(srv: Server, ass: AssetManager) {
    let conn = Connection::open(srv.db.as_str()).expect("");
    initialize_database(&conn);

    let server = rocket::custom(build_figment(srv))
        .attach(data::cors::CORS)
        .manage(Arc::new(Cache::new(ass)))
        .manage(Arc::new(Database(Arc::new(Mutex::new(conn)))))
        .mount(
            "/",
            routes![handler::db::list,handler::db::delete_video,handler::db::hidden_video,handler::files::files,handler::html::file,handler::video::parse,handler::video::get,handler::video::get_url],
        )
        .register("/", catchers![not_found]);
    server.launch().await;
}
