use leptos::prelude::*;
use leptos_router::components::A;
use wasm_bindgen_futures::spawn_local;

use crate::crypto;

#[component]
pub fn HomePage() -> impl IntoView {
    let (preview_input, set_preview_input) =
        signal("The quick brown fox jumps over the lazy dog".to_owned());
    let (preview_hash, set_preview_hash) = signal("Calculating…".to_owned());
    let (hash_request, set_hash_request) = signal(0_u32);

    {
        let input = preview_input.get_untracked();
        spawn_local(async move {
            match crypto::sha256_hex(&input).await {
                Ok(hash) => set_preview_hash.set(hash),
                Err(_) => {
                    set_preview_hash.set("Hashing is unavailable in this browser.".to_owned())
                }
            }
        });
    }

    view! {
        <section class="home-hero">
            <p class="home-kicker">"Interactive cryptography lessons"</p>
            <h1>"Cryptography is everywhere." <span>"Learn what it actually does."</span></h1>
            <div class="home-intro-grid">
                <div class="home-intro-copy">
                    <p class="lede">"Every secure website you visit, every locked phone, and much of the software you rely on depends on cryptography."</p>
                    <p class="lede">"Use the tools yourself and learn what they guarantee—without first learning how to implement the algorithms."</p>
                </div>
                <div class="home-hash-preview">
                    <p class="exercise-number">"LIVE PREVIEW"</p>
                    <h2>"Change one character."</h2>
                    <p>"The SHA-256 fingerprint changes with the exact input."</p>
                    <label for="home-hash-input">"Message"</label>
                    <div class="paste-input-row" data-pasteable="preview message">
                        <input
                            id="home-hash-input"
                            prop:value=move || preview_input.get()
                            on:input=move |event| {
                                let input = event_target_value(&event);
                                set_preview_input.set(input.clone());
                                set_preview_hash.set("Calculating…".to_owned());
                                set_hash_request.update(|request| *request += 1);
                                let request = hash_request.get_untracked();
                                spawn_local(async move {
                                    match crypto::sha256_hex(&input).await {
                                        Ok(hash) if hash_request.get_untracked() == request => set_preview_hash.set(hash),
                                        Err(_) if hash_request.get_untracked() == request => set_preview_hash.set("Hashing is unavailable in this browser.".to_owned()),
                                        _ => {}
                                    }
                                });
                            }
                        />
                    </div>
                    <div class="output" aria-live="polite"><span>"SHA-256"</span><code>{move || preview_hash.get()}</code></div>
                    <A href="/hashes">"Explore hashes →"</A>
                </div>
            </div>
            <p class="course-intro"><strong>"Seven short lessons · about 55 minutes."</strong> " Follow the concepts in order, or jump directly to one you want to understand."</p>
            <div class="course-list">
                <A href="/bytes-and-hexadecimal" attr:class="course-row course-row-start"><span class="course-number">"INTRO"</span><span class="course-details"><strong>"Bytes & hexadecimal"</strong><span>"Read and copy the exact data cryptography uses."</span></span><span class="course-meta"><span>"4 min"</span><span class="start-badge">"Start →"</span></span></A>
                <A href="/hashes" attr:class="course-row"><span class="course-number">"01"</span><span class="course-details"><strong>"Hashes"</strong><span>"Detect when data has changed."</span></span><span class="course-meta"><span>"8 min"</span><span class="course-arrow">"→"</span></span></A>
                <A href="/shared-key-encryption" attr:class="course-row"><span class="course-number">"02"</span><span class="course-details"><strong>"Shared-key encryption"</strong><span>"Encrypt and recover data with one secret key."</span></span><span class="course-meta"><span>"10 min"</span><span class="course-arrow">"→"</span></span></A>
                <A href="/public-key" attr:class="course-row"><span class="course-number">"03"</span><span class="course-details"><strong>"Public key"</strong><span>"Generate one secret value and one shareable value."</span></span><span class="course-meta"><span>"6 min"</span><span class="course-arrow">"→"</span></span></A>
                <A href="/public-key-encryption" attr:class="course-row"><span class="course-number">"04"</span><span class="course-details"><strong>"Public-key encryption"</strong><span>"Encrypt a message for a specific recipient."</span></span><span class="course-meta"><span>"10 min"</span><span class="course-arrow">"→"</span></span></A>
                <A href="/digital-signatures" attr:class="course-row"><span class="course-number">"05"</span><span class="course-details"><strong>"Digital signatures"</strong><span>"Sign exact data and detect tampering."</span></span><span class="course-meta"><span>"9 min"</span><span class="course-arrow">"→"</span></span></A>
                <A href="/verification-and-identity" attr:class="course-row"><span class="course-number">"06"</span><span class="course-details"><strong>"Verification & identity"</strong><span>"Separate proof about a key from claims about a person."</span></span><span class="course-meta"><span>"8 min"</span><span class="course-arrow">"→"</span></span></A>
            </div>
        </section>
        <section class="home-flow-section content-section">
            <h2>"Hi, I'm Michael."</h2>
            <p class="section-copy">"I'm Michael Snoyman, a software engineer, open-source developer, engineering leader, and author. I built TryCrypto as a straightforward introduction to the cryptographic building blocks we rely on."</p>
        </section>
        <section id="protocol-interest" class="home-flow-section content-section">
            <h2>"Cryptography can prove some facts. It can't prove everything."</h2>
            <p class="section-copy">"I'm working on a protocol that uses cryptographic evidence to help build networks of trust. I'll be sharing more information as the work develops."</p>
            <p class="section-copy">"If you'd like to be notified when there's something new, join the mailing list below. I'll only use it for relevant updates about this work, and I won't sell your information or share it with marketers."</p>
            <a href="https://www.snoyman.com/blog/" target="_blank" rel="noopener noreferrer">"Alternatively, follow updates on my blog ↗"</a>
        </section>
        <section class="home-start-section content-section">
            <h2>"Ready to try it? Start with the intro lesson."</h2>
            <A href="/bytes-and-hexadecimal" attr:class="button primary">"Start the intro →"</A>
        </section>
    }
}
