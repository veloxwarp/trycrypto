use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::{LessonIntro, curve};

#[component]
pub fn KeypairsLesson() -> impl IntoView {
    let (pair, set_pair) = signal(Option::<curve::KeyPair>::None);
    view! {
        <LessonIntro number="03" eyebrow="" title="Public/private keypairs" summary="A keypair gives us one value we can share and another we keep secret." />
        <section class="content-section">
            <h2>"Generate an X25519 keypair."</h2>
            <p class="section-copy">"The public value is designed to be shared. The secret value stays under your control."</p>
            <button class="button primary" type="button" on:click=move |_| spawn_local(async move { if let Ok(value)=curve::generate_x25519().await { set_pair.set(Some(value)); } })>"Generate keypair"</button>
            <Show when=move || pair.get().is_some()>
                <div class="mini-workbench">
                    <div class="output"><span>"PUBLIC KEY · 32 BYTES"</span><code>{move || pair.get().map(|p| p.public_hex).unwrap_or_default()}</code></div>
                    <div class="output"><span>"SECRET KEY · 32 BYTES"</span><code>{move || pair.get().map(|p| p.private_hex).unwrap_or_default()}</code></div>
                </div>
            </Show>
        </section>
    }
}
