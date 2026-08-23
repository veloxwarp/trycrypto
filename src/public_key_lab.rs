use crate::{LessonEnd, curve};
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::CryptoKey;

#[component]
pub fn PublicKeyLab() -> impl IntoView {
    let (a, set_a) = signal(Option::<curve::KeyPair>::None);
    let (b, set_b) = signal(Option::<curve::KeyPair>::None);
    let (text, set_text) = signal("Backup metadata".to_owned());
    let (ephemeral, set_ephemeral) = signal(Option::<CryptoKey>::None);
    let (ephemeral_hex, set_ephemeral_hex) = signal(String::new());
    let (nonce, set_nonce) = signal(String::new());
    let (ciphertext, set_ciphertext) = signal(String::new());
    let (result, set_result) = signal(String::new());
    let (encrypted_seen, set_encrypted_seen) = signal(false);
    let (a_ok, set_a_ok) = signal(false);
    let (b_failed, set_b_failed) = signal(false);
    let (recipient_answer, set_recipient_answer) = signal(Option::<bool>::None);
    let complete = Memo::new(move |_| {
        encrypted_seen.get() && a_ok.get() && b_failed.get() && recipient_answer.get() == Some(true)
    });

    view! {
        <section class="workbench">
            <div class="workbench-heading"><div><h2>"Encrypt for one recipient."</h2></div><p>"The message is available immediately. Generate recipients A and B, encrypt for A, then prove which private key can recover it."</p></div>
            <div class="mini-workbench">
                <p class="exercise-number">"Recipients"</p>
                <button type="button" class="button primary" on:click=move |_| spawn_local(async move { if let (Ok(pa),Ok(pb))=(curve::generate_x25519().await,curve::generate_x25519().await){ set_a.set(Some(pa)); set_b.set(Some(pb)); set_ephemeral.set(None); set_ephemeral_hex.set(String::new()); set_nonce.set(String::new()); set_ciphertext.set(String::new()); set_result.set("Generated fresh recipients A and B.".to_owned()); set_encrypted_seen.set(false); set_a_ok.set(false); set_b_failed.set(false); set_recipient_answer.set(None); } })>"Generate recipients"</button>
                <div class="output"><span>"RECIPIENT A PUBLIC KEY"</span><code>{move || a.get().map(|p|p.public_hex).unwrap_or_else(||"—".to_owned())}</code></div>
                <div class="output"><span>"RECIPIENT B PUBLIC KEY"</span><code>{move || b.get().map(|p|p.public_hex).unwrap_or_else(||"—".to_owned())}</code></div>
            </div>
            <div class="workbench-quiz">
                <p class="exercise-number">"Step 1 · Encrypt"</p><h3>"Protect a value for recipient A."</h3>
                <div class="mini-workbench">
                    <label for="public-key-plaintext">"Plaintext"</label><input id="public-key-plaintext" prop:value=move||text.get() on:input=move|ev|set_text.set(event_target_value(&ev)) />
                    <button type="button" class="button primary" disabled=move||a.get().is_none() on:click=move |_| { if let Some(pair)=a.get(){ let public=pair.public; let message=text.get(); spawn_local(async move { match curve::seal_for(&public,&message).await { Ok(sealed)=>{ set_ephemeral.set(Some(sealed.ephemeral_public)); set_ephemeral_hex.set(sealed.ephemeral_public_hex); set_nonce.set(sealed.nonce_hex); set_ciphertext.set(sealed.ciphertext_hex); set_result.set("Encrypted for recipient A.".to_owned()); set_encrypted_seen.set(true); set_a_ok.set(false); set_b_failed.set(false); }, Err(_)=>set_result.set("Encryption failed.".to_owned()), } }); } }>"Encrypt for A"</button>
                    <p class="field-note">{move||if a.get().is_none(){"Generate recipients to enable encryption."}else{"Recipient A's public key is ready."}}</p>
                    <div class="output"><span>"SENDER TEMPORARY PUBLIC KEY"</span><code>{move||{let value=ephemeral_hex.get();if value.is_empty(){"—".to_owned()}else{value}}}</code></div>
                    <div class="output"><span>"NONCE"</span><code>{move||{let value=nonce.get();if value.is_empty(){"—".to_owned()}else{value}}}</code></div>
                    <div class="output"><span>"CIPHERTEXT"</span><code>{move||{let value=ciphertext.get();if value.is_empty(){"—".to_owned()}else{value}}}</code></div>
                </div>
            </div>
            <div class="workbench-quiz">
                <p class="exercise-number">"Step 2 · Decrypt"</p><h3>"Try both private keys."</h3>
                <div class="hero-actions">
                    <button type="button" disabled=move||!encrypted_seen.get() on:click=move |_| { if let (Some(pair),Some(eph))=(a.get(),ephemeral.get()){ let n=nonce.get(); let c=ciphertext.get(); spawn_local(async move { match curve::open_from(&pair.private,&eph,&n,&c).await { Ok(value)=>{set_result.set(format!("A recovered: {value}"));set_a_ok.set(true);}, Err(_)=>set_result.set("A could not decrypt.".to_owned()), } }); } }>"Decrypt with A"</button>
                    <button type="button" disabled=move||!encrypted_seen.get() on:click=move |_| { if let (Some(pair),Some(eph))=(b.get(),ephemeral.get()){ let n=nonce.get(); let c=ciphertext.get(); spawn_local(async move { if curve::open_from(&pair.private,&eph,&n,&c).await.is_err(){set_result.set("B failed to decrypt, as expected.".to_owned());set_b_failed.set(true);}else{set_result.set("Unexpectedly decrypted with B.".to_owned());} }); } }>"Decrypt with B"</button>
                </div><p class="quiz-feedback" aria-live="polite">{move||result.get()}</p>
            </div>
        </section>
        <section id="public-key-exercises" class="content-section planned-quiz">
            <h2>"Exercises"</h2><p class="section-copy">"Completion requires actually encrypting and trying both recipient keys above."</p>
            <div class="mini-workbench exercise-checklist"><p>{move||if encrypted_seen.get(){"✓ Encrypt a value for A"}else{"○ Encrypt a value for A"}}</p><p>{move||if a_ok.get(){"✓ Recover it with A's private key"}else{"○ Decrypt it with A's private key"}}</p><p>{move||if b_failed.get(){"✓ Observe that B's private key cannot decrypt it"}else{"○ Try decrypting it with B's private key"}}</p></div>
            <div class="workbench-quiz"><p class="exercise-number">"Question"</p><h3>"You encrypted for A. Who should be able to recover the plaintext?"</h3><div class="quiz-choice-row"><button type="button" on:click=move |_|set_recipient_answer.set(Some(true))>"Recipient A"</button><button type="button" on:click=move |_|set_recipient_answer.set(Some(false))>"Recipient B"</button><button type="button" on:click=move |_|set_recipient_answer.set(Some(false))>"Either one"</button></div><p class="quiz-feedback">{move||match recipient_answer.get(){Some(true)=>"Correct. A's private key corresponds to the public key used for this encryption.",Some(false)=>"Try both decryption buttons above and compare the results.",None=>""}}</p></div>
        </section>
        <LessonEnd exercises_complete=complete exercises_id="public-key-exercises" next_href="/signatures" next_label="Continue to 05 — Digital signatures →" />
    }
}
