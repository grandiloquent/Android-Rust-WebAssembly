use wasm_bindgen::prelude::Closure;

pub struct Listner(pub Closure<dyn FnMut()>);
unsafe impl Send for Listner {}
unsafe impl Sync for Listner {}
