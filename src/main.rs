mod crypto;

use std::{cell::Cell, rc::Rc};

use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <section class="hero shell">
                <div class="eyebrow">"TryCrypto"</div>
                <h1>"Cryptography makes more sense when you can touch it."</h1>
                <p class="lede">
                    "Generate keys. Encrypt messages. Sign things. Break assumptions.
                    TryCrypto is a short, hands-on introduction to the cryptographic
                    primitives behind secure communication and verifiable claims."
                </p>
                <div class="privacy-note">
                    <strong>"Browser only."</strong>
                    " Your plaintext and keys never need to leave this page."
                </div>
            </section>

            <div class="shell">
                <HashLesson />
                <ComingNext />
            </div>
        </main>

        <footer class="shell">
            "TryCrypto is an educational project from Velox Warp. Do not treat a teaching
            playground as production key-management software."
        </footer>
    }
}

#[component]
fn HashLesson() -> impl IntoView {
    let (input, set_input) = signal(String::from("The quick brown fox jumps over the lazy dog"));
    let (digest, set_digest) = signal(String::from("Calculating…"));
    let (error, set_error) = signal(Option::<String>::None);
    let request_id = Rc::new(Cell::new(0_u64));

    Effect::new({
        let request_id = Rc::clone(&request_id);
        move |_| {
            let text = input.get();
            let id = request_id.get().wrapping_add(1);
            request_id.set(id);
            let request_id = Rc::clone(&request_id);

            spawn_local(async move {
                let result = crypto::sha256_hex(&text).await;

                // WebCrypto operations may resolve out of order. Only the most
                // recently started request is allowed to update the UI.
                if request_id.get() != id {
                    return;
                }

                match result {
                    Ok(value) => {
                        set_error.set(None);
                        set_digest.set(value);
                    }
                    Err(err) => set_error.set(Some(format!("{err:?}"))),
                }
            });
        }
    });

    view! {
        <section class="lesson">
            <div class="lesson-number">"Lesson 1"</div>
            <h2>"Hashes: a fingerprint for data"</h2>
            <p>
                "A cryptographic hash turns arbitrary data into a fixed-size value.
                The same input gives the same hash. Change even one character and the
                output should become dramatically different."
            </p>

            <label for="hash-input"><strong>"Try it"</strong></label>
            <textarea
                id="hash-input"
                prop:value=move || input.get()
                on:input=move |ev| set_input.set(event_target_value(&ev))
            />

            <div class="output" aria-live="polite">
                {move || match error.get() {
                    Some(err) => format!("WebCrypto error: {err}"),
                    None => format!("SHA-256\n{}", digest.get()),
                }}
            </div>

            <div class="callout">
                <strong>"What did this prove?"</strong>
                " Almost nothing about who created the data or whether it is true.
                A hash gives us a stable fingerprint. It becomes useful as a building
                block for integrity, content addressing, and signatures."
            </div>
        </section>
    }
}

#[component]
fn ComingNext() -> impl IntoView {
    let lessons = [
        (
            "02",
            "Shared-secret encryption",
            "AES-GCM: one secret, encryption and decryption.",
        ),
        (
            "03",
            "Public/private key pairs",
            "Generate a keypair and see which half is safe to share.",
        ),
        (
            "04",
            "Public-key encryption",
            "Encrypt for someone without possessing their private key.",
        ),
        (
            "05",
            "Digital signatures",
            "Sign with a private key; verify with a public key.",
        ),
        (
            "06",
            "Identity is not a key",
            "A valid signature proves a key signed—not automatically which human did.",
        ),
    ];

    view! {
        <section class="lesson">
            <div class="lesson-number">"Next"</div>
            <h2>"The path through TryCrypto"</h2>
            <p>
                "The site will stay deliberately small. Each lesson introduces one
                primitive, lets you manipulate it, then asks the question that matters:
                what does this result actually establish?"
            </p>

            <ol class="roadmap">
                {lessons
                    .into_iter()
                    .map(|(number, title, description)| {
                        view! {
                            <li>
                                <span class="roadmap-index">{number}</span>
                                <div>
                                    <strong>{title}</strong>
                                    <div class="status">{description}</div>
                                </div>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </section>
    }
}
