// mod utils;

use wasm_bindgen::prelude::*;
// use std::collections::HashMap;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// fn data_injection() {
//     let mut data = HashMap::new();
//     data.insert(
//         "a".to_string(),
//         "shit1".to_string()
//     );
//     data.insert(
//         "b".to_string(),
//         "shit2".to_string()
//     );
//     data.insert(
//         "c".to_string(),
//         "shit3".to_string()
//     );
//     println!("{}", data.len());
//     for (k, v) in &data {
//         println!("{}->{}", k, v);
//     }
// }

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}
// #[wasm_bindgen]
// pub fn do_calculation(s: &str) {
//
// }

#[wasm_bindgen]
pub fn say_something(s: &str) {
    alert(&*format!("Though you said {}, shit still happens.", s));
}

#[wasm_bindgen]
pub fn important_3(s: &str) -> String{
    return format!("{}!{}!{}!", s, s, s).to_string();
}
