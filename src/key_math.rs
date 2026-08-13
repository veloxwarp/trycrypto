pub fn public_from_private_hex(private_hex: &str) -> Result<String, &'static str> {
    let bytes = hex::decode(private_hex).map_err(|_| "Use exactly 64 hexadecimal digits.")?;
    let private: [u8; 32] = bytes
        .try_into()
        .map_err(|_| "Use exactly 64 hexadecimal digits.")?;
    Ok(hex::encode(x25519_dalek::x25519(
        private,
        x25519_dalek::X25519_BASEPOINT_BYTES,
    )))
}
