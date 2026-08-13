use leptos::prelude::*;
use crate::LessonIntro;

#[component]
pub fn PublicKeyEncryptionIntro() -> impl IntoView {
    view! {
        <LessonIntro number="04" eyebrow="" title="Public-key encryption" summary="A public key lets someone establish an encryption key with you without sharing that key ahead of time." />
        <section class="content-section">
            <h2>"Encrypt without a pre-shared key"</h2>
            <p class="section-copy">"X25519 lets two keypairs independently arrive at the same shared value. TryCrypto uses that result with AES-GCM to encrypt the message."</p>
        </section>
    }
}
