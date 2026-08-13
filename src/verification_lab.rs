use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::{LessonEnd, curve};

#[component]
pub fn VerificationLab() -> impl IntoView {
    let (a, set_a) = signal(Option::<curve::KeyPair>::None);
    let (b, set_b) = signal(Option::<curve::KeyPair>::None);
    let (signature, set_signature) = signal(String::new());
    let (feedback, set_feedback) = signal(String::new());
    let (key_tested, set_key_tested) = signal(false);
    let (meaning_ok, set_meaning_ok) = signal(Option::<bool>::None);
    let complete = Memo::new(move |_| key_tested.get() && meaning_ok.get() == Some(true));
    const MESSAGE: &str = "The release is approved.";

    view! {
        <section class="workbench"><div class="workbench-heading"><h2>"Same message, different public keys."</h2><p>"Generate two signing keypairs. Key A signs the message; then verify against A and B."</p></div><button class="button primary" type="button" on:click=move |_| spawn_local(async move {if let (Ok(pa),Ok(pb))=(curve::generate_ed25519().await,curve::generate_ed25519().await){let private=pa.private.clone();if let Ok(sig)=curve::sign(&private,MESSAGE).await{set_a.set(Some(pa));set_b.set(Some(pb));set_signature.set(sig);set_feedback.set("Key A signed the message.".into());}}})>"Generate keys and sign"</button>
            <Show when=move||a.get().is_some()><div class="mini-workbench"><div class="output"><span>"MESSAGE"</span><code>{MESSAGE}</code></div><div class="output"><span>"KEY A"</span><code>{move||a.get().map(|p|p.public_hex).unwrap_or_default()}</code></div><div class="output"><span>"KEY B"</span><code>{move||b.get().map(|p|p.public_hex).unwrap_or_default()}</code></div><div class="output"><span>"SIGNATURE"</span><code>{move||signature.get()}</code></div></div></Show>
        </section>
        <section id="verification-exercises" class="content-section planned-quiz"><h2>"Exercises"</h2><div class="workbench-quiz"><h3>"Try both public keys."</h3><div class="hero-actions"><button type="button" on:click=move |_| {if let Some(p)=a.get(){let key=p.public;let sig=signature.get();spawn_local(async move{if matches!(curve::verify(&key,MESSAGE,&sig).await,Ok(true)){set_feedback.set("Key A: valid.".into());}});}}>"Verify with A"</button><button type="button" on:click=move |_| {if let Some(p)=b.get(){let key=p.public;let sig=signature.get();spawn_local(async move{if matches!(curve::verify(&key,MESSAGE,&sig).await,Ok(false)){set_feedback.set("Key B: not valid. The signature is tied to A's key.".into());set_key_tested.set(true);}});}}>"Verify with B"</button></div><p class="quiz-feedback">{move||feedback.get()}</p></div>
            <div class="workbench-quiz"><h3>"A signature verifies with Key A. What does that establish by itself?"</h3><div class="quiz-choice-row"><button on:click=move |_|set_meaning_ok.set(Some(true))>"This exact data matches Key A"</button><button on:click=move |_|set_meaning_ok.set(Some(false))>"A particular person wrote it"</button><button on:click=move |_|set_meaning_ok.set(Some(false))>"The statement is true"</button></div><p class="quiz-feedback">{move||match meaning_ok.get(){Some(true)=>"Exactly. Identity and truth need evidence outside the signature itself.",Some(false)=>"That conclusion needs more evidence than the signature provides.",None=>""}}</p></div></section>
        <LessonEnd exercises_complete=complete exercises_id="verification-exercises" next_href="/complete" next_label="Finish TryCrypto →" />
    }
}
