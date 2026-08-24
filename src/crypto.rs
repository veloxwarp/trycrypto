use js_sys::{Array, Object, Reflect, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CryptoKey, SubtleCrypto};

fn browser_crypto() -> Result<web_sys::Crypto, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window is unavailable"))?;
    window.crypto()
}

fn subtle() -> Result<SubtleCrypto, JsValue> {
    Ok(browser_crypto()?.subtle())
}

fn usages(values: &[&str]) -> Array {
    let result = Array::new();
    for value in values {
        result.push(&JsValue::from_str(value));
    }
    result
}

fn set(object: &Object, name: &str, value: &JsValue) -> Result<(), JsValue> {
    Reflect::set(object, &JsValue::from_str(name), value).map(|_| ())
}

fn aes_gcm_params(iv: &[u8]) -> Result<Object, JsValue> {
    let params = Object::new();
    let iv = Uint8Array::from(iv);
    set(&params, "name", &JsValue::from_str("AES-GCM"))?;
    set(&params, "iv", iv.as_ref())?;
    Ok(params)
}

async fn import_aes_key(raw_key: &[u8]) -> Result<CryptoKey, JsValue> {
    if raw_key.len() != 32 {
        return Err(JsValue::from_str("AES-256 keys must be exactly 32 bytes"));
    }

    let key_data = Uint8Array::from(raw_key);
    let key = subtle()?
        .import_key_with_str(
            "raw",
            key_data.as_ref(),
            "AES-GCM",
            false,
            usages(&["encrypt", "decrypt"]).as_ref(),
        )?
        .await?;

    key.dyn_into::<CryptoKey>()
}

pub fn random_hex(byte_len: usize) -> Result<String, JsValue> {
    let mut bytes = vec![0_u8; byte_len];
    browser_crypto()?.get_random_values_with_u8_array(&mut bytes)?;
    Ok(hex::encode(bytes))
}

pub async fn aes_gcm_encrypt(key_hex: &str, plaintext: &str) -> Result<String, JsValue> {
    let key_bytes =
        hex::decode(key_hex).map_err(|_| JsValue::from_str("key must be valid hexadecimal"))?;
    let key = import_aes_key(&key_bytes).await?;

    let mut iv = vec![0_u8; 12];
    browser_crypto()?.get_random_values_with_u8_array(&mut iv)?;
    let params = aes_gcm_params(&iv)?;

    let encrypted = subtle()?
        .encrypt_with_object_and_u8_array(&params, &key, plaintext.as_bytes())?
        .await?;

    let mut packaged = iv;
    packaged.extend(Uint8Array::new(&encrypted).to_vec());
    Ok(hex::encode(packaged))
}

pub async fn aes_gcm_decrypt(key_hex: &str, ciphertext_hex: &str) -> Result<String, JsValue> {
    let key_bytes =
        hex::decode(key_hex).map_err(|_| JsValue::from_str("key must be valid hexadecimal"))?;
    let packaged = hex::decode(ciphertext_hex)
        .map_err(|_| JsValue::from_str("ciphertext must be valid hexadecimal"))?;

    if packaged.len() <= 12 {
        return Err(JsValue::from_str("ciphertext package is too short"));
    }

    let (iv, ciphertext) = packaged.split_at(12);
    let key = import_aes_key(&key_bytes).await?;
    let params = aes_gcm_params(iv)?;
    let decrypted = subtle()?
        .decrypt_with_object_and_u8_array(&params, &key, ciphertext)?
        .await?;

    String::from_utf8(Uint8Array::new(&decrypted).to_vec())
        .map_err(|_| JsValue::from_str("decrypted bytes are not valid text"))
}
