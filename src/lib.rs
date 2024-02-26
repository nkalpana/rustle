use top_english_words::get_words_range;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    let _first_5_words = get_words_range::<String>(..5).unwrap();
    a + b
}
