use crate::LessonIntro;
use leptos::prelude::*;

#[component]
pub fn HashLesson() -> impl IntoView {
    view! {
     <LessonIntro number="01" eyebrow="" title="Hashes" summary="A cryptographic hash turns any number of input bytes into a fixed-size fingerprint."/>
     <section class="content-section motivation-section"><h2>"I made a 10 GB backup. How do I know it hasn't been corrupted?"</h2><p class="section-copy">"You could keep another complete copy to compare against, but that means storing an extra 10 GB. Hashes offer a better option. A hash reads the entire input and produces a small, fixed-size fingerprint. The same input always produces the same fingerprint. Different input should, with overwhelming probability, produce a different one."</p><p class="section-copy">"Calculate and save the fingerprint when you create the backup. Later, hash the restored copy and compare the two values. A match is strong evidence that the bytes are unchanged; it does not tell you whether the original file was trustworthy."</p></section>
     <section class="content-section"><h2>"Other common uses"</h2><ul class="use-case-list"><li><strong>"Validate a download."</strong> " A website can publish a file's hash so you can check that the file you received is exactly the one it provided."</li><li><strong>"Detect an unexpected change."</strong> " Saving a trusted hash gives you something small to compare later if you suspect that an attacker—or a faulty system—modified the data behind your back."</li></ul></section>
     <crate::hash_workbench::HashWorkbench/>
     <section class="content-section"><h2>"What's SHA-256?"</h2><p class="section-copy">"SHA-256 is the hash algorithm used here. It produces 256 bits: 32 bytes, shown as 64 hex digits. Because the output has a fixed size, collisions—two different inputs with the same output—must exist in principle. SHA-256 is designed so that finding one would require an absurd amount of brute-force guessing: far too much computation to be practical. TryCrypto uses common algorithms for the exercises, but no single algorithm represents every hash, cipher, or signature system."</p></section>
     <crate::hash_exercises::HashExercises/>
    }
}
