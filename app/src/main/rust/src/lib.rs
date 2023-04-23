#[macro_use]
extern crate rocket;

mod utils;
mod seek_stream;
mod server;
mod data;
mod handler;


use jni::JNIEnv;
use jni::objects::{JObject, JString};
use utils::net::get_local_ip_address;
use crate::data::server::Server;
use crate::server::run_server;
use crate::utils::asset::get_asset_manager;
use crate::utils::java::get_string;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_psycho_euphoria_plane_ServerService_startServer<'a>(
    env: JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    context: JObject<'a>,
    asset_manager: JObject<'a>,
) -> JString<'a> {
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Trace)
            .with_tag("Rust"),
    );
    let host = match get_local_ip_address(false) {
        Some(h) => h.to_string(),
        None => "0.0.0.0".to_string(),
    };

    // TODO:
    // use the TcpListener.bind to find some available port
    // would let the rocket bind fails
    // drop TcpListener early also don't work.
    //listen_available_port(5000).expect("Couldn't listen_available_port");
    let ass = get_asset_manager(env, asset_manager);


    unsafe {
        let temp_dir = get_string(&env, context, "temp_dir");
        let port = get_string(&env, context, "port").parse::<u16>().unwrap();
        let db = get_string(&env, context, "db");

        let output = env
            .new_string(format!("{}:{}", host, port))
            .expect("Couldn't create java string!");
        std::thread::spawn(move || {
            run_server(Server {
                host,
                port,
                temp_dir,
                db,
            }, ass);
        });

        output
    }
}
