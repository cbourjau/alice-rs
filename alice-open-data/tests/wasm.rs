#![cfg(test)]
#![cfg(target_arch = "wasm32")]

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys;

use reqwest::Client;

use alice_open_data::get_file_list;

wasm_bindgen_test_configure!(run_in_browser);

fn log(x: &str) {
    use web_sys;
    web_sys::console::log_1(&JsValue::from_str(x));
}

#[wasm_bindgen_test(async)]
pub async fn test_get_list() -> () {
    let list = get_file_list(139_038).await.unwrap();
    log(&format!("{:?}", list));
}

#[wasm_bindgen_test(async)]
pub async fn test_get_bytes() -> () {
    let resp = Client::new()
        .get("http://cirrocumuli.com/ALICE_LHC10h_PbPb_ESD_139038_file_index.txt")
        .send()
        .await
        .unwrap();
    let bytes = resp.bytes().await;
    log(&format!("{:?}", bytes));
}

#[wasm_bindgen_test(async)]
async fn download_partial() {
    use reqwest::header::RANGE;
    let url =
        "http://cirrocumuli.com/eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root";
    let rsp = Client::new()
        .get(url)
        .header("User-Agent", "alice-rs")
        .header(RANGE, "bytes=0-1023")
        .send()
        .await
        .unwrap();
    log(&format!("{:?}", &rsp));

    let partial = rsp.bytes().await.unwrap();
    assert_eq!(partial.len(), 1024);
}
