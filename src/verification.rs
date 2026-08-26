use crate::LessonIntro;
use leptos::prelude::*;

#[component]
pub fn VerificationIntro() -> impl IntoView {
    view! {
        <LessonIntro number="06" eyebrow="" title="Verification & identity" summary="Cryptography gives precise facts about keys and data; identity and truth require more evidence." />
        <section class="content-section motivation-section"><h2>"Does this key really belong to Alice?"</h2><p class="section-copy">"You receive a signed message that says it came from Alice. The signature is valid for the public key attached to the message. Is that enough to prove Alice sent it?"</p><p class="section-copy">"No. An attacker could generate a new key pair, sign the message with the new private key, and claim that the new public key belongs to Alice. The signature would be valid, but the identity claim would be false."</p></section>
        <section class="content-section"><h2>"A signature proves something precise"</h2><p class="section-copy">"A valid signature establishes that these exact bytes were signed with the private key corresponding to this public key. It does not identify the human controlling the key, and it does not prove that the signed statement is true."</p><p class="section-copy">"To connect the key to Alice, you need separate trusted evidence that Alice controls it. Once you trust that connection, future valid signatures give you useful evidence that the holder of Alice's private key signed those exact bytes."</p></section>
        <section class="content-section"><h2>"Where identity evidence comes from"</h2><ul class="use-case-list"><li><strong>"HTTPS website identity."</strong> " Your browser uses trusted certificates to connect a website name to keys it can verify."</li><li><strong>"Trusted software publishers."</strong> " A known publisher key lets you verify a release, provided you have trustworthy evidence that the key is really theirs."</li><li><strong>"A direct comparison."</strong> " For a personal key, you might compare a fingerprint with its owner in person or through another channel you already trust."</li></ul></section>
    }
}
