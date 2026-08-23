use crate::LessonEnd;
use leptos::prelude::*;

#[component]
pub fn IntroExercises() -> impl IntoView {
    let (hex_answer, set_hex_answer) = signal(String::new());
    let (decimal_answer, set_decimal_answer) = signal(String::new());
    let (hex_result, set_hex_result) = signal(Option::<bool>::None);
    let (decimal_result, set_decimal_result) = signal(Option::<bool>::None);
    let done =
        Memo::new(move |_| hex_result.get() == Some(true) && decimal_result.get() == Some(true));

    view! {
        <section id="intro-exercises" class="content-section planned-quiz">
            <h2>"Exercises"</h2>
            <div class="mini-workbench exercises-box">
                <form on:submit=move |event| {
                    event.prevent_default();
                    set_hex_result.set(Some(hex_answer.get().trim().eq_ignore_ascii_case("C8")));
                }>
                    <h3>"Decimal 200 in hex?"</h3>
                    <div class="quiz-answer-row">
                        <input aria-label="Decimal 200 in hexadecimal" maxlength="2" autocapitalize="characters" spellcheck="false" prop:value=move || hex_answer.get() on:input=move |event| { set_hex_answer.set(event_target_value(&event)); set_hex_result.set(None); } />
                        <button>"Check"</button>
                    </div>
                    <p class="quiz-feedback" aria-live="polite">{move || match hex_result.get() { Some(true) => "Correct: C8.", Some(false) => "Not quite. Convert 200 into groups of 16 and try again.", None => "" }}</p>
                </form>
                <form on:submit=move |event| {
                    event.prevent_default();
                    set_decimal_result.set(Some(decimal_answer.get().trim() == "123"));
                }>
                    <h3>"Hex 7B in decimal?"</h3>
                    <div class="quiz-answer-row">
                        <input aria-label="Hexadecimal 7B in decimal" inputmode="numeric" prop:value=move || decimal_answer.get() on:input=move |event| { set_decimal_answer.set(event_target_value(&event)); set_decimal_result.set(None); } />
                        <button>"Check"</button>
                    </div>
                    <p class="quiz-feedback" aria-live="polite">{move || match decimal_result.get() { Some(true) => "Correct: 123.", Some(false) => "Not quite. Seven sixteens plus eleven is 123.", None => "" }}</p>
                </form>
            </div>
        </section>
        <LessonEnd exercises_complete=done exercises_id="intro-exercises" next_href="/hashes" next_label="Continue to 01 — Hashes →" />
    }
}
