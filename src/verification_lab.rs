use crate::{LessonEnd, curve};
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn VerificationLab() -> impl IntoView {
    let (a, set_a) = signal(Option::<curve::KeyPair>::None);
    let (b, set_b) = signal(Option::<curve::KeyPair>::None);
    let (signature, set_signature) = signal(String::new());
    let (feedback, set_feedback) = signal(String::new());
    let (a_tested, set_a_tested) = signal(false);
    let (b_tested, set_b_tested) = signal(false);
    let (meaning_ok, set_meaning_ok) = signal(Option::<bool>::None);
    let keys_ready = Memo::new(move |_| a.get().is_some() && b.get().is_some());
    let complete =
        Memo::new(move |_| a_tested.get() && b_tested.get() && meaning_ok.get() == Some(true));
    const MESSAGE: &str = "The release is approved.";

    view! {
        <section class="workbench">
            <div class="workbench-heading">
                <h2>"Same message, different public keys."</h2>
                <p>"Generate two signing keypairs. Key A signs the message. Then hold the message and signature constant while you try each public key."</p>
            </div>
            <button
                class="button primary"
                type="button"
                on:click=move |_| {
                    set_feedback.set("Generating two signing keys…".to_owned());
                    spawn_local(async move {
                        match (curve::generate_ed25519().await, curve::generate_ed25519().await) {
                            (Ok(pa), Ok(pb)) => {
                                let private = pa.private.clone();
                                match curve::sign(&private, MESSAGE).await {
                                    Ok(sig) => {
                                        set_a.set(Some(pa));
                                        set_b.set(Some(pb));
                                        set_signature.set(sig);
                                        set_a_tested.set(false);
                                        set_b_tested.set(false);
                                        set_meaning_ok.set(None);
                                        set_feedback.set("Key A signed the message. Now test both public keys below.".to_owned());
                                    }
                                    Err(_) => set_feedback.set("Signing failed in this browser.".to_owned()),
                                }
                            }
                            _ => set_feedback.set("Couldn't generate Ed25519 keys in this browser.".to_owned()),
                        }
                    });
                }
            >"Generate keys and sign"</button>
            <p class="quiz-feedback" aria-live="polite">{move || feedback.get()}</p>
            <Show when=move || keys_ready.get()>
                <div class="mini-workbench">
                    <div class="output"><span>"MESSAGE"</span><code>{MESSAGE}</code></div>
                    <div class="output"><span>"KEY A · SIGNED THE MESSAGE"</span><code>{move || a.get().map(|p| p.public_hex).unwrap_or_default()}</code></div>
                    <div class="output"><span>"KEY B · DID NOT SIGN THE MESSAGE"</span><code>{move || b.get().map(|p| p.public_hex).unwrap_or_default()}</code></div>
                    <div class="output"><span>"SIGNATURE"</span><code>{move || signature.get()}</code></div>
                </div>
            </Show>
        </section>

        <section id="verification-exercises" class="content-section planned-quiz">
            <h2>"Exercises"</h2>
            <div class="workbench-quiz">
                <h3>"Try both public keys."</h3>
                <p>"Only the public key corresponding to the signing key should validate this signature."</p>
                <div class="hero-actions">
                    <button
                        type="button"
                        disabled=move || !keys_ready.get()
                        on:click=move |_| {
                            if let Some(p) = a.get() {
                                let key = p.public;
                                let sig = signature.get();
                                spawn_local(async move {
                                    match curve::verify(&key, MESSAGE, &sig).await {
                                        Ok(true) => {
                                            set_a_tested.set(true);
                                            set_feedback.set("Key A: valid. This public key corresponds to the private key that signed the message.".to_owned());
                                        }
                                        _ => set_feedback.set("Key A verification failed unexpectedly.".to_owned()),
                                    }
                                });
                            }
                        }
                    >"Verify with A"</button>
                    <button
                        type="button"
                        disabled=move || !keys_ready.get()
                        on:click=move |_| {
                            if let Some(p) = b.get() {
                                let key = p.public;
                                let sig = signature.get();
                                spawn_local(async move {
                                    match curve::verify(&key, MESSAGE, &sig).await {
                                        Ok(false) => {
                                            set_b_tested.set(true);
                                            set_feedback.set("Key B: not valid. The same message and signature do not verify under a different public key.".to_owned());
                                        }
                                        _ => set_feedback.set("Key B produced an unexpected verification result.".to_owned()),
                                    }
                                });
                            }
                        }
                    >"Verify with B"</button>
                </div>
                <div class="mini-workbench exercise-checklist">
                    <p>{move || if a_tested.get() { "✓ Verify with Key A" } else { "○ Verify with Key A" }}</p>
                    <p>{move || if b_tested.get() { "✓ Verify with Key B" } else { "○ Verify with Key B" }}</p>
                </div>
                <p class="quiz-feedback" aria-live="polite">{move || feedback.get()}</p>
            </div>

            <div class="workbench-quiz">
                <h3>"A signature verifies with Key A. What does that establish by itself?"</h3>
                <div class="quiz-choice-row">
                    <button type="button" on:click=move |_| set_meaning_ok.set(Some(true))>"These exact bytes were signed by Key A's corresponding private key"</button>
                    <button type="button" on:click=move |_| set_meaning_ok.set(Some(false))>"A particular person wrote it"</button>
                    <button type="button" on:click=move |_| set_meaning_ok.set(Some(false))>"The statement is true"</button>
                </div>
                <p class="quiz-feedback">{move || match meaning_ok.get() {
                    Some(true) => "Exactly. Identity and truth need evidence outside the signature itself.",
                    Some(false) => "That conclusion needs more evidence than the signature provides.",
                    None => "",
                }}</p>
            </div>
        </section>
        <LessonEnd exercises_complete=complete exercises_id="verification-exercises" next_href="/complete" next_label="Finish TryCrypto →" />
    }
}
