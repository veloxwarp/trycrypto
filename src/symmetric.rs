use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{LessonEnd, LessonIntro, crypto};

#[component]
pub fn SymmetricEncryptionLesson() -> impl IntoView {
    let initial_salt =
        crypto::random_hex(16).unwrap_or_else(|_| "00112233445566778899aabbccddeeff".to_owned());

    let (passphrase, set_passphrase) = signal(String::from("my backup passphrase"));
    let (salt, set_salt) = signal(initial_salt);
    let (derived_key, set_derived_key) = signal(String::new());
    let (derive_feedback, set_derive_feedback) = signal(String::new());

    let (encrypt_key, set_encrypt_key) = signal(String::new());
    let (plaintext, set_plaintext) = signal(String::from("My important backup contents"));
    let (nonce, set_nonce) = signal(String::new());
    let (ciphertext, set_ciphertext) = signal(String::new());
    let (encrypt_feedback, set_encrypt_feedback) = signal(String::new());

    let (decrypt_key, set_decrypt_key) = signal(String::new());
    let (decrypt_nonce, set_decrypt_nonce) = signal(String::new());
    let (decrypt_ciphertext, set_decrypt_ciphertext) = signal(String::new());
    let (decrypted_text, set_decrypted_text) = signal(String::new());
    let (decrypt_feedback, set_decrypt_feedback) = signal(String::new());

    let (passphrase_exercise, set_passphrase_exercise) = signal(Option::<bool>::None);
    let (wrong_key_exercise, set_wrong_key_exercise) = signal(Option::<bool>::None);
    let exercises_complete = Memo::new(move |_| {
        passphrase_exercise.get() == Some(true) && wrong_key_exercise.get() == Some(true)
    });

    view! {
        <LessonIntro
            number="02"
            title="Shared-secret encryption"
            eyebrow="Keep the backup private"
            summary="Encryption turns readable data into ciphertext that only someone with the secret key can recover."
        />

        <section class="motivation-section content-section">
            <p class="eyebrow">"Why would I want this?"</p>
            <h2>"I want to store my backup somewhere else. How do I stop the storage provider from reading it?"</h2>
            <div class="prose-grid">
                <p>"Before uploading the backup, I can encrypt it with a secret key. The storage provider sees only ciphertext. Later, I use that same key to decrypt the backup and recover the original data. This is called shared-secret or symmetric encryption: the same secret is used in both directions."</p>
                <p>"A random encryption key is great for cryptography but annoying to remember. One practical option is to start with a password or passphrase and run it through a key-derivation function. That gives us the fixed-size key that AES-GCM needs. Present-you can encrypt the backup; future-you can derive the same key and decrypt it."</p>
            </div>
            <aside class="precision-note">
                <strong>"Passphrase + salt → key."</strong>
                <p>"The salt isn't secret; you keep it with the encrypted backup. Using the same passphrase and the same salt derives the same key. In this lesson the browser uses PBKDF2 with SHA-256 to demonstrate that step, then uses the resulting 256-bit key with AES-GCM."</p>
            </aside>
        </section>

        <section class="workbench">
            <div class="workbench-heading">
                <div>
                    <p class="eyebrow">"Browser workbench"</p>
                    <h2>"Derive a key, encrypt, then decrypt."</h2>
                </div>
                <p>"Everything happens locally in your browser. The fields stay editable so you can change the passphrase, key, nonce, ciphertext, or plaintext and see what breaks."</p>
            </div>

            <div class="primer-grid">
                <div class="mini-workbench">
                    <p class="exercise-number">"Step 1 · Passphrase → key"</p>
                    <h3>"Derive a 256-bit key."</h3>
                    <label for="backup-passphrase">"Passphrase"</label>
                    <input
                        id="backup-passphrase"
                        prop:value=move || passphrase.get()
                        on:input=move |ev| {
                            set_passphrase.set(event_target_value(&ev));
                            set_derive_feedback.set(String::new());
                        }
                    />
                    <label for="backup-salt">"Salt (hex)"</label>
                    <input
                        id="backup-salt"
                        prop:value=move || salt.get()
                        on:input=move |ev| {
                            set_salt.set(event_target_value(&ev));
                            set_derive_feedback.set(String::new());
                        }
                    />
                    <div class="hero-actions">
                        <button
                            type="button"
                            class="button primary"
                            on:click=move |_| {
                                let passphrase = passphrase.get();
                                let salt = salt.get();
                                set_derive_feedback.set("Deriving…".to_owned());
                                spawn_local(async move {
                                    match crypto::derive_aes_key_hex(&passphrase, &salt).await {
                                        Ok(key) => {
                                            set_derived_key.set(key.clone());
                                            set_encrypt_key.set(key.clone());
                                            set_decrypt_key.set(key);
                                            set_derive_feedback.set("Key derived. It has been copied into the encryption and decryption steps below.".to_owned());
                                        }
                                        Err(_) => set_derive_feedback.set("Couldn't derive a key. Check that the salt is valid hex.".to_owned()),
                                    }
                                });
                            }
                        >"Derive key"</button>
                        <button
                            type="button"
                            class="button ghost"
                            on:click=move |_| {
                                match crypto::random_hex(16) {
                                    Ok(value) => {
                                        set_salt.set(value);
                                        set_derived_key.set(String::new());
                                        set_derive_feedback.set("Generated a new random salt. Derive the key again.".to_owned());
                                    }
                                    Err(_) => set_derive_feedback.set("Couldn't generate random bytes in this browser.".to_owned()),
                                }
                            }
                        >"New random salt"</button>
                    </div>
                    <div class="output" aria-live="polite">
                        <span>"DERIVED KEY · 32 BYTES · 64 HEX DIGITS"</span>
                        <code>{move || {
                            let value = derived_key.get();
                            if value.is_empty() { "—".to_owned() } else { value }
                        }}</code>
                    </div>
                    <p class="quiz-feedback" aria-live="polite">{move || derive_feedback.get()}</p>
                </div>

                <div class="mini-workbench">
                    <p class="exercise-number">"Step 2 · Plaintext → ciphertext"</p>
                    <h3>"Encrypt with AES-GCM."</h3>
                    <label for="encrypt-key">"Encryption key (64 hex digits)"</label>
                    <input
                        id="encrypt-key"
                        maxlength="64"
                        prop:value=move || encrypt_key.get()
                        on:input=move |ev| set_encrypt_key.set(event_target_value(&ev))
                    />
                    <label for="encrypt-plaintext">"Plaintext"</label>
                    <input
                        id="encrypt-plaintext"
                        prop:value=move || plaintext.get()
                        on:input=move |ev| set_plaintext.set(event_target_value(&ev))
                    />
                    <div class="hero-actions">
                        <button
                            type="button"
                            class="button primary"
                            on:click=move |_| {
                                let key = encrypt_key.get();
                                let text = plaintext.get();
                                set_encrypt_feedback.set("Encrypting…".to_owned());
                                spawn_local(async move {
                                    match crypto::aes_gcm_encrypt(&key, &text).await {
                                        Ok((new_nonce, encrypted)) => {
                                            set_nonce.set(new_nonce.clone());
                                            set_ciphertext.set(encrypted.clone());
                                            set_decrypt_key.set(key);
                                            set_decrypt_nonce.set(new_nonce);
                                            set_decrypt_ciphertext.set(encrypted);
                                            set_decrypted_text.set(String::new());
                                            set_encrypt_feedback.set("Encrypted. A fresh random nonce was generated and the values were copied into the decryption step.".to_owned());
                                        }
                                        Err(_) => set_encrypt_feedback.set("Encryption failed. Use a 32-byte key, shown as exactly 64 hex digits.".to_owned()),
                                    }
                                });
                            }
                        >"Encrypt"</button>
                        <button
                            type="button"
                            class="button ghost"
                            on:click=move |_| {
                                match crypto::random_hex(32) {
                                    Ok(key) => {
                                        set_encrypt_key.set(key.clone());
                                        set_decrypt_key.set(key);
                                        set_encrypt_feedback.set("Generated a random 256-bit key. Encrypt with it when you're ready.".to_owned());
                                    }
                                    Err(_) => set_encrypt_feedback.set("Couldn't generate random bytes in this browser.".to_owned()),
                                }
                            }
                        >"Use a random key"</button>
                    </div>
                    <div class="output">
                        <span>"NONCE · 12 BYTES"</span>
                        <code>{move || {
                            let value = nonce.get();
                            if value.is_empty() { "—".to_owned() } else { value }
                        }}</code>
                    </div>
                    <div class="output">
                        <span>"CIPHERTEXT + AUTHENTICATION TAG"</span>
                        <code>{move || {
                            let value = ciphertext.get();
                            if value.is_empty() { "—".to_owned() } else { value }
                        }}</code>
                    </div>
                    <p class="quiz-feedback" aria-live="polite">{move || encrypt_feedback.get()}</p>
                </div>
            </div>

            <div class="workbench-quiz">
                <p class="exercise-number">"Step 3 · Ciphertext → plaintext"</p>
                <h3>"Decrypt with the same key."</h3>
                <p>"Encryption copies the key, nonce, and ciphertext here for convenience. Edit any of them and see what happens."</p>
                <div class="mini-workbench">
                    <label for="decrypt-key">"Decryption key (64 hex digits)"</label>
                    <input
                        id="decrypt-key"
                        maxlength="64"
                        prop:value=move || decrypt_key.get()
                        on:input=move |ev| set_decrypt_key.set(event_target_value(&ev))
                    />
                    <label for="decrypt-nonce">"Nonce (24 hex digits)"</label>
                    <input
                        id="decrypt-nonce"
                        maxlength="24"
                        prop:value=move || decrypt_nonce.get()
                        on:input=move |ev| set_decrypt_nonce.set(event_target_value(&ev))
                    />
                    <label for="decrypt-ciphertext">"Ciphertext"</label>
                    <input
                        id="decrypt-ciphertext"
                        prop:value=move || decrypt_ciphertext.get()
                        on:input=move |ev| set_decrypt_ciphertext.set(event_target_value(&ev))
                    />
                    <div class="hero-actions">
                        <button
                            type="button"
                            class="button primary"
                            on:click=move |_| {
                                let key = decrypt_key.get();
                                let nonce = decrypt_nonce.get();
                                let ciphertext = decrypt_ciphertext.get();
                                set_decrypt_feedback.set("Decrypting…".to_owned());
                                spawn_local(async move {
                                    match crypto::aes_gcm_decrypt(&key, &nonce, &ciphertext).await {
                                        Ok(text) => {
                                            set_decrypted_text.set(text);
                                            set_decrypt_feedback.set("Decryption succeeded.".to_owned());
                                        }
                                        Err(_) => {
                                            set_decrypted_text.set(String::new());
                                            set_decrypt_feedback.set("Decryption failed. The key, nonce, and ciphertext must all match exactly.".to_owned());
                                        }
                                    }
                                });
                            }
                        >"Decrypt"</button>
                    </div>
                    <div class="output" aria-live="polite">
                        <span>"RECOVERED PLAINTEXT"</span>
                        <code>{move || {
                            let value = decrypted_text.get();
                            if value.is_empty() { "—".to_owned() } else { value }
                        }}</code>
                    </div>
                    <p class="quiz-feedback" aria-live="polite">{move || decrypt_feedback.get()}</p>
                </div>
            </div>
        </section>

        <section id="encryption-exercises" class="content-section planned-quiz">
            <p class="eyebrow">"Exercises"</p>
            <h2>"Use the workbench, then answer."</h2>

            <div class="workbench-quiz">
                <p class="exercise-number">"1 of 2 · The passphrase is exact input too"</p>
                <h3>"What happens if you add one trailing space to a passphrase?"</h3>
                <p>"Keep the same salt. Derive a key from “backup key”, then derive again from “backup key ” with one trailing space. Compare the two keys."</p>
                <div class="hero-actions">
                    <button class="button ghost" type="button" on:click=move |_| set_passphrase.set("backup key".to_owned())>"Load “backup key”"</button>
                    <button class="button ghost" type="button" on:click=move |_| set_passphrase.set("backup key ".to_owned())>"Load with trailing space"</button>
                </div>
                <div class="quiz-choice-row">
                    <span>"Do they derive the same key?"</span>
                    <button type="button" on:click=move |_| set_passphrase_exercise.set(Some(false))>"Same"</button>
                    <button type="button" on:click=move |_| set_passphrase_exercise.set(Some(true))>"Different"</button>
                </div>
                <p class="quiz-feedback" aria-live="polite">
                    {move || match passphrase_exercise.get() {
                        Some(true) => "Correct. The passphrase is input data too. One extra space changes the derived key completely.",
                        Some(false) => "Try both passphrases with the same salt and compare the derived keys.",
                        None => "",
                    }}
                </p>
            </div>

            <div class="workbench-quiz">
                <p class="exercise-number">"2 of 2 · The key must match"</p>
                <h3>"What if the decryption key is almost right?"</h3>
                <p>"Encrypt some plaintext. In the decryption step, change a single hex digit of the key and click Decrypt."</p>
                <div class="quiz-choice-row">
                    <span>"What should happen?"</span>
                    <button type="button" on:click=move |_| set_wrong_key_exercise.set(Some(false))>"It still decrypts"</button>
                    <button type="button" on:click=move |_| set_wrong_key_exercise.set(Some(true))>"Decryption fails"</button>
                </div>
                <p class="quiz-feedback" aria-live="polite">
                    {move || match wrong_key_exercise.get() {
                        Some(true) => "Exactly. There is no “close enough” key. AES-GCM also authenticates the encrypted data, so a wrong key—or altered ciphertext—causes decryption to fail rather than returning slightly-wrong plaintext.",
                        Some(false) => "Try changing one hex digit in the decryption key. AES-GCM should reject it.",
                        None => "",
                    }}
                </p>
            </div>
        </section>

        <section class="lesson-explanation content-section">
            <p class="eyebrow">"What did this prove?"</p>
            <h2>"The secret key protects the contents—but now we have to manage the secret."</h2>
            <p class="section-copy">"With AES-GCM, someone who doesn't know the key should not be able to read the encrypted backup. AES-GCM also detects accidental or deliberate changes to the encrypted data. But none of this tells us how two different people safely agree on a secret key in the first place."</p>
            <div class="principles">
                <article><span>"YES"</span><h3>"Confidentiality"</h3><p>"The ciphertext hides the plaintext from someone who doesn't have the key."</p></article>
                <article><span>"YES"</span><h3>"Tamper detection"</h3><p>"AES-GCM authenticates the encrypted data, so altered ciphertext should fail to decrypt."</p></article>
                <article><span>"NEXT"</span><h3>"Key distribution"</h3><p>"If two people want to communicate, they still need some safe way to establish or exchange that secret key."</p></article>
            </div>
        </section>

        <LessonEnd
            exercises_complete
            exercises_id="encryption-exercises"
            next_href="/keypairs"
            next_label="Continue to 03 — Public/private keypairs →"
        />
    }
}
