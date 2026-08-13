use js_sys::{Array, Object, Reflect, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CryptoKey, SubtleCrypto};

const PBKDF2_ITERATIONS: u32 = 100_000;

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

pub async fn sha256_hex(input: &str) -> Result<String, JsValue> {
    let digest = subtle()?
        .digest_with_str_and_u8_array("SHA-256", input.as_bytes())?
        .await?;

    let bytes = Uint8Array::new(&digest).to_vec();
    Ok(hex::encode(bytes))
}

pub async fn derive_aes_key_hex(passphrase: &str, salt_hex: &str) -> Result<String, JsValue> {
    let salt =
        hex::decode(salt_hex).map_err(|_| JsValue::from_str("salt must be valid hexadecimal"))?;

    let passphrase_bytes = Uint8Array::from(passphrase.as_bytes());
    let base_key = subtle()?
        .import_key_with_str(
            "raw",
            passphrase_bytes.as_ref(),
            "PBKDF2",
            false,
            usages(&["deriveBits"]).as_ref(),
        )?
        .await?
        .dyn_into::<CryptoKey>()?;

    let params = Object::new();
    let salt_array = Uint8Array::from(salt.as_slice());
    set(&params, "name", &JsValue::from_str("PBKDF2"))?;
    set(&params, "salt", salt_array.as_ref())?;
    set(
        &params,
        "iterations",
        &JsValue::from_f64(f64::from(PBKDF2_ITERATIONS)),
    )?;
    set(&params, "hash", &JsValue::from_str("SHA-256"))?;

    let bits = subtle()?
        .derive_bits_with_object(&params, &base_key, 256)?
        .await?;

    Ok(hex::encode(Uint8Array::new(&bits).to_vec()))
}

pub async fn aes_gcm_encrypt(key_hex: &str, plaintext: &str) -> Result<(String, String), JsValue> {
    let key_bytes =
        hex::decode(key_hex).map_err(|_| JsValue::from_str("key must be valid hexadecimal"))?;
    let key = import_aes_key(&key_bytes).await?;

    let mut iv = vec![0_u8; 12];
    browser_crypto()?.get_random_values_with_u8_array(&mut iv)?;
    let params = aes_gcm_params(&iv)?;

    let encrypted = subtle()?
        .encrypt_with_object_and_u8_array(&params, &key, plaintext.as_bytes())?
        .await?;

    Ok((
        hex::encode(iv),
        hex::encode(Uint8Array::new(&encrypted).to_vec()),
    ))
}

pub async fn aes_gcm_decrypt(
    key_hex: &str,
    iv_hex: &str,
    ciphertext_hex: &str,
) -> Result<String, JsValue> {
    let key_bytes =
        hex::decode(key_hex).map_err(|_| JsValue::from_str("key must be valid hexadecimal"))?;
    let iv =
        hex::decode(iv_hex).map_err(|_| JsValue::from_str("nonce must be valid hexadecimal"))?;
    let ciphertext = hex::decode(ciphertext_hex)
        .map_err(|_| JsValue::from_str("ciphertext must be valid hexadecimal"))?;

    if iv.len() != 12 {
        return Err(JsValue::from_str("AES-GCM nonce must be exactly 12 bytes"));
    }

    let key = import_aes_key(&key_bytes).await?;
    let params = aes_gcm_params(&iv)?;
    let decrypted = subtle()?
        .decrypt_with_object_and_u8_array(&params, &key, &ciphertext)?
        .await?;

    String::from_utf8(Uint8Array::new(&decrypted).to_vec())
        .map_err(|_| JsValue::from_str("decrypted bytes are not valid text"))
}
