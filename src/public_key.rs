use crate::LessonIntro;
use leptos::prelude::*;

#[component]
pub fn PublicKeyEncryptionIntro() -> impl IntoView {
    view! {
        <LessonIntro
            number="04"
            eyebrow=""
            title="Public-key encryption"
            summary="A public key lets someone encrypt for you without first arranging a shared secret with you."
        />
        <section class="content-section">
            <h2>"Now use the public half."</h2>
            <p class="section-copy">"In the last lesson, we started with a private key and derived a public key that could be shared. Here, the sender creates a temporary X25519 keypair and combines its private key with the recipient's public key. The recipient combines their private key with the sender's temporary public key. Both calculations produce the same shared value, without either private key being published."</p>
            <p class="section-copy">"TryCrypto turns that shared value into an AES-GCM key and encrypts the message. This protects the message for the chosen recipient, but the temporary sender key does not identify who sent it. Encryption and sender identity are separate questions."</p>
        </section>
    }
}
