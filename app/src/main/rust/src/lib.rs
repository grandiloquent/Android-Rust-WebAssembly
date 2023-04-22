mod utils;
mod seek_stream;
mod server;
mod data;
mod handler;

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring};
use utils::net::get_local_ip_address;
use crate::utils::net::listen_available_port;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_psycho_euphoria_plane_MainActivity_startServer<'a>(
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
    let port = listen_available_port(3000).expect("Couldn't listen_available_port");

    let output = env
        .new_string(format!("{}:{}", host, port))
        .expect("Couldn't create java string!");
    output
}
