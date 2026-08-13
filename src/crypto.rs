use js_sys::Uint8Array;
use wasm_bindgen::JsValue;

pub async fn sha256_hex(input: &str) -> Result<String, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window is unavailable"))?;
    let crypto = window.crypto()?;
    let subtle = crypto.subtle();

    let digest = subtle
        .digest_with_str_and_u8_array("SHA-256", input.as_bytes())?
        .await?;

    let bytes = Uint8Array::new(&digest).to_vec();
    Ok(hex::encode(bytes))
}
