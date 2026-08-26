use js_sys::{Array, Object, Reflect, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CryptoKey, SubtleCrypto};

fn subtle() -> Result<SubtleCrypto, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window unavailable"))?;
    Ok(window.crypto()?.subtle())
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

fn pair(value: JsValue) -> Result<(CryptoKey, CryptoKey), JsValue> {
    let public = Reflect::get(&value, &JsValue::from_str("publicKey"))?.dyn_into()?;
    let private = Reflect::get(&value, &JsValue::from_str("privateKey"))?.dyn_into()?;
    Ok((public, private))
}

fn decode_base64url(value: &str) -> Result<Vec<u8>, JsValue> {
    let mut out = Vec::with_capacity(value.len() * 3 / 4);
    let mut acc = 0_u32;
    let mut bits = 0_u8;
    for byte in value.bytes() {
        let digit = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            _ => return Err(JsValue::from_str("invalid exported key encoding")),
        };
        acc = (acc << 6) | u32::from(digit);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((acc >> bits) & 0xff) as u8);
        }
    }
    Ok(out)
}

async fn raw_hex(key: &CryptoKey) -> Result<String, JsValue> {
    let raw = subtle()?.export_key("raw", key)?.await?;
    Ok(hex::encode(Uint8Array::new(&raw).to_vec()))
}

async fn private_hex(key: &CryptoKey) -> Result<String, JsValue> {
    let jwk = subtle()?.export_key("jwk", key)?.await?;
    let d = Reflect::get(&jwk, &JsValue::from_str("d"))?
        .as_string()
        .ok_or_else(|| JsValue::from_str("private key export did not contain d"))?;
    Ok(hex::encode(decode_base64url(&d)?))
}

#[derive(Clone)]
pub struct KeyPair {
    pub public: CryptoKey,
    pub private: CryptoKey,
    pub public_hex: String,
    pub private_hex: String,
}

pub struct SealedMessage {
    pub ciphertext_hex: String,
}

async fn import_x25519_public(bytes: &[u8]) -> Result<CryptoKey, JsValue> {
    let key_data: Object = Uint8Array::from(bytes).unchecked_into();
    subtle()?
        .import_key_with_str("raw", &key_data, "X25519", true, usages(&[]).as_ref())?
        .await?
        .dyn_into()
}

pub async fn x25519_public_from_hex(value: &str) -> Result<CryptoKey, JsValue> {
    let bytes = hex::decode(value.trim())
        .map_err(|_| JsValue::from_str("public key must be hexadecimal"))?;
    if bytes.len() != 32 {
        return Err(JsValue::from_str("public key must be 32 bytes"));
    }
    import_x25519_public(&bytes).await
}

pub async fn generate_x25519() -> Result<KeyPair, JsValue> {
    let generated = subtle()?
        .generate_key_with_str("X25519", true, usages(&["deriveKey"]).as_ref())?
        .await?;
    let (public, private) = pair(generated)?;
    Ok(KeyPair {
        public_hex: raw_hex(&public).await?,
        private_hex: private_hex(&private).await?,
        public,
        private,
    })
}

async fn shared_aes_key(private: &CryptoKey, public: &CryptoKey) -> Result<CryptoKey, JsValue> {
    let agreement = Object::new();
    set(&agreement, "name", &JsValue::from_str("X25519"))?;
    set(&agreement, "public", public.as_ref())?;

    let aes = Object::new();
    set(&aes, "name", &JsValue::from_str("AES-GCM"))?;
    set(&aes, "length", &JsValue::from_f64(256.0))?;

    subtle()?
        .derive_key_with_object_and_object(
            &agreement,
            private,
            &aes,
            false,
            usages(&["encrypt", "decrypt"]).as_ref(),
        )?
        .await?
        .dyn_into()
}

fn aes_params(nonce: &[u8]) -> Result<Object, JsValue> {
    let params = Object::new();
    let nonce = Uint8Array::from(nonce);
    set(&params, "name", &JsValue::from_str("AES-GCM"))?;
    set(&params, "iv", nonce.as_ref())?;
    Ok(params)
}

pub async fn seal_for(
    recipient_public: &CryptoKey,
    plaintext: &str,
) -> Result<SealedMessage, JsValue> {
    let ephemeral = generate_x25519().await?;
    let key = shared_aes_key(&ephemeral.private, recipient_public).await?;

    let mut nonce = vec![0_u8; 12];
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window unavailable"))?;
    window
        .crypto()?
        .get_random_values_with_u8_array(&mut nonce)?;
    let params = aes_params(&nonce)?;
    let encrypted = subtle()?
        .encrypt_with_object_and_u8_array(&params, &key, plaintext.as_bytes())?
        .await?;

    let mut packaged = hex::decode(&ephemeral.public_hex)
        .map_err(|_| JsValue::from_str("invalid temporary public key"))?;
    packaged.extend(nonce);
    packaged.extend(Uint8Array::new(&encrypted).to_vec());

    Ok(SealedMessage {
        ciphertext_hex: hex::encode(packaged),
    })
}

pub async fn open_from(
    recipient_private: &CryptoKey,
    ciphertext_hex: &str,
) -> Result<String, JsValue> {
    let packaged =
        hex::decode(ciphertext_hex).map_err(|_| JsValue::from_str("invalid ciphertext hex"))?;
    if packaged.len() <= 44 {
        return Err(JsValue::from_str("ciphertext package is too short"));
    }
    let (temporary_public, encrypted) = packaged.split_at(32);
    let temporary_public = import_x25519_public(temporary_public).await?;
    let key = shared_aes_key(recipient_private, &temporary_public).await?;
    let (iv, ciphertext) = encrypted.split_at(12);
    let params = aes_params(iv)?;
    let decrypted = subtle()?
        .decrypt_with_object_and_u8_array(&params, &key, ciphertext)?
        .await?;
    String::from_utf8(Uint8Array::new(&decrypted).to_vec())
        .map_err(|_| JsValue::from_str("decrypted bytes were not text"))
}

pub async fn generate_ed25519() -> Result<KeyPair, JsValue> {
    let generated = subtle()?
        .generate_key_with_str("Ed25519", true, usages(&["sign", "verify"]).as_ref())?
        .await?;
    let (public, private) = pair(generated)?;
    Ok(KeyPair {
        public_hex: raw_hex(&public).await?,
        private_hex: private_hex(&private).await?,
        public,
        private,
    })
}

pub async fn sign(private: &CryptoKey, message: &str) -> Result<String, JsValue> {
    let signature = subtle()?
        .sign_with_str_and_u8_array("Ed25519", private, message.as_bytes())?
        .await?;
    Ok(hex::encode(Uint8Array::new(&signature).to_vec()))
}

pub async fn verify(
    public: &CryptoKey,
    message: &str,
    signature_hex: &str,
) -> Result<bool, JsValue> {
    let signature =
        hex::decode(signature_hex).map_err(|_| JsValue::from_str("invalid signature hex"))?;
    subtle()?
        .verify_with_str_and_u8_array_and_u8_array(
            "Ed25519",
            public,
            &signature,
            message.as_bytes(),
        )?
        .await?
        .as_bool()
        .ok_or_else(|| JsValue::from_str("verify returned a non-boolean"))
}
