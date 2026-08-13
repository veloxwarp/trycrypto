use crate::{LessonEnd, curve};
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn KeypairExercises() -> impl IntoView {
    let (share_ok, set_share_ok) = signal(Option::<bool>::None);
    let (regenerated_pair, set_regenerated_pair) = signal(Option::<curve::KeyPair>::None);
    let complete = Memo::new(move |_| {
        share_ok.get() == Some(true) && regenerated_pair.get().is_some()
    });

    view! {
        <section id="keypair-exercises" class="content-section planned-quiz">
            <h2>"Exercises"</h2>
            <div class="workbench-quiz">
                <h3>"Which value is intended to be shared?"</h3>
                <div class="quiz-choice-row">
                    <button on:click=move |_| set_share_ok.set(Some(true))>"Public key"</button>
                    <button on:click=move |_| set_share_ok.set(Some(false))>"Secret key"</button>
                </div>
                <p class="quiz-feedback">
                    {move || match share_ok.get() {
                        Some(true) => "Right. The public key is meant to be shared.",
                        Some(false) => "Keep the secret key secret.",
                        None => "",
                    }}
                </p>
            </div>
            <div class="workbench-quiz">
                <h3>"Generate another pair."</h3>
                <p>"A fresh secret produces a fresh public key too. Compare these values with the pair you generated above."</p>
                <button
                    type="button"
                    on:click=move |_| spawn_local(async move {
                        if let Ok(pair) = curve::generate_x25519().await {
                            set_regenerated_pair.set(Some(pair));
                        }
                    })
                >"Generate another pair"</button>
                <Show when=move || regenerated_pair.get().is_some()>
                    <div class="mini-workbench">
                        <div class="output">
                            <span>"NEW PUBLIC KEY · 32 BYTES"</span>
                            <code>{move || regenerated_pair.get().map(|pair| pair.public_hex).unwrap_or_default()}</code>
                        </div>
                        <div class="output">
                            <span>"NEW SECRET KEY · 32 BYTES"</span>
                            <code>{move || regenerated_pair.get().map(|pair| pair.private_hex).unwrap_or_default()}</code>
                        </div>
                    </div>
                </Show>
            </div>
        </section>
        <LessonEnd
            exercises_complete=complete
            exercises_id="keypair-exercises"
            next_href="/public-key-encryption"
            next_label="Continue to 04 — Public-key encryption →"
        />
    }
}
