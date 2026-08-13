use leptos::prelude::*;
use crate::LessonIntro;

#[component]
pub fn SignaturesIntro() -> impl IntoView {
    view! {
        <LessonIntro number="05" eyebrow="" title="Digital signatures" summary="A signature lets anyone with the public key check that exact data was approved using the corresponding secret key." />
        <section class="content-section"><h2>"Approve exact data without revealing the secret key"</h2><p class="section-copy">"TryCrypto uses Ed25519 here, the same signature algorithm used by the protocol this course is leading toward. Sign a message, verify it with the public key, then change the message and try again."</p></section>
    }
}
