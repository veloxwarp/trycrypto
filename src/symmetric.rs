use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{LessonEnd, LessonIntro, crypto};

#[component]
pub fn SymmetricEncryptionLesson() -> impl IntoView {
    const EXERCISE_KEY: &str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
    const ENCRYPT_EXERCISE_PLAINTEXT: &str = "Archive this backup.";
    const DECRYPT_EXERCISE_NONCE: &str = "101112131415161718191a1b";
    const DECRYPT_EXERCISE_CIPHERTEXT: &str =
        "3f9ffb7d3cb91ac5af07617b661c0d73b63e2a2e69a736d59ed7a2e2d432a154b575453da87ba5d7f58a";
    const DECRYPT_EXERCISE_PLAINTEXT: &str = "Backup verified and ready.";

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
    let (exercise_nonce, set_exercise_nonce) = signal(String::new());
    let (exercise_ciphertext, set_exercise_ciphertext) = signal(String::new());
    let (encrypt_exercise_done, set_encrypt_exercise_done) = signal(false);
    let (exercise_plaintext, set_exercise_plaintext) = signal(String::new());
    let (decrypt_exercise_feedback, set_decrypt_exercise_feedback) = signal(String::new());
    let (decrypt_exercise_done, set_decrypt_exercise_done) = signal(false);
    let exercises_complete =
        Memo::new(move |_| encrypt_exercise_done.get() && decrypt_exercise_done.get());

    view! {
        <LessonIntro number="02" title="Shared-key encryption" eyebrow="Keep the backup private" summary="Encryption turns plaintext into ciphertext. Anyone with the shared key can recover the plaintext." />
        <section class="motivation-section content-section">
            <h2>"I want to store my backup somewhere else. How do I stop the storage provider from reading it?"</h2>
            <p class="section-copy">"The original readable data is called plaintext. Encryption combines that plaintext with a secret value called a key and produces scrambled data called ciphertext. Later, decryption uses the same key to recover the plaintext. This is called shared-key or symmetric encryption because the same key is used in both directions."</p>
            <p class="section-copy">"Encryption also creates a nonce: a fresh random value used once with that key. The nonce is not secret, but it must be saved with the ciphertext so decryption can use the same value. For this lesson, the shared key is a randomly generated 256-bit number displayed as 64 hexadecimal digits."</p>
            <div class="crypto-flow" aria-label="Shared-key encryption and decryption flow">
                <div><strong>"Encrypt"</strong><span>"Plaintext"</span><b>"+"</b><span>"Shared key"</span><b>"+"</b><span>"Fresh random nonce"</span><b>"→"</b><span>"Ciphertext"</span></div>
                <div><strong>"Decrypt"</strong><span>"Ciphertext"</span><b>"+"</b><span>"Same shared key"</span><b>"+"</b><span>"Saved nonce"</span><b>"→"</b><span>"Plaintext"</span></div>
            </div>
        </section>
        <section class="content-section"><h2>"Other common uses"</h2><ul class="use-case-list"><li><strong>"Private messages."</strong> " People or devices that share a key can exchange messages without exposing their contents to everyone carrying the traffic."</li><li><strong>"Data stored on a device."</strong> " A phone or computer can encrypt sensitive files so they remain unreadable without the key."</li></ul></section>
        <section class="workbench">
            <div class="workbench-heading"><div><h2>"Encrypt, then decrypt with the same key."</h2></div><p>"Everything happens locally in your browser. Generate a shared key, encrypt some plaintext, then use the same key to recover it."</p></div>
            <div class="mini-workbench">
                <p class="exercise-number">"Shared key"</p><h3>"A random 256-bit secret."</h3>
                <label for="shared-key">"Shared key (64 hex digits)"</label>
                <input id="shared-key" maxlength="64" prop:value=move || shared_key.get() on:input=move |ev| { let value = event_target_value(&ev); set_shared_key.set(value.clone()); set_decrypt_key.set(value); set_key_feedback.set(String::new()); } />
                <button type="button" class="button primary" on:click=move |_| { match crypto::random_hex(32) { Ok(key) => { set_shared_key.set(key.clone()); set_decrypt_key.set(key); set_key_feedback.set("Generated a new random 256-bit key.".to_owned()); } Err(_) => set_key_feedback.set("Couldn't generate random bytes in this browser.".to_owned()), } }>"Generate new key"</button>
                <p class="quiz-feedback" aria-live="polite">{move || key_feedback.get()}</p>
            </div>
            <div class="workbench-quiz">
                <p class="exercise-number">"Step 1 · Plaintext → ciphertext"</p><h3>"Encrypt a value with AES-GCM."</h3>
                <div class="mini-workbench">
                    <label for="encrypt-plaintext">"Plaintext"</label><input id="encrypt-plaintext" prop:value=move || plaintext.get() on:input=move |ev| set_plaintext.set(event_target_value(&ev)) />
                    <button type="button" class="button primary" on:click=move |_| { let key=shared_key.get(); let text=plaintext.get(); set_encrypt_feedback.set("Encrypting…".to_owned()); spawn_local(async move { match crypto::aes_gcm_encrypt(&key,&text).await { Ok((new_nonce,encrypted)) => { set_nonce.set(new_nonce.clone()); set_ciphertext.set(encrypted.clone()); set_decrypt_key.set(key); set_decrypt_nonce.set(new_nonce); set_decrypt_ciphertext.set(encrypted); set_decrypted_text.set(String::new()); set_decrypt_feedback.set(String::new()); set_encrypt_feedback.set("Encrypted. The values were copied into the decryption step below.".to_owned()); } Err(_) => set_encrypt_feedback.set("Encryption failed. The shared key must be exactly 64 hexadecimal digits.".to_owned()), } }); }>"Encrypt"</button>
                    <div class="output"><span>"NONCE · FRESH RANDOM 12 BYTES · NOT SECRET"</span><code>{move || { let value=nonce.get(); if value.is_empty(){"—".to_owned()}else{value} }}</code></div>
                    <div class="output"><span>"CIPHERTEXT + AUTHENTICATION TAG"</span><code>{move || { let value=ciphertext.get(); if value.is_empty(){"—".to_owned()}else{value} }}</code></div>
                    <p class="quiz-feedback" aria-live="polite">{move || encrypt_feedback.get()}</p>
                </div>
            </div>
            <div class="workbench-quiz">
                <p class="exercise-number">"Step 2 · Ciphertext → plaintext"</p><h3>"Decrypt it."</h3><p>"Encryption copies the key, nonce, and ciphertext here for convenience. Edit any of them and see what happens."</p>
                <div class="mini-workbench">
                    <label for="decrypt-key">"Decryption key (64 hex digits)"</label><input id="decrypt-key" maxlength="64" prop:value=move || decrypt_key.get() on:input=move |ev| set_decrypt_key.set(event_target_value(&ev)) />
                    <label for="decrypt-nonce">"Nonce (24 hex digits)"</label><input id="decrypt-nonce" maxlength="24" prop:value=move || decrypt_nonce.get() on:input=move |ev| set_decrypt_nonce.set(event_target_value(&ev)) />
                    <label for="decrypt-ciphertext">"Ciphertext"</label><input id="decrypt-ciphertext" prop:value=move || decrypt_ciphertext.get() on:input=move |ev| set_decrypt_ciphertext.set(event_target_value(&ev)) />
                    <button type="button" class="button primary" on:click=move |_| { let key=decrypt_key.get(); let nonce=decrypt_nonce.get(); let ciphertext=decrypt_ciphertext.get(); set_decrypt_feedback.set("Decrypting…".to_owned()); spawn_local(async move { match crypto::aes_gcm_decrypt(&key,&nonce,&ciphertext).await { Ok(text) => { set_decrypted_text.set(text); set_decrypt_feedback.set("Decryption succeeded.".to_owned()); } Err(_) => { set_decrypted_text.set(String::new()); set_decrypt_feedback.set("Decryption failed. The key, nonce, and ciphertext must all match exactly.".to_owned()); } } }); }>"Decrypt"</button>
                    <div class="output" aria-live="polite"><span>"RECOVERED PLAINTEXT"</span><code>{move || { let value=decrypted_text.get(); if value.is_empty(){"—".to_owned()}else{value} }}</code></div>
                    <p class="quiz-feedback" aria-live="polite">{move || decrypt_feedback.get()}</p>
                </div>
            </div>
        </section>
        <section id="encryption-exercises" class="content-section planned-quiz">
            <h2>"Exercises"</h2>
            <div class="workbench-quiz">
                <p class="exercise-number">"1 of 2 · Create ciphertext"</p>
                <h3>"Encrypt this plaintext with the provided shared key."</h3>
                <div class="output"><span>"PLAINTEXT"</span><code>{ENCRYPT_EXERCISE_PLAINTEXT}</code></div>
                <div class="output"><span>"SHARED KEY"</span><code>{EXERCISE_KEY}</code></div>
                <button type="button" class="button primary" on:click=move |_| spawn_local(async move { match crypto::aes_gcm_encrypt(EXERCISE_KEY, ENCRYPT_EXERCISE_PLAINTEXT).await { Ok((new_nonce, encrypted)) => { set_exercise_nonce.set(new_nonce); set_exercise_ciphertext.set(encrypted); set_encrypt_exercise_done.set(true); }, Err(_) => { set_exercise_nonce.set(String::new()); set_exercise_ciphertext.set("Encryption failed in this browser.".to_owned()); } } })>"Encrypt the exercise plaintext"</button>
                <div class="output"><span>"FRESH NONCE"</span><code>{move || { let value=exercise_nonce.get(); if value.is_empty(){"—".to_owned()}else{value} }}</code></div>
                <div class="output"><span>"YOUR CIPHERTEXT + AUTHENTICATION TAG"</span><code>{move || { let value=exercise_ciphertext.get(); if value.is_empty(){"—".to_owned()}else{value} }}</code></div>
            </div>
            <div class="workbench-quiz">
                <p class="exercise-number">"2 of 2 · Recover plaintext"</p>
                <h3>"Decrypt this ciphertext."</h3>
                <p>"The shared key and the nonce used during encryption are provided. Decrypt the package to reveal the original plaintext."</p>
                <div class="output"><span>"SHARED KEY"</span><code>{EXERCISE_KEY}</code></div>
                <div class="output"><span>"SAVED NONCE"</span><code>{DECRYPT_EXERCISE_NONCE}</code></div>
                <div class="output"><span>"CIPHERTEXT + AUTHENTICATION TAG"</span><code>{DECRYPT_EXERCISE_CIPHERTEXT}</code></div>
                <button type="button" class="button primary" on:click=move |_| spawn_local(async move { match crypto::aes_gcm_decrypt(EXERCISE_KEY, DECRYPT_EXERCISE_NONCE, DECRYPT_EXERCISE_CIPHERTEXT).await { Ok(value) => { let correct=value==DECRYPT_EXERCISE_PLAINTEXT; set_exercise_plaintext.set(value); set_decrypt_exercise_done.set(correct); set_decrypt_exercise_feedback.set(if correct { "Decryption succeeded. You recovered the original plaintext.".to_owned() } else { "The decrypted value was not the expected plaintext.".to_owned() }); }, Err(_) => { set_exercise_plaintext.set(String::new()); set_decrypt_exercise_feedback.set("Decryption failed. Check that the key, nonce, and ciphertext are exact.".to_owned()); } } })>"Decrypt the exercise ciphertext"</button>
                <div class="output"><span>"RECOVERED PLAINTEXT"</span><code>{move || { let value=exercise_plaintext.get(); if value.is_empty(){"—".to_owned()}else{value} }}</code></div>
                <p class="quiz-feedback" aria-live="polite">{move || decrypt_exercise_feedback.get()}</p>
            </div>
        </section>
        <section class="lesson-explanation content-section"><h2>"What did this prove?"</h2><p class="section-copy">"With AES-GCM, someone who doesn't know the shared key should not be able to read the encrypted backup. AES-GCM also detects changes to the encrypted data. The word authenticated here means that the ciphertext and tag are consistent with the key; it does not identify which person encrypted the data. Anyone holding the shared key could have done that."</p><p class="section-copy">"Now we have a new problem: if two people want to communicate, how do they safely establish or exchange that secret key?"</p><div class="principles"><article><span>"YES"</span><h3>"Confidentiality"</h3><p>"The ciphertext hides the plaintext from someone who doesn't have the key."</p></article><article><span>"YES"</span><h3>"Tamper detection"</h3><p>"AES-GCM detects ciphertext, nonce, or tag changes and refuses to return plaintext."</p></article><article><span>"NEXT"</span><h3>"Key distribution"</h3><p>"Two people still need some safe way to establish or exchange the shared secret key."</p></article></div></section>
        <LessonEnd exercises_complete exercises_id="encryption-exercises" next_href="/keypairs" next_label="Continue to 03 — Public & private keys →" />
    }
}
