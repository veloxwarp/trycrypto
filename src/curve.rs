use js_sys::Array;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CryptoKey;

fn usages(values: &[&str]) -> Array {
    let result = Array::new();
    for value in values {
        result.push(&JsValue::from_str(value));
    }
    result
}

pub async fn generate_x25519() -> Result<(CryptoKey, CryptoKey), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window unavailable"))?;
    let generated = window
        .crypto()?
        .subtle()
        .generate_key_with_str("X25519", true, usages(&["deriveBits"]).as_ref())?
        .await?;
    let public = js_sys::Reflect::get(&generated, &JsValue::from_str("publicKey"))?.dyn_into()?;
    let private = js_sys::Reflect::get(&generated, &JsValue::from_str("privateKey"))?.dyn_into()?;
    Ok((public, private))
}
