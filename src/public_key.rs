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
        <section class="content-section motivation-section">
            <h2>"How can you send Alice a secret?"</h2>
            <p class="section-copy">"You want to send Alice a private message, but the two of you have never exchanged a shared key. You could arrange a separate secure meeting just to hand her one, but then you would need a safe way to do that before you could send the original message."</p>
            <p class="section-copy">"Alice's public key removes that catch. She can publish it anywhere. You use it to encrypt the message, and the resulting ciphertext can only be decrypted with Alice's private key."</p>
        </section>
        <section class="content-section">
            <h2>"Encrypt with the public key. Decrypt with the private key."</h2>
            <p class="section-copy">"The sender needs only Alice's public key and the plaintext. Encryption combines them to produce ciphertext. Alice supplies her matching private key to decryption and recovers the original plaintext."</p>
            <p class="section-copy">"The sender never receives Alice's private key. Choosing the right public key still matters: ciphertext encrypted with Bob's public key is intended for Bob's private key, not Alice's."</p>
        </section>
        <section class="content-section"><h2>"Other common use cases"</h2><ul class="use-case-list"><li><strong>"HTTPS—the secure web."</strong> " The S in HTTPS stands for secure. Public-key cryptography helps your browser confirm the website and establish encryption keys; fast shared-key encryption then protects the data you send and receive."</li><li><strong>"Encrypted email and messaging."</strong> " A sender can protect a message using a recipient's published key."</li><li><strong>"Sharing a secret file with one person."</strong> " You can encrypt a copy using their public key without giving them your own private key."</li></ul></section>
    }
}
