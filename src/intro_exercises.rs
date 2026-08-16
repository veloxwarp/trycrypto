use crate::LessonEnd;
use leptos::prelude::*;

#[component]
pub fn IntroExercises() -> impl IntoView {
    let (a, set_a) = signal(String::new());
    let (b, set_b) = signal(String::new());
    let (ra, set_ra) = signal(false);
    let (rb, set_rb) = signal(false);
    let done = Memo::new(move |_| ra.get() && rb.get());
    view! {<section id="intro-exercises" class="content-section planned-quiz"><h2>"Exercises"</h2><div class="mini-workbench exercises-box">
    <form on:submit=move|e|{e.prevent_default();set_ra.set(a.get().trim().eq_ignore_ascii_case("C8"));}><h3>"Decimal 200 in hex?"</h3><div class="quiz-answer-row"><input prop:value=move||a.get() on:input=move|e|set_a.set(event_target_value(&e))/><button>"Check"</button></div><p class="quiz-feedback">{move||if ra.get(){"Correct: C8."}else{""}}</p></form>
    <form on:submit=move|e|{e.prevent_default();set_rb.set(b.get().trim()=="123");}><h3>"Hex 7B in decimal?"</h3><div class="quiz-answer-row"><input prop:value=move||b.get() on:input=move|e|set_b.set(event_target_value(&e))/><button>"Check"</button></div><p class="quiz-feedback">{move||if rb.get(){"Correct: 123."}else{""}}</p></form>
    </div></section><LessonEnd exercises_complete=done exercises_id="intro-exercises" next_href="/hashes" next_label="Continue to 01 — Hashes →"/>}
}
