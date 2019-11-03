#![cfg(test)]

pub use cfg_gated::log;

#[cfg(target_arch="wasm32")]
mod cfg_gated {
    use wasm_bindgen::JsValue;
    use web_sys;

    /// Print a debuggable object to the console
    pub fn log<D: std::fmt::Debug>(thing: D) {
        let s = format!("{:?}", thing);
        web_sys::console::log_1(&JsValue::from_str(&s));
    }
}

#[cfg(not(target_arch="wasm32"))]
mod cfg_gated {
    pub fn log<D: std::fmt::Debug>(thing: D) {
        std::dbg!(thing);
    }
}
