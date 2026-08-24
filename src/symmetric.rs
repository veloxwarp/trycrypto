use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{LessonEnd, LessonIntro, crypto};

const ENCRYPT_EXERCISE_KEY: &str =
    "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
const ENCRYPT_EXERCISE_PLAINTEXT: &str = "Here are the nuclear codes: 1 2 3 4 5";

fn random_five_digit_code() -> String {
    let value = crypto::random_hex(3)
        .ok()
        .and_then(|hex| u32::from_str_radix(&hex, 16).ok())
        .unwrap_or(73_810);
    format!("{:05}", 10_000 + value % 90_000)
}

#[component]
pub fn SymmetricEncryptionLesson() -> impl IntoView {
    let initial_key = crypto::random_hex(32).unwrap_or_else(|_| {
        "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff".to_owned()
    });

    let (shared_key, set_shared_key) = signal(initial_key.clone());
    let (encrypt_key, set_encrypt_key) = signal(initial_key.clone());
    let (key_feedback, set_key_feedback) = signal(String::new());
    let (plaintext, set_plaintext) = signal(String::from("My important backup contents"));
    let (ciphertext, set_ciphertext) = signal(String::new());
    let (encrypt_feedback, set_encrypt_feedback) = signal(String::new());
    let (decrypt_key, set_decrypt_key) = signal(initial_key);
    let (decrypt_ciphertext, set_decrypt_ciphertext) = signal(String::new());
    let (decrypted_text, set_decrypted_text) = signal(String::new());
    let (decrypt_feedback, set_decrypt_feedback) = signal(String::new());

    let (encrypt_answer, set_encrypt_answer) = signal(String::new());
    let (encrypt_exercise_feedback, set_encrypt_exercise_feedback) = signal(String::new());
    let (encrypt_exercise_done, set_encrypt_exercise_done) = signal(false);

    let decrypt_exercise_key = crypto::random_hex(32).unwrap_or_else(|_| "42".repeat(32));
    let secret_code = random_five_digit_code();
    let secret_plaintext = format!("The code is {secret_code}");
    let (secret_ciphertext, set_secret_ciphertext) = signal(String::new());
    let (decrypt_answer, set_decrypt_answer) = signal(String::new());
    let (decrypt_exercise_feedback, set_decrypt_exercise_feedback) = signal(String::new());
    let (decrypt_exercise_done, set_decrypt_exercise_done) = signal(false);

    {
        let key = decrypt_exercise_key.clone();
        let message = secret_plaintext.clone();
        spawn_local(async move {
            match crypto::aes_gcm_encrypt(&key, &message).await {
                Ok(value) => set_secret_ciphertext.set(value),
                Err(_) => set_decrypt_exercise_feedback
                    .set("Couldn't prepare the decryption exercise in this browser.".to_owned()),
            }
        });
    }

    let exercises_complete =
        Memo::new(move |_| encrypt_exercise_done.get() && decrypt_exercise_done.get());

    view! {
        <LessonIntro number="02" title="Shared-key encryption" eyebrow="Keep the backup private" summary="Encryption turns readable plaintext into unreadable ciphertext. Anyone with the shared key can reverse the process." />

        <section class="motivation-section content-section">
            <h2>"I want to store my backup somewhere else. How do I stop the storage provider from reading it?"</h2>
            <p class="section-copy">"The original readable data is called plaintext. Encryption applies a secret value called a shared key to that plaintext and produces scrambled data called ciphertext. Later, decryption applies the same shared key to the ciphertext and recovers the plaintext."</p>
            <p class="section-copy">"This is called shared-key or symmetric encryption because the same secret key works in both directions. The key in this lesson is just one very large number: a 256-bit value with 2²⁵⁶ possible choices. That's about 1.16 × 10⁷⁷ possibilities—roughly a 1 followed by 77 zeros. Even trying a billion keys every second for the age of the universe would not come close to searching them all."</p>
            <p class="section-copy">"The encryption and decryption steps must receive exact copies of that same number. TryCrypto displays it as 64 hexadecimal digits, which makes the value easier to copy without changing it."</p>

            <div class="encryption-visual" aria-label="Shared-key encryption and decryption">
                <article>
                    <div class="visual-step-label">"ENCRYPT"</div>
                    <div class="visual-path">
                        <div class="visual-node visual-plain"><span>"PLAINTEXT"</span><strong>"Readable data"</strong></div>
                        <div class="visual-key"><span>"SHARED KEY"</span><strong>"Secret"</strong></div>
                        <div class="visual-arrow" aria-hidden="true">"→"</div>
                        <div class="visual-node visual-cipher"><span>"CIPHERTEXT"</span><strong>"Scrambled data"</strong></div>
                    </div>
                </article>
                <article>
                    <div class="visual-step-label">"DECRYPT"</div>
                    <div class="visual-path">
                        <div class="visual-node visual-cipher"><span>"CIPHERTEXT"</span><strong>"Scrambled data"</strong></div>
                        <div class="visual-key"><span>"SAME SHARED KEY"</span><strong>"Secret"</strong></div>
                        <div class="visual-arrow" aria-hidden="true">"→"</div>
                        <div class="visual-node visual-plain"><span>"PLAINTEXT"</span><strong>"Readable again"</strong></div>
                    </div>
                </article>
            </div>
        </section>

        <section class="content-section">
            <h2>"Other common uses"</h2>
            <ul class="use-case-list">
                <li><strong>"Private messages."</strong> " People or devices that share a key can exchange messages without exposing their contents to everyone carrying the traffic."</li>
                <li><strong>"Encrypted drives and ZIP files."</strong> " The contents remain unreadable until you provide the password needed to unlock them."</li>
            </ul>
        </section>

        <section id="encryption-workbench" class="workbench">
            <div class="workbench-heading"><div><h2>"Encrypt, then decrypt with the same key."</h2></div><p>"Everything happens locally in your browser. Generate a shared key, encrypt some plaintext, then use the same key to recover it."</p></div>
            <div class="mini-workbench">
                <p class="exercise-number">"Shared key"</p><h3>"A random 256-bit secret."</h3>
                <div class="output"><span>"GENERATED SHARED KEY · 64 HEX DIGITS"</span><code>{move || shared_key.get()}</code></div>
                <button type="button" class="button primary" on:click=move |_| { match crypto::random_hex(32) { Ok(key) => { set_shared_key.set(key); set_key_feedback.set("Generated a new key. Copy it into both key fields below.".to_owned()); } Err(_) => set_key_feedback.set("Couldn't generate random bytes in this browser.".to_owned()), } }>"Generate new key"</button>
                <p class="quiz-feedback" aria-live="polite">{move || key_feedback.get()}</p>
            </div>

            <div id="encrypt-step" class="workbench-quiz">
                <p class="exercise-number">"Step 1 · Plaintext → ciphertext"</p><h3>"Encrypt a value with AES-GCM."</h3>
                <div class="mini-workbench">
                    <label for="encrypt-key">"Encryption key (64 hex digits)"</label><div class="paste-input-row" data-pasteable="encryption key"><input id="encrypt-key" maxlength="64" prop:value=move || encrypt_key.get() on:input=move |ev| set_encrypt_key.set(event_target_value(&ev)) /></div>
                    <label for="encrypt-plaintext">"Plaintext"</label><input id="encrypt-plaintext" prop:value=move || plaintext.get() on:input=move |ev| set_plaintext.set(event_target_value(&ev)) />
                    <button type="button" class="button primary" on:click=move |_| { let key=encrypt_key.get(); let text=plaintext.get(); set_encrypt_feedback.set("Encrypting…".to_owned()); spawn_local(async move { match crypto::aes_gcm_encrypt(&key,&text).await { Ok(encrypted) => { set_ciphertext.set(encrypted); set_decrypted_text.set(String::new()); set_decrypt_feedback.set(String::new()); set_encrypt_feedback.set("Encrypted. Copy the generated ciphertext when you are ready to decrypt it.".to_owned()); } Err(_) => set_encrypt_feedback.set("Encryption failed. The shared key must be exactly 64 hexadecimal digits.".to_owned()), } }); }>"Encrypt"</button>
                    <div class="output" data-copyable="Ciphertext"><span>"CIPHERTEXT"</span><code>{move || { let value=ciphertext.get(); if value.is_empty(){"—".to_owned()}else{value} }}</code></div>
                    <p class="quiz-feedback" aria-live="polite">{move || encrypt_feedback.get()}</p>
                </div>
            </div>

            <div id="decrypt-step" class="workbench-quiz">
                <p class="exercise-number">"Step 2 · Ciphertext → plaintext"</p><h3>"Decrypt it."</h3><p>"Paste the same shared key and the generated ciphertext here. Change either value to see why they must match exactly."</p>
                <div class="mini-workbench">
                    <label for="decrypt-key">"Decryption key (64 hex digits)"</label><div class="paste-input-row" data-pasteable="decryption key"><input id="decrypt-key" maxlength="64" prop:value=move || decrypt_key.get() on:input=move |ev| set_decrypt_key.set(event_target_value(&ev)) /></div>
                    <label for="decrypt-ciphertext">"Ciphertext"</label><div class="paste-input-row" data-pasteable="ciphertext"><input id="decrypt-ciphertext" prop:value=move || decrypt_ciphertext.get() on:input=move |ev| set_decrypt_ciphertext.set(event_target_value(&ev)) /></div>
                    <button type="button" class="button primary" on:click=move |_| { let key=decrypt_key.get(); let ciphertext=decrypt_ciphertext.get(); set_decrypt_feedback.set("Decrypting…".to_owned()); spawn_local(async move { match crypto::aes_gcm_decrypt(&key,&ciphertext).await { Ok(text) => { set_decrypted_text.set(text); set_decrypt_feedback.set("Decryption succeeded.".to_owned()); } Err(_) => { set_decrypted_text.set(String::new()); set_decrypt_feedback.set("Decryption failed. The key and ciphertext must match exactly.".to_owned()); } } }); }>"Decrypt"</button>
                    <div class="output" data-copyable="Recovered plaintext" aria-live="polite"><span>"RECOVERED PLAINTEXT"</span><code>{move || { let value=decrypted_text.get(); if value.is_empty(){"—".to_owned()}else{value} }}</code></div>
                    <p class="quiz-feedback" aria-live="polite">{move || decrypt_feedback.get()}</p>
                </div>
            </div>

            <details class="bonus-note"><summary>"Bonus: why does the ciphertext change each time I encrypt?"</summary><p>"Encrypt the same plaintext twice with the same key and the ciphertext will still look different. The encryption process adds fresh randomness inside the ciphertext package so repeated values do not create an obvious pattern. Decryption reads that extra information automatically; you do not need to manage it yourself."</p></details>
        </section>

        <section id="encryption-exercises" class="content-section planned-quiz">
            <h2>"Exercises"</h2>
            <div class="workbench-quiz">
                <p class="exercise-number">"1 of 2 · Provide the ciphertext"</p>
                <h3>"Encrypt this plaintext with the provided shared key."</h3>
                <div class="output" data-copyable="Exercise plaintext"><span>"PLAINTEXT"</span><code>{ENCRYPT_EXERCISE_PLAINTEXT}</code></div>
                <div class="output" data-copyable="Exercise shared key"><span>"SHARED KEY"</span><code>{ENCRYPT_EXERCISE_KEY}</code></div>
                <a class="button ghost" href="#encrypt-step" on:click=move |_| { set_shared_key.set(ENCRYPT_EXERCISE_KEY.to_owned()); set_encrypt_key.set(ENCRYPT_EXERCISE_KEY.to_owned()); set_plaintext.set(ENCRYPT_EXERCISE_PLAINTEXT.to_owned()); set_ciphertext.set(String::new()); set_encrypt_feedback.set("Exercise inputs loaded. Click Encrypt, then explicitly copy the ciphertext into the answer field.".to_owned()); }>"Load these inputs in Encrypt ↑"</a>
                <label for="encrypt-exercise-answer">"Your ciphertext"</label>
                <div class="paste-input-row" data-pasteable="ciphertext answer"><textarea id="encrypt-exercise-answer" prop:value=move || encrypt_answer.get() on:input=move |ev| { set_encrypt_answer.set(event_target_value(&ev)); set_encrypt_exercise_done.set(false); set_encrypt_exercise_feedback.set(String::new()); }></textarea></div>
                <button type="button" class="button primary" on:click=move |_| { let answer=encrypt_answer.get(); set_encrypt_exercise_feedback.set("Checking…".to_owned()); spawn_local(async move { match crypto::aes_gcm_decrypt(ENCRYPT_EXERCISE_KEY,&answer).await { Ok(value) if value==ENCRYPT_EXERCISE_PLAINTEXT => { set_encrypt_exercise_done.set(true); set_encrypt_exercise_feedback.set("Correct. That ciphertext decrypts to the requested plaintext.".to_owned()); }, _ => { set_encrypt_exercise_done.set(false); set_encrypt_exercise_feedback.set("That ciphertext does not decrypt to the requested plaintext with this key.".to_owned()); } } }); }>"Check ciphertext"</button>
                <p class="quiz-feedback" aria-live="polite">{move || encrypt_exercise_feedback.get()}</p>
            </div>

            <div class="workbench-quiz">
                <p class="exercise-number">"2 of 2 · Find the secret code"</p>
                <h3>"Decrypt this ciphertext."</h3>
                <p>"Use the provided shared key and ciphertext in the workbench. The recovered plaintext contains a five-digit secret code. Enter only those five digits below."</p>
                <div class="output" data-copyable="Exercise shared key"><span>"SHARED KEY"</span><code>{decrypt_exercise_key.clone()}</code></div>
                <div class="output" data-copyable="Exercise ciphertext"><span>"CIPHERTEXT"</span><code>{move || { let value=secret_ciphertext.get(); if value.is_empty(){"Preparing…".to_owned()}else{value} }}</code></div>
                <a class="button ghost" href="#decrypt-step" on:click={let decrypt_exercise_key=decrypt_exercise_key.clone(); move |_| { set_decrypt_key.set(decrypt_exercise_key.clone()); set_decrypt_ciphertext.set(secret_ciphertext.get()); set_decrypted_text.set(String::new()); set_decrypt_feedback.set("Exercise inputs loaded. Click Decrypt, then return to enter the code.".to_owned()); }}>"Load these inputs in Decrypt ↑"</a>
                <label for="decrypt-exercise-answer">"Secret code"</label>
                <input id="decrypt-exercise-answer" inputmode="numeric" maxlength="5" prop:value=move || decrypt_answer.get() on:input=move |ev| { set_decrypt_answer.set(event_target_value(&ev)); set_decrypt_exercise_done.set(false); set_decrypt_exercise_feedback.set(String::new()); } />
                <button type="button" class="button primary" on:click={let secret_code=secret_code.clone(); move |_| { let correct=decrypt_answer.get().trim()==secret_code; set_decrypt_exercise_done.set(correct); set_decrypt_exercise_feedback.set(if correct { "Correct. You recovered the secret code.".to_owned() } else { "That is not the code in the decrypted plaintext. Try the workbench again.".to_owned() }); }}>"Check code"</button>
                <p class="quiz-feedback" aria-live="polite">{move || decrypt_exercise_feedback.get()}</p>
            </div>
        </section>

        <section class="lesson-explanation content-section">
            <h2>"What did this prove?"</h2>
            <p class="section-copy">"With AES-GCM, someone who does not know the shared key should not be able to read the encrypted backup. AES-GCM also detects changes to the ciphertext and refuses to return altered plaintext."</p>
            <div class="principles"><article><span>"YES"</span><h3>"Confidentiality"</h3><p>"The ciphertext hides the plaintext from someone who does not have the key."</p></article><article><span>"YES"</span><h3>"Tamper detection"</h3><p>"Changing the ciphertext causes decryption to fail instead of returning modified data."</p></article></div>
        </section>

        <section class="limitations-section content-section">
            <h2>"Limitations"</h2>
            <p class="section-copy">"Shared-key encryption assumes that everyone who needs access already has the same secret key. That creates a difficult problem: how do two people establish or exchange the key without exposing it to someone else? Anyone who has the shared key can also create ciphertext, so it cannot tell you which key holder encrypted a particular message."</p>
            <p class="section-copy">"The next lesson introduces public and private keys, which give us a new way to address the key-sharing problem."</p>
        </section>

        <LessonEnd exercises_complete exercises_id="encryption-exercises" next_href="/public-key" next_label="Continue to 03 — Public key →" />
    }
}
