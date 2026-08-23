use crate::LessonIntro;
use leptos::prelude::*;

#[component]
pub fn IntroLesson() -> impl IntoView {
    view! {
        <LessonIntro number="INTRO" eyebrow="" title="Bytes & hexadecimal" summary="Cryptographic algorithms consume bytes. Hexadecimal is one convenient way for people to read and copy those bytes." />
        <section class="content-section primer-section">
            <h2>"Bytes are the actual input."</h2>
            <p class="section-copy">"A byte is a number from 0 through 255: 256 possible values. Text, images, and files can all be represented as bytes, which are the values cryptographic algorithms actually process."</p>
            <h2>"Hexadecimal is a display format."</h2>
            <p class="section-copy">"Hex uses 0–9 and A–F. A means 10, and F means 15. Two hex digits can represent every possible byte, from 00 through FF. Changing how bytes are displayed does not change the bytes themselves."</p>
        </section>
        <section class="workbench"><h2>"Decimal and hex show the same byte."</h2><p class="section-copy">"Edit either field. The number changes format, not value."</p><div class="byte-converter mini-workbench"><label>"Decimal (0–255)"<input id="decimal-value" value="173" inputmode="numeric" /></label><span class="conversion-equals">"="</span><label>"Hex (00–FF)"<input id="hex-value" value="AD" maxlength="2" autocapitalize="characters" spellcheck="false" /></label></div><p id="byte-converter-error" class="field-error" aria-live="polite"></p></section>
        <crate::intro_exercises::IntroExercises />
    }
}
