#![cfg(test)]
#![cfg(target_arch = "wasm32")]

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys;

use alice_open_data::get_file_list;

wasm_bindgen_test_configure!(run_in_browser);

fn log(x: &str) {
    web_sys::console::log_1(&JsValue::from_str(x));
}

#[wasm_bindgen_test(async)]
pub async fn test_get_list() -> (){
    let list = get_file_list(139_038).await.unwrap();
    log(&format!("{:?}", list));
}
