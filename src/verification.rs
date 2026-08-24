use crate::LessonIntro;
use leptos::prelude::*;

#[component]
pub fn VerificationIntro() -> impl IntoView {
    view! {
        <LessonIntro number="06" eyebrow="" title="Verification & identity" summary="Cryptography gives precise facts about keys and data; identity and truth require more evidence." />
        <section class="content-section"><h2>"What did verification actually establish?"</h2><p class="section-copy">"A valid signature establishes that these exact bytes were signed with the private key corresponding to this public key. That's a useful, precise fact. It is not yet a claim about a human being."</p><p class="section-copy">"To connect the key to Alice, you need some trusted evidence that Alice controls it: perhaps you compared the key with her in person, followed a certificate chain you trust, or relied on an account that had already been authenticated. Whether Alice's signed statement is true is another question again."</p></section>
    }
}
