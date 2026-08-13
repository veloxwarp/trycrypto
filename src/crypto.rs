use js_sys::{Array, JSON, Object, Reflect, Uint8Array};
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

fn key_pair(value: JsValue) -> Result<(CryptoKey, CryptoKey), JsValue> {
    let public_key = Reflect::get(&value, &JsValue::from_str("publicKey"))?
        .dyn_into::<CryptoKey>()?;
    let private_key = Reflect::get(&value, &JsValue::from_str("privateKey"))?
        .dyn_into::<CryptoKey>()?;
    Ok((public_key, private_key))
}

async fn export_jwk(key: &CryptoKey) -> Result<String, JsValue> {
    let jwk = subtle()?.export_key("jwk", key)?.await?;
    JSON::stringify(&jwk)?
        .as_string()
        .ok_or_else(|| JsValue::from_str("could not serialize JWK"))
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

pub struct DisplayKeyPair {
    pub public_key: CryptoKey,
    pub private_key: CryptoKey,
    pub public_jwk: String,
    pub private_jwk: String,
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

pub async fn generate_rsa_oaep_keypair() -> Result<DisplayKeyPair, JsValue> {
    let params = Object::new();
    let exponent = Uint8Array::from(&[1_u8, 0, 1][..]);
    set(&params, "name", &JsValue::from_str("RSA-OAEP"))?;
    set(&params, "modulusLength", &JsValue::from_f64(2048.0))?;
    set(&params, "publicExponent", exponent.as_ref())?;
    set(&params, "hash", &JsValue::from_str("SHA-256"))?;

    let pair = subtle()?
        .generate_key_with_object(&params, true, usages(&["encrypt", "decrypt"]).as_ref())?
        .await?;
    let (public_key, private_key) = key_pair(pair)?;
    let public_jwk = export_jwk(&public_key).await?;
    let private_jwk = export_jwk(&private_key).await?;

    Ok(DisplayKeyPair {
        public_key,
        private_key,
        public_jwk,
        private_jwk,
    })
}

pub async fn rsa_oaep_encrypt(public_key: &CryptoKey, plaintext: &str) -> Result<String, JsValue> {
    let ciphertext = subtle()?
        .encrypt_with_str_and_u8_array("RSA-OAEP", public_key, plaintext.as_bytes())?
        .await?;
    Ok(hex::encode(Uint8Array::new(&ciphertext).to_vec()))
}

pub async fn rsa_oaep_decrypt(
    private_key: &CryptoKey,
    ciphertext_hex: &str,
) -> Result<String, JsValue> {
    let ciphertext = hex::decode(ciphertext_hex)
        .map_err(|_| JsValue::from_str("ciphertext must be valid hexadecimal"))?;
    let plaintext = subtle()?
        .decrypt_with_str_and_u8_array("RSA-OAEP", private_key, &ciphertext)?
        .await?;
    String::from_utf8(Uint8Array::new(&plaintext).to_vec())
        .map_err(|_| JsValue::from_str("decrypted bytes are not valid text"))
}

pub async fn generate_ecdsa_keypair() -> Result<DisplayKeyPair, JsValue> {
    let params = Object::new();
    set(&params, "name", &JsValue::from_str("ECDSA"))?;
    set(&params, "namedCurve", &JsValue::from_str("P-256"))?;

    let pair = subtle()?
        .generate_key_with_object(&params, true, usages(&["sign", "verify"]).as_ref())?
        .await?;
    let (public_key, private_key) = key_pair(pair)?;
    let public_jwk = export_jwk(&public_key).await?;
    let private_jwk = export_jwk(&private_key).await?;

    Ok(DisplayKeyPair {
        public_key,
        private_key,
        public_jwk,
        private_jwk,
    })
}

fn ecdsa_params() -> Result<Object, JsValue> {
    let params = Object::new();
    set(&params, "name", &JsValue::from_str("ECDSA"))?;
    set(&params, "hash", &JsValue::from_str("SHA-256"))?;
    Ok(params)
}

pub async fn ecdsa_sign(private_key: &CryptoKey, message: &str) -> Result<String, JsValue> {
    let params = ecdsa_params()?;
    let signature = subtle()?
        .sign_with_object_and_u8_array(&params, private_key, message.as_bytes())?
        .await?;
    Ok(hex::encode(Uint8Array::new(&signature).to_vec()))
}

pub async fn ecdsa_verify(
    public_key: &CryptoKey,
    message: &str,
    signature_hex: &str,
) -> Result<bool, JsValue> {
    let signature = hex::decode(signature_hex)
        .map_err(|_| JsValue::from_str("signature must be valid hexadecimal"))?;
    let params = ecdsa_params()?;
    subtle()?
        .verify_with_object_and_u8_array_and_u8_array(
            &params,
            public_key,
            &signature,
            message.as_bytes(),
        )?
        .await?
        .as_bool()
        .ok_or_else(|| JsValue::from_str("verify did not return a boolean"))
}
