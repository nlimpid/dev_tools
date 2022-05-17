mod core;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn gen(name: &str) {
    let output = log(&format!("Hello, {}!", name));
}

struct Columns {
    name: String,
    sql_type: String,
}
