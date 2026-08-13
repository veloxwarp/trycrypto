use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::{LessonEnd, curve};

#[component]
pub fn KeypairExercises() -> impl IntoView {
    let (share_ok, set_share_ok) = signal(Option::<bool>::None);
    let (regenerated, set_regenerated) = signal(false);
    let complete = Memo::new(move |_| share_ok.get() == Some(true) && regenerated.get());
    view! {
        <section id="keypair-exercises" class="content-section planned-quiz"><h2>"Exercises"</h2>
            <div class="workbench-quiz"><h3>"Which value is intended to be shared?"</h3><div class="quiz-choice-row"><button on:click=move |_| set_share_ok.set(Some(true))>"Public key"</button><button on:click=move |_| set_share_ok.set(Some(false))>"Secret key"</button></div><p class="quiz-feedback">{move || match share_ok.get(){Some(true)=>"Right. The public key is meant to be shared.",Some(false)=>"Keep the secret key secret.",None=>""}}</p></div>
            <div class="workbench-quiz"><h3>"Generate another pair."</h3><p>"A fresh secret produces a fresh public key too."</p><button type="button" on:click=move |_| spawn_local(async move { if curve::generate_x25519().await.is_ok(){set_regenerated.set(true);} })>"Generate another pair"</button><p class="quiz-feedback">{move || if regenerated.get(){"Done — both values were generated together."}else{"Generate another pair to complete this exercise."}}</p></div>
        </section>
        <LessonEnd exercises_complete=complete exercises_id="keypair-exercises" next_href="/public-key-encryption" next_label="Continue to 04 — Public-key encryption →" />
    }
}
