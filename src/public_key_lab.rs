use crate::{LessonEnd, curve, key_math};
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn PublicKeyLab() -> impl IntoView {
    let maya_message = "Meet me at 10:30 by the north entrance.".to_owned();
    let incoming_code = "83810";

    let (recipient, set_recipient) = signal(Option::<curve::KeyPair>::None);
    let (private_input, set_private_input) = signal(String::new());
    let (derived_public, set_derived_public) = signal(String::new());
    let (encrypt_public, set_encrypt_public) = signal(String::new());
    let (plaintext, set_plaintext) = signal("Backup metadata".to_owned());
    let (ciphertext, set_ciphertext) = signal(String::new());
    let (decrypt_private, set_decrypt_private) = signal(String::new());
    let (decrypt_ciphertext, set_decrypt_ciphertext) = signal(String::new());
    let (recovered, set_recovered) = signal(String::new());
    let (feedback, set_feedback) = signal(String::new());
    let (encrypted_seen, set_encrypted_seen) = signal(false);
    let (decrypted_seen, set_decrypted_seen) = signal(false);
    let (maya_pair, set_maya_pair) = signal(Option::<curve::KeyPair>::None);
    let (maya_answer, set_maya_answer) = signal(String::new());
    let (maya_feedback, set_maya_feedback) = signal(String::new());
    let (inbound_pair, set_inbound_pair) = signal(Option::<curve::KeyPair>::None);
    let (inbound_ciphertext, set_inbound_ciphertext) = signal(String::new());
    let (code_answer, set_code_answer) = signal(String::new());
    let (code_feedback, set_code_feedback) = signal(String::new());
    let workbench_complete = Memo::new(move |_| encrypted_seen.get() && decrypted_seen.get());

    spawn_local(async move {
        if let Ok(pair) = curve::generate_x25519().await {
            set_maya_pair.set(Some(pair));
        }
        if let Ok(pair) = curve::generate_x25519().await
            && let Ok(sealed) =
                curve::seal_for(&pair.public, &format!("The code is {incoming_code}")).await
        {
            set_inbound_ciphertext.set(sealed.ciphertext_hex);
            set_inbound_pair.set(Some(pair));
        }
    });

    view! {
        <section class="workbench">
            <div class="workbench-heading"><div><h2>"Encrypt with a public key, then decrypt with its private key."</h2></div><p>"Follow the four steps in order. Generated values are outputs; the next step always starts with an empty field for you to fill."</p></div>
            <div class="workbench-quiz">
                <p class="exercise-number">"Step 1 · Generate"</p><h3>"Generate a recipient's private key."</h3>
                <div class="mini-workbench">
                    <button type="button" class="button primary" on:click=move |_| spawn_local(async move {
                        match curve::generate_x25519().await {
                            Ok(pair) => { set_recipient.set(Some(pair)); set_private_input.set(String::new()); set_derived_public.set(String::new()); set_encrypt_public.set(String::new()); set_ciphertext.set(String::new()); set_decrypt_private.set(String::new()); set_decrypt_ciphertext.set(String::new()); set_recovered.set(String::new()); set_encrypted_seen.set(false); set_decrypted_seen.set(false); set_feedback.set("Generated a private key. Copy it into the next step.".to_owned()); }
                            Err(_) => set_feedback.set("Couldn't generate a private key in this browser.".to_owned()),
                        }
                    })>"Generate private key"</button>
                    <div class="output"><span>"RECIPIENT PRIVATE KEY · KEEP SECRET"</span><code>{move || recipient.get().map(|p| p.private_hex).unwrap_or_else(|| "—".to_owned())}</code></div>
                </div>
            </div>
            <div class="workbench-quiz">
                <p class="exercise-number">"Step 2 · Derive"</p><h3>"Derive the recipient's public key."</h3>
                <div class="mini-workbench">
                    <label for="derive-private-key">"Private key"</label><div class="paste-input-row" data-pasteable="private key"><input id="derive-private-key" maxlength="64" prop:value=move || private_input.get() on:input=move |ev| { set_private_input.set(event_target_value(&ev)); set_derived_public.set(String::new()); } /></div>
                    <button type="button" class="button primary" on:click=move |_| match key_math::public_from_private_hex(&private_input.get()) { Ok(value) => { set_derived_public.set(value); set_feedback.set("Derived the public key. Copy it into the encryption step.".to_owned()); }, Err(message) => set_feedback.set(message.to_owned()) }>"Derive public key"</button>
                    <div class="output"><span>"RECIPIENT PUBLIC KEY · SAFE TO SHARE"</span><code>{move || { let value = derived_public.get(); if value.is_empty() { "—".to_owned() } else { value } }}</code></div>
                </div>
            </div>
            <div class="workbench-quiz">
                <p class="exercise-number">"Step 3 · Encrypt"</p><h3>"Encrypt a message for that public key."</h3>
                <div class="mini-workbench">
                    <label for="recipient-public-key">"Recipient public key"</label><div class="paste-input-row" data-pasteable="recipient public key"><input id="recipient-public-key" maxlength="64" prop:value=move || encrypt_public.get() on:input=move |ev| set_encrypt_public.set(event_target_value(&ev)) /></div>
                    <label for="public-key-plaintext">"Plaintext"</label><input id="public-key-plaintext" prop:value=move || plaintext.get() on:input=move |ev| set_plaintext.set(event_target_value(&ev)) />
                    <button type="button" class="button primary" on:click=move |_| { let public = encrypt_public.get(); let message = plaintext.get(); spawn_local(async move { match curve::x25519_public_from_hex(&public).await { Ok(key) => match curve::seal_for(&key, &message).await { Ok(sealed) => { let matches_workbench = derived_public.get() == public.trim(); set_ciphertext.set(sealed.ciphertext_hex); if matches_workbench { set_encrypted_seen.set(true); set_decrypted_seen.set(false); } set_recovered.set(String::new()); set_feedback.set(if matches_workbench { "Encrypted. Copy the ciphertext and private key into the final step." } else { "Encrypted. To complete the workbench, use the public key derived in step 2." }.to_owned()); }, Err(_) => set_feedback.set("Encryption failed.".to_owned()) }, Err(_) => set_feedback.set("Paste exactly 64 hexadecimal digits for the recipient's public key.".to_owned()) } }); } >"Encrypt"</button>
                    <div class="output"><span>"CIPHERTEXT"</span><code>{move || { let value = ciphertext.get(); if value.is_empty() { "—".to_owned() } else { value } }}</code></div>
                </div>
            </div>
            <div class="workbench-quiz">
                <p class="exercise-number">"Step 4 · Decrypt"</p><h3>"Decrypt with the matching private key."</h3>
                <div class="mini-workbench">
                    <label for="recipient-private-key">"Recipient private key"</label><div class="paste-input-row" data-pasteable="recipient private key"><input id="recipient-private-key" maxlength="64" prop:value=move || decrypt_private.get() on:input=move |ev| set_decrypt_private.set(event_target_value(&ev)) /></div>
                    <label for="public-key-ciphertext">"Ciphertext"</label><div class="paste-input-row" data-pasteable="ciphertext"><textarea id="public-key-ciphertext" prop:value=move || decrypt_ciphertext.get() on:input=move |ev| set_decrypt_ciphertext.set(event_target_value(&ev))></textarea></div>
                    <button type="button" class="button primary" on:click=move |_| { let supplied = decrypt_private.get(); let selected = recipient.get().filter(|pair| pair.private_hex == supplied.trim()).map(|pair| (pair, true)).or_else(|| inbound_pair.get().filter(|pair| pair.private_hex == supplied.trim()).map(|pair| (pair, false))); let Some((pair, workbench_key)) = selected else { set_feedback.set("Copy a private key from step 1 or the incoming-message exercise into this field.".to_owned()); return; }; let input = decrypt_ciphertext.get(); let expected = ciphertext.get(); spawn_local(async move { match curve::open_from(&pair.private, &input).await { Ok(value) => { let complete = workbench_key && input.trim() == expected && value == plaintext.get(); set_recovered.set(value); if complete { set_decrypted_seen.set(true); } set_feedback.set(if complete { "Decrypted successfully. You completed the workbench." } else if workbench_key { "Decrypted successfully. To complete the workbench, use the ciphertext from step 3." } else { "Decrypted successfully. Read the five-digit code, then check it below." }.to_owned()); }, Err(_) => { set_recovered.set(String::new()); set_feedback.set("Decryption failed. The private key and ciphertext must match.".to_owned()); } } }); } >"Decrypt"</button>
                    <div class="output"><span>"RECOVERED PLAINTEXT"</span><code>{move || { let value = recovered.get(); if value.is_empty() { "—".to_owned() } else { value } }}</code></div>
                </div>
                <p class="quiz-feedback" aria-live="polite">{move || feedback.get()}</p>
            </div>
        </section>
        <section id="public-key-exercises" class="content-section planned-quiz">
            <h2>"Exercise 1: encrypt for a specific recipient"</h2>
            <p class="section-copy">"Maya has published this public key. Copy it and the message into the encryption step, then paste the resulting ciphertext here. Maya will check it using her private key. Do not use that private key yourself: the sender does not need it."</p>
            <div class="output"><span>"MAYA'S PUBLIC KEY"</span><code>{move || maya_pair.get().map(|pair| pair.public_hex).unwrap_or_else(|| "Preparing…".to_owned())}</code></div>
            <div class="output"><span>"MESSAGE FOR MAYA"</span><code>{maya_message.clone()}</code></div>
            <label for="maya-ciphertext">"Encrypted message for Maya"</label><div class="paste-input-row" data-pasteable="encrypted message for Maya"><textarea id="maya-ciphertext" prop:value=move || maya_answer.get() on:input=move |ev| set_maya_answer.set(event_target_value(&ev))></textarea></div>
            <button type="button" class="button primary" on:click=move |_| { let Some(pair) = maya_pair.get() else { set_maya_feedback.set("Maya's key is still being prepared.".to_owned()); return; }; let answer = maya_answer.get(); let expected = maya_message.clone(); spawn_local(async move { match curve::open_from(&pair.private, &answer).await { Ok(value) if value == expected => set_maya_feedback.set("✓ Maya decrypted your message. You encrypted it for the right recipient.".to_owned()), Ok(_) => set_maya_feedback.set("That ciphertext opens for Maya, but it contains a different message. Encrypt the message shown above.".to_owned()), Err(_) => set_maya_feedback.set("Maya could not decrypt that ciphertext. Check that you copied the complete generated ciphertext.".to_owned()), } }); } >"Check encrypted message"</button>
            <p class="quiz-feedback" aria-live="polite">{move || maya_feedback.get()}</p>
            <h2>"Exercise 2: decrypt an incoming message"</h2>
            <p class="section-copy">"Someone sent you this encrypted message. Copy its private key and ciphertext into the decryption step above. The recovered plaintext contains a five-digit code; enter that code below."</p>
            <div class="output"><span>"YOUR PRIVATE KEY · KEEP SECRET"</span><code>{move || inbound_pair.get().map(|pair| pair.private_hex).unwrap_or_else(|| "Preparing…".to_owned())}</code></div>
            <div class="output"><span>"INCOMING CIPHERTEXT"</span><code>{move || { let value = inbound_ciphertext.get(); if value.is_empty() { "Preparing…".to_owned() } else { value } }}</code></div>
            <label for="incoming-code">"Five-digit code"</label>
            <form class="quiz-answer-row" on:submit=move |event| {
                event.prevent_default();
                match code_answer.get().trim() == incoming_code {
                    true => set_code_feedback.set("✓ Correct. You used your private key to recover the secret code.".to_owned()),
                    false => set_code_feedback.set("Not quite. Decrypt the incoming ciphertext above, then enter the five-digit code from the recovered plaintext.".to_owned()),
                }
            }>
                <input id="incoming-code" inputmode="numeric" autocomplete="one-time-code" maxlength="5" prop:value=move || code_answer.get() on:input=move |ev| set_code_answer.set(event_target_value(&ev)) />
                <button type="submit">"Check code"</button>
            </form>
            <p class="quiz-feedback" aria-live="polite">{move || code_feedback.get()}</p>
        </section>
        <section class="content-section lesson-explanation">
            <h2>"What this still does not tell us"</h2>
            <p class="section-copy">"The ciphertext can be opened by the holder of the matching private key, but encryption alone does not tell them who sent it. The next lesson introduces signatures, which answer a different question: which private key signed these exact bytes?"</p>
        </section>
        <LessonEnd exercises_complete=workbench_complete exercises_id="public-key-exercises" next_href="/digital-signatures" next_label="Continue to 05 — Digital signatures →" />
    }
}
