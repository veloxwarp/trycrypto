use crate::LessonIntro;
use leptos::prelude::*;

#[component]
pub fn SignaturesIntro() -> impl IntoView {
    view! {
        <LessonIntro
            number="05"
            eyebrow=""
            title="Digital signatures"
            summary="A signature lets anyone with the public key check that exact data was approved using the corresponding private key."
        />
        <section class="content-section">
            <h2>"The same public/private idea, used for a different job."</h2>
            <p class="section-copy">"Encryption is not the only thing we can build from asymmetric keys. For signatures, TryCrypto uses an Ed25519 signing keypair: the private key signs exact data, and the public key verifies the resulting signature. This is a purpose-specific signing keypair, not the X25519 encryption keypair from the previous lesson."</p>
        </section>
    }
}
