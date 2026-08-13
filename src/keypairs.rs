use leptos::prelude::*;

use crate::{LessonIntro, crypto, key_math};

#[component]
pub fn KeypairsLesson() -> impl IntoView {
    let initial_private = crypto::random_hex(32).unwrap_or_else(|_| "00".repeat(32));
    let (private_key, set_private_key) = signal(initial_private);
    let (public_key, set_public_key) = signal(String::new());
    let (feedback, set_feedback) = signal(String::new());

    view! {
        <LessonIntro
            number="03"
            eyebrow="One secret value, one shareable value"
            title="Public & private keys"
            summary="Start with a private key. From it, we can derive a public key that is safe to share."
        />
        <section class="content-section motivation-section">
            <h2>"How can I publish something useful without publishing my secret?"</h2>
            <p class="section-copy">"Public-key cryptography starts with an asymmetric relationship. The private key is secret input. The public key is calculated from it and can be shared freely."</p>
            <p class="section-copy">"Going in that direction is easy: private key → public key. Going backwards—from only the public key to the private key—is designed to be computationally infeasible."</p>
            <div class="precision-note"><strong>"One idea, different kinds of keys."</strong><p>"This workbench uses X25519 as one concrete example. The signing lesson uses Ed25519. Both use public and private keys, but real systems use keys made for the specific algorithm and purpose."</p></div>
        </section>
        <section class="workbench">
            <div class="workbench-heading"><div><h2>"Derive a public key from a private key."</h2></div><p>"Generate 32 random private bytes, or paste your own 64 hexadecimal digits. The same private key always produces the same X25519 public key."</p></div>
            <div class="mini-workbench">
                <label for="private-key-input">"Private key · 32 bytes · keep secret"</label>
                <input id="private-key-input" maxlength="64" prop:value=move || private_key.get() on:input=move |ev| { set_private_key.set(event_target_value(&ev)); set_public_key.set(String::new()); set_feedback.set(String::new()); } />
                <div class="hero-actions">
                    <button type="button" class="button ghost" on:click=move |_| { match crypto::random_hex(32) { Ok(value) => { set_private_key.set(value); set_public_key.set(String::new()); set_feedback.set("Generated a fresh random private key.".to_owned()); } Err(_) => set_feedback.set("Couldn't generate random bytes in this browser.".to_owned()), } }>"Generate random private key"</button>
                    <button type="button" class="button primary" on:click=move |_| { match key_math::public_from_private_hex(&private_key.get()) { Ok(public) => { set_public_key.set(public); set_feedback.set("Derived the corresponding public key.".to_owned()); } Err(message) => { set_public_key.set(String::new()); set_feedback.set(message.to_owned()); } } }>"Derive public key"</button>
                </div>
                <div class="output"><span>"PUBLIC KEY · 32 BYTES · SAFE TO SHARE"</span><code>{move || { let value = public_key.get(); if value.is_empty() { "—".to_owned() } else { value } }}</code></div>
                <p class="quiz-feedback" aria-live="polite">{move || feedback.get()}</p>
            </div>
        </section>
    }
}
