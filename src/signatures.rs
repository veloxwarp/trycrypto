use crate::LessonIntro;
use leptos::prelude::*;

#[component]
pub fn SignaturesIntro() -> impl IntoView {
    view! {
        <LessonIntro
            number="05"
            eyebrow=""
            title="Digital signatures"
            summary="A signature lets anyone with the public key check that the holder of the corresponding private key signed these exact bytes."
        />
        <section class="content-section">
            <h2>"The same public/private idea, used for a different job."</h2>
            <p class="section-copy">"Encryption is not the only thing we can build from asymmetric keys. For signatures, TryCrypto uses an Ed25519 signing keypair: the private key signs exact data, and the public key verifies the resulting signature. This is a purpose-specific signing keypair, not the X25519 encryption keypair from the previous lesson."</p>
            <p class="section-copy">"Notice the deliberately narrow claim: verification connects the message to a key. It does not, by itself, name the person controlling that key or prove that the signed statement is true. We'll separate those questions in the final lesson."</p>
        </section>
    }
}
