use leptos::prelude::*;

use crate::{LessonEnd, key_math};

#[component]
pub fn KeypairExercises() -> impl IntoView {
    let challenge_private = "01".repeat(32);
    let correct_public = key_math::public_from_private_hex(&challenge_private).unwrap_or_default();
    let decoy_one = key_math::public_from_private_hex(&"02".repeat(32)).unwrap_or_default();
    let decoy_two = key_math::public_from_private_hex(&"03".repeat(32)).unwrap_or_default();

    let (private_input, set_private_input) = signal(String::new());
    let (derived_public, set_derived_public) = signal(String::new());
    let (derived_challenge, set_derived_challenge) = signal(false);
    let (share_ok, set_share_ok) = signal(Option::<bool>::None);
    let (match_ok, set_match_ok) = signal(Option::<bool>::None);
    let complete = Memo::new(move |_| share_ok.get() == Some(true) && match_ok.get() == Some(true));

    view! {
        <section id="keypair-exercises" class="content-section planned-quiz">
            <h2>"Exercises"</h2>

            <div class="workbench-quiz">
                <p class="exercise-number">"1 of 2 · What gets shared?"</p>
                <h3>"Which value is intended to leave your device?"</h3>
                <div class="quiz-choice-row">
                    <button type="button" on:click=move |_| set_share_ok.set(Some(true))>"The public key"</button>
                    <button type="button" on:click=move |_| set_share_ok.set(Some(false))>"The private key"</button>
                </div>
                <p class="quiz-feedback">{move || match share_ok.get() {
                    Some(true) => "Right. Share the public key; protect the private key.",
                    Some(false) => "The private key is the secret. The public key is the shareable value derived from it.",
                    None => "",
                }}</p>
            </div>

            <div class="workbench-quiz">
                <p class="exercise-number">"2 of 2 · Find the matching public key"</p>
                <h3>"Derive the public key for the private key below."</h3>
                <p>"Copy the provided private key into the derivation field, then match its public key against the three candidates."</p>
                <div class="output"><span>"PROVIDED PRIVATE KEY"</span><code>{challenge_private.clone()}</code></div>
                <div class="mini-workbench">
                    <label for="challenge-private-key">"Private key"</label>
                    <div class="paste-input-row" data-pasteable="private key"><input
                        id="challenge-private-key"
                        maxlength="64"
                        prop:value=move || private_input.get()
                        on:input=move |ev| {
                            set_private_input.set(event_target_value(&ev));
                            set_derived_public.set(String::new());
                            set_derived_challenge.set(false);
                            set_match_ok.set(None);
                        }
                    /></div>
                    <button type="button" class="button primary" on:click=move |_| {
                        let value = private_input.get();
                        match key_math::public_from_private_hex(&value) {
                            Ok(public) => {
                                set_derived_public.set(public);
                                set_derived_challenge.set(value == "01".repeat(32));
                            }
                            Err(_) => {
                                set_derived_public.set(String::new());
                                set_derived_challenge.set(false);
                            }
                        }
                    }>"Derive public key"</button>
                    <div class="output">
                        <span>"DERIVED PUBLIC KEY"</span>
                        <code>{move || {
                            let value = derived_public.get();
                            if value.is_empty() { "—".to_owned() } else { value }
                        }}</code>
                    </div>
                </div>
                <div class="quiz-choice-row key-choice-row">
                    <button type="button" on:click=move |_| {
                        if derived_challenge.get() { set_match_ok.set(Some(false)); }
                    }><span class="candidate-label">"A"</span><code>{decoy_one.clone()}</code></button>
                    <button type="button" on:click=move |_| {
                        if derived_challenge.get() { set_match_ok.set(Some(true)); }
                    }><span class="candidate-label">"B"</span><code>{correct_public.clone()}</code></button>
                    <button type="button" on:click=move |_| {
                        if derived_challenge.get() { set_match_ok.set(Some(false)); }
                    }><span class="candidate-label">"C"</span><code>{decoy_two.clone()}</code></button>
                </div>
                <p class="quiz-feedback">{move || {
                    if !derived_challenge.get() {
                        "Derive the public key from the challenge value first."
                    } else {
                        match match_ok.get() {
                            Some(true) => "Correct. The same private bytes deterministically produce that public key.",
                            Some(false) => "Compare the derived value with the candidates exactly.",
                            None => "Now choose the candidate that exactly matches the derived value.",
                        }
                    }
                }}</p>
            </div>
        </section>
        <section class="content-section lesson-explanation">
            <h2>"Now the asymmetry becomes useful."</h2>
            <p class="section-copy">"You can publish a public key without publishing the private key that created it. Next we'll use public/private keys to establish an encryption key. After that we'll use a signing-specific keypair to create signatures."</p>
        </section>
        <LessonEnd
            exercises_complete=complete
            exercises_id="keypair-exercises"
            next_href="/public-key-encryption"
            next_label="Continue to 04 — Public-key encryption →"
        />
    }
}
