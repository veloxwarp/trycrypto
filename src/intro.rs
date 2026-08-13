use leptos::prelude::*;
use crate::LessonIntro;

#[component]
pub fn IntroLesson() -> impl IntoView {
    view! {
        <LessonIntro number="INTRO" eyebrow="" title="Bytes & hexadecimal" summary="Cryptographic tools work with bytes, often displayed in hexadecimal." />
        <section class="content-section primer-section"><h2>"Bytes"</h2><p class="section-copy">"A byte is a number from 0 through 255: 256 possible values."</p><h2>"Hexadecimal"</h2><p class="section-copy">"Hex uses 0–9 and A–F. A means 10 through F meaning 15. Two hex digits represent one byte, from 00 through FF."</p></section>
        <section class="workbench"><h2>"Decimal and hex show the same byte."</h2><div class="byte-converter mini-workbench"><label>"Decimal (0–255)"<input id="decimal-value" value="173" /></label><span class="conversion-equals">"="</span><label>"Hex (00–FF)"<input id="hex-value" value="AD" maxlength="2" /></label></div><p id="byte-converter-error" class="field-error"></p></section>
        <crate::intro_exercises::IntroExercises />
    }
}
