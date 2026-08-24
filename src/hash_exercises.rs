use crate::LessonEnd;
use leptos::prelude::*;

#[component]
pub fn HashExercises() -> impl IntoView {
    let (done, set_done) = signal(false);
    let complete = Memo::new(move |_| done.get());
    view! {
        <section id="hash-exercises" class="content-section planned-quiz">
            <h2>"Exercises"</h2>
            <div id="hash-exercise-content"></div>
            <button id="hash-complete" hidden on:click=move |_| set_done.set(true)>"Complete"</button>
        </section>
        <LessonEnd exercises_complete=complete exercises_id="hash-exercises" next_href="/shared-key-encryption" next_label="Continue to 02 — Shared-key encryption →" />
    }
}
