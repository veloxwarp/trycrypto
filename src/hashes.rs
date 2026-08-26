use crate::LessonIntro;
use leptos::prelude::*;

#[component]
pub fn HashLesson() -> impl IntoView {
    view! {
     <LessonIntro number="01" eyebrow="" title="Hashes" summary="A cryptographic hash turns any number of input bytes into a fixed-size fingerprint."/>
     <section class="content-section motivation-section"><h2>"Did my file get corrupted?"</h2><p class="section-copy">"You've just created a backup: a 10 GB ZIP file stored on an external hard drive. You're worried that the file might change. Maybe the drive will fail, or maybe someone will get access to it and alter it intentionally. How can you check later that the file is unchanged?"</p><p class="section-copy">"One option is to keep two copies on two different drives and compare them every time. But that requires another 10 GB of storage, and comparing two large files is slow."</p><p class="section-copy">"A hash function is a better fit. You can give it any amount of data, and it returns a small fingerprint representing that data. Even a tiny change to the input produces a very different hash."</p><p class="section-copy">"When you create the backup, save its tiny hash somewhere else—or in several places. Later, hash the backup again and compare the result with the hash you saved. If they match, that's strong evidence that the file's bytes have not changed since you created the hash. It does not tell you whether the original file was trustworthy."</p></section>
     <section class="content-section"><h2>"Other common use cases"</h2><ul class="use-case-list"><li><strong>"Validate a download."</strong> " A website can publish a file's hash so you can check that the file you received is exactly the one it provided."</li><li><strong>"Identify duplicate data."</strong> " If two files have the same cryptographic hash, that is strong evidence that their contents are identical. Backup and storage tools can use this to avoid storing the same data twice."</li></ul></section>
     <crate::hash_workbench::HashWorkbench/>
     <section class="content-section"><h2>"What's SHA-256?"</h2><p class="section-copy">"SHA-256 is the hash algorithm used here. It produces 256 bits: 32 bytes, shown as 64 hex digits. Because the output has a fixed size, collisions—two different inputs with the same output—must exist in principle. SHA-256 is designed so that finding one would require an absurd amount of brute-force guessing: far too much computation to be practical. TryCrypto uses common algorithms for the exercises, but no single algorithm represents every hash, cipher, or signature system."</p></section>
     <crate::hash_exercises::HashExercises/>
    }
}
