use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::CryptoKey;
use crate::{LessonEnd, curve};

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
    let (a_ok, set_a_ok) = signal(false);
    let (b_failed, set_b_failed) = signal(false);
    let complete = Memo::new(move |_| a_ok.get() && b_failed.get());

    view! {
        <section class="workbench"><div class="workbench-heading"><h2>"Encrypt for recipient A."</h2><p>"Generate two recipients, encrypt for A, then try both recipients."</p></div>
            <button type="button" class="button primary" on:click=move |_| spawn_local(async move { if let (Ok(pa),Ok(pb))=(curve::generate_x25519().await,curve::generate_x25519().await){set_a.set(Some(pa));set_b.set(Some(pb));} })>"Generate recipients"</button>
            <Show when=move || a.get().is_some()><div class="mini-workbench"><div class="output"><span>"RECIPIENT A PUBLIC KEY"</span><code>{move||a.get().map(|p|p.public_hex).unwrap_or_default()}</code></div><div class="output"><span>"RECIPIENT B PUBLIC KEY"</span><code>{move||b.get().map(|p|p.public_hex).unwrap_or_default()}</code></div>
                <label>"Plaintext"<input prop:value=move||text.get() on:input=move|ev|set_text.set(event_target_value(&ev)) /></label>
                <button type="button" on:click=move |_| { if let Some(pair)=a.get(){let public=pair.public;let message=text.get();spawn_local(async move {if let Ok(sealed)=curve::seal_for(&public,&message).await{set_ephemeral.set(Some(sealed.ephemeral_public));set_ephemeral_hex.set(sealed.ephemeral_public_hex);set_nonce.set(sealed.nonce_hex);set_ciphertext.set(sealed.ciphertext_hex);set_result.set("Encrypted for A.".into());}});} }>"Encrypt for A"</button>
                <div class="output"><span>"SENDER TEMPORARY PUBLIC KEY"</span><code>{move||ephemeral_hex.get()}</code></div><div class="output"><span>"NONCE"</span><code>{move||nonce.get()}</code></div><div class="output"><span>"CIPHERTEXT"</span><code>{move||ciphertext.get()}</code></div>
            </div></Show>
        </section>
        <section id="public-key-exercises" class="content-section planned-quiz"><h2>"Exercises"</h2><div class="workbench-quiz"><h3>"Which recipient can decrypt?"</h3><div class="hero-actions">
            <button type="button" on:click=move |_| {if let (Some(pair),Some(eph))=(a.get(),ephemeral.get()){let n=nonce.get();let c=ciphertext.get();spawn_local(async move{match curve::open_from(&pair.private,&eph,&n,&c).await{Ok(v)=>{set_result.set(format!("A recovered: {v}"));set_a_ok.set(true);},Err(_)=>set_result.set("A could not decrypt.".into())}});}}>"Decrypt with A"</button>
            <button type="button" on:click=move |_| {if let (Some(pair),Some(eph))=(b.get(),ephemeral.get()){let n=nonce.get();let c=ciphertext.get();spawn_local(async move{if curve::open_from(&pair.private,&eph,&n,&c).await.is_err(){set_result.set("B failed, as expected.".into());set_b_failed.set(true);}else{set_result.set("Unexpectedly decrypted with B.".into());}});}}>"Decrypt with B"</button>
        </div><p class="quiz-feedback">{move||result.get()}</p></div></section>
        <LessonEnd exercises_complete=complete exercises_id="public-key-exercises" next_href="/signatures" next_label="Continue to 05 — Digital signatures →" />
    }
}
