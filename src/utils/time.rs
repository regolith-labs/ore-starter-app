pub use web_time::*;

mod web_impl {
    use js_sys::Promise;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = setTimeout)]
        fn set_timeout(handler: &Closure<dyn FnMut()>, timeout: i32); // No return value needed here
    }

    /// Asynchronously sleeps for the specified duration in milliseconds (web version).
    pub async unsafe fn sleep(millis: u64) {
        let millis_i32 = millis as i32; // setTimeout takes i32
        let promise = Promise::new(&mut |resolve, _reject| {
            let closure = Closure::once(move || {
                // Directly resolve without checking return value
                let _ = resolve.call0(&JsValue::UNDEFINED);
            });
            set_timeout(&closure, millis_i32);
            // Leak the closure so it lives long enough for setTimeout
            closure.forget();
        });
        // Await the promise to pause execution
        let _ = JsFuture::from(promise).await;
    }
}

pub use web_impl::sleep;
