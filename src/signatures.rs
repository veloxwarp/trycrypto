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
        <section class="content-section motivation-section">
            <h2>"Did this update really come from the publisher?"</h2>
            <p class="section-copy">"You download a software update from a website. A hash can tell you whether the file matches a hash shown on that same website—but an attacker who replaces the download may be able to replace the displayed hash too."</p>
            <p class="section-copy">"The publisher needs a way to approve the exact update without giving everyone the secret needed to create that approval. A digital signature does that: the publisher signs with a private key, and everyone else checks the result with the publisher's public key."</p>
        </section>
        <section class="content-section">
            <h2>"Sign with the private key. Verify with the public key."</h2>
            <p class="section-copy">"A private key can create a signature for an exact message. Anyone with the matching public key can check that signature. Changing the message, signature, or public key causes verification to fail."</p>
            <p class="section-copy">"Notice the deliberately narrow claim: verification connects the message to a key. It does not, by itself, name the person controlling that key or prove that the signed statement is true. We'll separate those questions in the final lesson."</p>
        </section>
        <section class="content-section"><h2>"Other common use cases"</h2><ul class="use-case-list"><li><strong>"Signed documents and requests."</strong> " A signature can bind exact bytes to a key without hiding their contents."</li><li><strong>"Messages that need proof of origin."</strong> " A recipient can check that a message was signed by the holder of an expected key, even though anyone can read it."</li></ul></section>
    }
}
