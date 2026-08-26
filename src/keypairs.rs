use leptos::prelude::*;

use crate::{LessonIntro, crypto, key_math};

#[component]
pub fn KeypairsLesson() -> impl IntoView {
    let initial_private = crypto::random_hex(32).unwrap_or_else(|_| "00".repeat(32));
    let (generated_private, set_generated_private) = signal(initial_private);
    let (private_key, set_private_key) = signal(String::new());
    let (public_key, set_public_key) = signal(String::new());
    let (generation_feedback, set_generation_feedback) = signal(String::new());
    let (derivation_feedback, set_derivation_feedback) = signal(String::new());

    view! {
        <LessonIntro
            number="03"
            eyebrow="One secret value, one shareable value"
            title="Public key"
            summary="Start with a private key. From it, we can derive a public key that is safe to share. The next two lessons show how to use the pair for encryption and signatures."
        />
        <section class="content-section motivation-section">
            <h2>"What can Alice safely publish?"</h2>
            <p class="section-copy">"Alice wants anyone to be able to send her an encrypted message. With shared-key encryption, she would first need to arrange a different secret key with every sender. Publishing that shared key would not work: once everyone knows it, it is no longer secret."</p>
            <p class="section-copy">"Public-key cryptography gives Alice two related values instead. She keeps one private and publishes the other. People can use the published value without learning the secret one."</p>
        </section>
        <section class="content-section">
            <h2>"One secret value, one shareable value"</h2>
            <p class="section-copy">"Alice starts with a randomly generated private key. Her public key is calculated from that private key and can be shared freely."</p>
            <p class="section-copy">"While it is very easy to calculate a public key from a private key, it is far too expensive to go the other way and calculate the private key from only the public key."</p>
            <p class="section-copy">"This lesson focuses only on creating that pair. In the next two lessons, you'll use public and private keys first for encryption, then for signatures."</p>
        </section>
        <section class="workbench">
            <div class="workbench-heading"><div><h2>"Generate a private key, then derive its public key."</h2></div><p>"Keep the generated private key secret. Copy it into the second step to calculate the public key that is safe to share."</p></div>
            <div class="workbench-quiz">
                <p class="exercise-number">"Step 1 · Generate"</p><h3>"Generate a random private key."</h3>
                <div class="mini-workbench">
                    <div class="output"><span>"PRIVATE KEY · 32 BYTES · KEEP SECRET"</span><code>{move || generated_private.get()}</code></div>
                    <button type="button" class="button primary" on:click=move |_| { match crypto::random_hex(32) { Ok(value) => { set_generated_private.set(value); set_generation_feedback.set("Generated a fresh random private key. Copy it into the next step.".to_owned()); } Err(_) => set_generation_feedback.set("Couldn't generate random bytes in this browser.".to_owned()), } }>"Generate new private key"</button>
                    <p class="quiz-feedback" aria-live="polite">{move || generation_feedback.get()}</p>
                </div>
            </div>
            <div class="workbench-quiz">
                <p class="exercise-number">"Step 2 · Derive"</p><h3>"Derive the matching public key."</h3>
                <div class="mini-workbench">
                    <label for="private-key-input">"Private key · 64 hexadecimal digits"</label>
                    <div class="paste-input-row" data-pasteable="private key"><input id="private-key-input" maxlength="64" prop:value=move || private_key.get() on:input=move |ev| { set_private_key.set(event_target_value(&ev)); set_public_key.set(String::new()); set_derivation_feedback.set(String::new()); } /></div>
                    <button type="button" class="button primary" on:click=move |_| { match key_math::public_from_private_hex(&private_key.get()) { Ok(public) => { set_public_key.set(public); set_derivation_feedback.set("Derived the corresponding public key.".to_owned()); } Err(message) => { set_public_key.set(String::new()); set_derivation_feedback.set(message.to_owned()); } } }>"Derive public key"</button>
                    <div class="output"><span>"PUBLIC KEY · 32 BYTES · SAFE TO SHARE"</span><code>{move || { let value = public_key.get(); if value.is_empty() { "—".to_owned() } else { value } }}</code></div>
                    <p class="quiz-feedback" aria-live="polite">{move || derivation_feedback.get()}</p>
                </div>
            </div>
        </section>
    }
}
