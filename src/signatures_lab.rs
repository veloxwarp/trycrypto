use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::{LessonEnd, curve};

#[component]
pub fn SignaturesLab() -> impl IntoView {
    let (pair, set_pair) = signal(Option::<curve::KeyPair>::None);
    let (message, set_message) = signal("Release 1.0 is approved.".to_owned());
    let (signature, set_signature) = signal(String::new());
    let (feedback, set_feedback) = signal(String::new());
    let (valid_seen, set_valid_seen) = signal(false);
    let (changed_seen, set_changed_seen) = signal(false);
    let complete = Memo::new(move |_| valid_seen.get() && changed_seen.get());

    view! {
        <section class="workbench"><div class="workbench-heading"><h2>"Sign and verify with Ed25519."</h2><p>"The message is part of the signature. Change even one character and verification should fail."</p></div>
            <button type="button" class="button primary" on:click=move |_| spawn_local(async move {if let Ok(value)=curve::generate_ed25519().await{set_pair.set(Some(value));set_signature.set(String::new());set_feedback.set("Keypair generated.".into());}})>"Generate signing keypair"</button>
            <Show when=move||pair.get().is_some()><div class="mini-workbench"><div class="output"><span>"PUBLIC KEY · 32 BYTES"</span><code>{move||pair.get().map(|p|p.public_hex).unwrap_or_default()}</code></div><label>"Message"<input prop:value=move||message.get() on:input=move|ev|set_message.set(event_target_value(&ev)) /></label><button type="button" on:click=move |_| {if let Some(p)=pair.get(){let key=p.private;let msg=message.get();spawn_local(async move{if let Ok(sig)=curve::sign(&key,&msg).await{set_signature.set(sig);set_feedback.set("Signed the exact message above.".into());}});}}>"Sign message"</button><div class="output"><span>"SIGNATURE · 64 BYTES"</span><code>{move||signature.get()}</code></div></div></Show>
        </section>
        <section id="signature-exercises" class="content-section planned-quiz"><h2>"Exercises"</h2><div class="workbench-quiz"><h3>"Verify the current message."</h3><button type="button" on:click=move |_| {if let Some(p)=pair.get(){let key=p.public;let msg=message.get();let sig=signature.get();spawn_local(async move{match curve::verify(&key,&msg,&sig).await{Ok(true)=>{set_feedback.set("Valid signature for this exact message.".into());set_valid_seen.set(true);},Ok(false)=>set_feedback.set("Signature does not match this message.".into()),Err(_)=>set_feedback.set("Verification failed.".into())}});}}>"Verify"</button></div>
            <div class="workbench-quiz"><h3>"Now alter the message and verify again."</h3><button type="button" on:click=move |_| set_message.update(|m|m.push('!'))>"Add one character"</button><button type="button" on:click=move |_| {if let Some(p)=pair.get(){let key=p.public;let msg=message.get();let sig=signature.get();spawn_local(async move{if matches!(curve::verify(&key,&msg,&sig).await,Ok(false)){set_feedback.set("Changed message: signature no longer valid.".into());set_changed_seen.set(true);}});}}>"Verify changed message"</button></div><p class="quiz-feedback">{move||feedback.get()}</p></section>
        <LessonEnd exercises_complete=complete exercises_id="signature-exercises" next_href="/verification" next_label="Continue to 06 — Verification & identity →" />
    }
}
