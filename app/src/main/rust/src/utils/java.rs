use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};

pub unsafe fn get_string(env: JNIEnv, context: JObject, key: &str) -> String {
    let jv = |s: &str| -> jni::errors::Result<JValue> { Ok(JObject::from(env.new_string(s)?).into()) };
    let v = env.call_method(context, "getString", "(Ljava/lang/String;)Ljava/lang/String;", &[jv(key).unwrap()]).unwrap().l().unwrap();
    let d = env.get_string(JString::from(v)).unwrap();
    d.into()
}