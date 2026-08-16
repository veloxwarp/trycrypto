use crate::LessonIntro;
use leptos::prelude::*;

#[component]
pub fn HashLesson() -> impl IntoView {
    view! {
     <LessonIntro number="01" eyebrow="" title="Hashes" summary="A hash turns data of any size into a small fingerprint."/>
     <section class="content-section motivation-section"><h2>"I made a 10 GB backup. How do I know it hasn't been corrupted?"</h2><p class="section-copy">"Comparing a huge restored backup with the original byte by byte is inconvenient. A hash reads the entire input and produces a small fixed-size fingerprint. The same input produces the same fingerprint; a change in the input changes the fingerprint."</p><p class="section-copy">"Save that small value when the backup is created, calculate it again later, and compare the two."</p></section>
     <crate::hash_workbench::HashWorkbench/>
     <section class="content-section"><h2>"What's SHA-256?"</h2><p class="section-copy">"SHA-256 is the hash algorithm used here. It produces 256 bits: 32 bytes, shown as 64 hex digits. It is one of many hash algorithms; TryCrypto chooses common algorithms for the exercises, but alternatives exist for every tool in this course."</p></section>
     <crate::hash_exercises::HashExercises/>
    }
}
