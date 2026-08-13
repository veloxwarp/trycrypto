use leptos::prelude::*;
use crate::LessonIntro;

#[component]
pub fn VerificationIntro() -> impl IntoView {
    view! {
        <LessonIntro number="06" eyebrow="" title="Verification & identity" summary="Cryptography gives precise facts about keys and data; identity and truth require more evidence." />
        <section class="content-section"><h2>"What did verification actually establish?"</h2><p class="section-copy">"A valid signature connects exact data to a public key. Connecting that key to a person or deciding whether the statement is true takes additional evidence."</p></section>
    }
}
