use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    String::from("Hello, world!")
}
