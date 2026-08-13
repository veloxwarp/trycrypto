use crate::LessonIntro;
use leptos::prelude::*;

#[component]
pub fn PublicKeyEncryptionIntro() -> impl IntoView {
    view! {
        <LessonIntro
            number="04"
            eyebrow=""
            title="Public-key encryption"
            summary="A public key lets someone establish an encryption key with you without sharing that key ahead of time."
        />
        <section class="content-section">
            <h2>"Now use the public half."</h2>
            <p class="section-copy">"In the last lesson, we started with a private key and derived a public key that could be shared. X25519 lets two such keypairs independently arrive at the same shared value without publishing either private key. TryCrypto uses that result with AES-GCM to encrypt the message."</p>
        </section>
    }
}
