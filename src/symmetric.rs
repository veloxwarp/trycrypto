use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{LessonEnd, LessonIntro, crypto};

#[component]
pub fn SymmetricEncryptionLesson() -> impl IntoView {
    let initial_key = crypto::random_hex(32).unwrap_or_else(|_| {
        "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_owned()
    });

    let (shared_key, set_shared_key) = signal(initial_key.clone());
    let (key_feedback, set_key_feedback) = signal(String::new());

    let (plaintext, set_plaintext) = signal(String::from("My important backup contents"));
    let (nonce, set_nonce) = signal(String::new());
    let (ciphertext, set_ciphertext) = signal(String::new());
    let (encrypt_feedback, set_encrypt_feedback) = signal(String::new());

    let (decrypt_key, set_decrypt_key) = signal(initial_key);
    let (decrypt_nonce, set_decrypt_nonce) = signal(String::new());
    let (decrypt_ciphertext, set_decrypt_ciphertext) = signal(String::new());
    let (decrypted_text, set_decrypted_text) = signal(String::new());
    let (decrypt_feedback, set_decrypt_feedback) = signal(String::new());

    let (round_trip_exercise, set_round_trip_exercise) = signal(Option::<bool>::None);
    let (wrong_key_exercise, set_wrong_key_exercise) = signal(Option::<bool>::None);
    let exercises_complete = Memo::new(move |_| {
        round_trip_exercise.get() == Some(true) && wrong_key_exercise.get() == Some(true)
    });

    view! {
        <LessonIntro
            number="02"
            title="Shared-secret encryption"
            eyebrow="Keep the backup private"
            summary="Encryption turns readable data into ciphertext that only someone with the secret key can recover."
        />

        <section class="motivation-section content-section">
            <h2>"I want to store my backup somewhere else. How do I stop the storage provider from reading it?"</h2>
            <p class="section-copy">"Before uploading the backup, I can encrypt it with a secret key. The storage provider sees only ciphertext. Later, I use that same key to decrypt the backup and recover the original data. This is called shared-secret or symmetric encryption: the same secret key is used for both encryption and decryption."</p>
            <p class="section-copy">"For this lesson we'll use a randomly generated 256-bit key, displayed as 64 hexadecimal digits. Don't worry yet about how two people safely exchange a key; that problem will motivate the lessons that follow."</p>
        </section>

        <section class="workbench">
            <div class="workbench-heading">
                <div>
                    <h2>"Encrypt, then decrypt with the same key."</h2>
                </div>
                <p>"Everything happens locally in your browser. Generate a shared key, encrypt some plaintext, then use the same key to recover it."</p>
            </div>

            <div class="mini-workbench">
                <p class="exercise-number">"Shared key"</p>
                <h3>"A random 256-bit secret."</h3>
                <label for="shared-key">"Shared key (64 hex digits)"</label>
                <input
                    id="shared-key"
                    maxlength="64"
                    prop:value=move || shared_key.get()
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        set_shared_key.set(value.clone());
                        set_decrypt_key.set(value);
                        set_key_feedback.set(String::new());
                    }
                />
                <div class="hero-actions">
                    <button
                        type="button"
                        class="button primary"
                        on:click=move |_| {
                            match crypto::random_hex(32) {
                                Ok(key) => {
                                    set_shared_key.set(key.clone());
                                    set_decrypt_key.set(key);
                                    set_key_feedback.set("Generated a new random 256-bit key.".to_owned());
                                }
                                Err(_) => set_key_feedback.set("Couldn't generate random bytes in this browser.".to_owned()),
                            }
                        }
                    >"Generate new key"</button>
                </div>
                <p class="quiz-feedback" aria-live="polite">{move || key_feedback.get()}</p>
            </div>

            <div class="workbench-quiz">
                <p class="exercise-number">"Step 1 · Plaintext → ciphertext"</p>
                <h3>"Encrypt the backup with AES-GCM."</h3>
                <div class="mini-workbench">
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
                                let key = shared_key.get();
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
                                            set_decrypt_feedback.set(String::new());
                                            set_encrypt_feedback.set("Encrypted. The nonce and ciphertext were copied into the decryption step below.".to_owned());
                                        }
                                        Err(_) => set_encrypt_feedback.set("Encryption failed. The shared key must be exactly 64 hexadecimal digits.".to_owned()),
                                    }
                                });
                            }
                        >"Encrypt"</button>
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
                <p class="exercise-number">"Step 2 · Ciphertext → plaintext"</p>
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
            <h2>"Exercises"</h2>
            <p class="section-copy">"Use the workbench above before answering."</p>

            <div class="workbench-quiz">
                <p class="exercise-number">"1 of 2 · Round trip"</p>
                <h3>"What happens when you decrypt with the same key?"</h3>
                <p>"Encrypt some plaintext, then decrypt the resulting ciphertext without changing the key, nonce, or ciphertext."</p>
                <div class="quiz-choice-row">
                    <span>"What should you recover?"</span>
                    <button type="button" on:click=move |_| set_round_trip_exercise.set(Some(true))>"The original plaintext"</button>
                    <button type="button" on:click=move |_| set_round_trip_exercise.set(Some(false))>"Something different"</button>
                </div>
                <p class="quiz-feedback" aria-live="polite">
                    {move || match round_trip_exercise.get() {
                        Some(true) => "Correct. With the same key and unchanged encrypted data, decryption recovers the original plaintext exactly.",
                        Some(false) => "Try the full encrypt-and-decrypt round trip in the workbench above.",
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
            <h2>"What did this prove?"</h2>
            <p class="section-copy">"With AES-GCM, someone who doesn't know the shared key should not be able to read the encrypted backup. AES-GCM also detects accidental or deliberate changes to the encrypted data. But now we have a new problem: if two people want to communicate, how do they safely establish or exchange that secret key?"</p>
            <div class="principles">
                <article><span>"YES"</span><h3>"Confidentiality"</h3><p>"The ciphertext hides the plaintext from someone who doesn't have the key."</p></article>
                <article><span>"YES"</span><h3>"Tamper detection"</h3><p>"AES-GCM authenticates the encrypted data, so altered ciphertext should fail to decrypt."</p></article>
                <article><span>"NEXT"</span><h3>"Key distribution"</h3><p>"Two people still need some safe way to establish or exchange the shared secret key."</p></article>
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
