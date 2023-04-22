use std::ffi::CString;
use std::io::Read;
use std::ptr::NonNull;
use jni::JNIEnv;
use jni::objects::{JObject};
use ndk::asset::AssetManager;

pub fn get_asset_manager(env: JNIEnv, asset_manager_object: JObject) -> AssetManager {
    let aasset_manager_pointer = unsafe {
        // https://docs.rs/android-ndk-sys/latest/android_ndk_sys/
        ndk_sys::AAssetManager_fromJava(env.get_native_interface(), *asset_manager_object)
    };
    let asset_manager = unsafe {
        // https://docs.rs/ndk/latest/ndk/asset/struct.AssetManager.html#method.from_ptr
        ndk::asset::AssetManager::from_ptr(NonNull::<ndk_sys::AAssetManager>::new_unchecked(
            aasset_manager_pointer,
        ))
    };
    // https://docs.rs/ndk-sys/0.4.0/ndk_sys/struct.AAssetManager.html
    asset_manager
}


pub fn read_resource_file(ass: &AssetManager, n: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let filename = &CString::new(n)?;
    match ass.open(filename) {
        Some(mut a) => {
            let mut bytes = vec![0; a.get_length()];
            a.read_exact(&mut bytes)?;
            Ok(bytes)
        }
        None => {
            Err("Error reading")?
        }
    }
}
