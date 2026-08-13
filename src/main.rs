mod crypto;

use std::{cell::Cell, rc::Rc};

use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

fn main() {
    mount_to_body(App);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Intro,
    Hashes,
    SymmetricEncryption,
    Keypairs,
    PublicKeyEncryption,
    Signatures,
    Verification,
    NotFound,
}

impl Page {
    fn from_path(path: &str) -> Self {
        match path.trim_end_matches('/') {
            "" => Self::Intro,
            "/hashes" => Self::Hashes,
            "/symmetric-encryption" => Self::SymmetricEncryption,
            "/keypairs" => Self::Keypairs,
            "/public-key-encryption" => Self::PublicKeyEncryption,
            "/signatures" => Self::Signatures,
            "/verification" => Self::Verification,
            _ => Self::NotFound,
        }
    }
}

struct Lesson {
    number: &'static str,
    short: &'static str,
    title: &'static str,
    href: &'static str,
    description: &'static str,
}

const LESSONS: [Lesson; 6] = [
    Lesson {
        number: "01",
        short: "Hashes",
        title: "Hashes",
        href: "/hashes",
        description: "Turn data into a stable fingerprint and see what changes when the input changes.",
    },
    Lesson {
        number: "02",
        short: "Encryption",
        title: "Shared-secret encryption",
        href: "/symmetric-encryption",
        description: "Use one secret to encrypt and decrypt data with authenticated encryption.",
    },
    Lesson {
        number: "03",
        short: "Keypairs",
        title: "Public/private keypairs",
        href: "/keypairs",
        description: "Create a keypair and learn which half can be shared and which must stay private.",
    },
    Lesson {
        number: "04",
        short: "Public key",
        title: "Public-key encryption",
        href: "/public-key-encryption",
        description: "Encrypt something for another person without ever possessing their private key.",
    },
    Lesson {
        number: "05",
        short: "Signatures",
        title: "Digital signatures",
        href: "/signatures",
        description: "Sign exact data with a private key and verify it with the corresponding public key.",
    },
    Lesson {
        number: "06",
        short: "Verification",
        title: "Verification & identity",
        href: "/verification",
        description: "Ask the important question: what did a valid cryptographic result actually prove?",
    },
];

#[component]
fn App() -> impl IntoView {
    let path = web_sys::window()
        .and_then(|window| window.location().pathname().ok())
        .unwrap_or_else(|| "/".to_owned());
    let page = Page::from_path(&path);

    view! {
        <SiteHeader current=page />
        <main>
            {match page {
                Page::Intro => view! { <IntroPage /> }.into_any(),
                Page::Hashes => view! { <HashLesson /> }.into_any(),
                Page::SymmetricEncryption => view! {
                    <ComingLesson
                        number="02"
                        title="Shared-secret encryption"
                        eyebrow="ONE SECRET, TWO DIRECTIONS"
                        summary="Encryption lets us transform readable data into ciphertext that only someone with the right secret can recover."
                        points=&["Encrypt and decrypt with AES-GCM", "See the role of a nonce", "Learn why authenticated encryption matters"]
                    />
                }.into_any(),
                Page::Keypairs => view! {
                    <ComingLesson
                        number="03"
                        title="Public/private keypairs"
                        eyebrow="TWO KEYS, DIFFERENT JOBS"
                        summary="Public-key cryptography separates what you can safely share from the secret material that proves control."
                        points=&["Generate a keypair in the browser", "Compare public and private material", "Understand what possession of each key permits"]
                    />
                }.into_any(),
                Page::PublicKeyEncryption => view! {
                    <ComingLesson
                        number="04"
                        title="Public-key encryption"
                        eyebrow="ENCRYPT FOR SOMEONE ELSE"
                        summary="A public key can let other people protect data for you without giving them the secret needed to decrypt it."
                        points=&["Encrypt using a recipient's public key", "Decrypt using the corresponding private key", "Contrast this with shared-secret encryption"]
                    />
                }.into_any(),
                Page::Signatures => view! {
                    <ComingLesson
                        number="05"
                        title="Digital signatures"
                        eyebrow="PROVE CONTROL OF A KEY"
                        summary="A signature binds a private key to exact data in a way that anyone with the public key can verify."
                        points=&["Sign an exact message", "Verify with the public key", "Watch verification fail when the message changes"]
                    />
                }.into_any(),
                Page::Verification => view! {
                    <ComingLesson
                        number="06"
                        title="Verification & identity"
                        eyebrow="WHAT DID WE ACTUALLY PROVE?"
                        summary="Cryptography can give precise answers about keys and data. Connecting those answers to people, organizations, and truth requires additional evidence."
                        points=&["Separate keys from identities", "Distinguish valid signatures from true statements", "Identify the trust assumptions outside the cryptography"]
                    />
                }.into_any(),
                Page::NotFound => view! { <NotFound /> }.into_any(),
            }}
        </main>
        <SiteFooter />
    }
}

#[component]
fn SiteHeader(current: Page) -> impl IntoView {
    view! {
        <header class="site-header">
            <a class="brand" href="/" aria-label="TryCrypto home">
                <span>"Try"<i>"Crypto"</i></span>
            </a>
            <nav aria-label="Lessons">
                <a class:active=current == Page::Intro href="/">"Intro"</a>
                {LESSONS.iter().enumerate().map(|(index, lesson)| {
                    let active = matches!(
                        (index, current),
                        (0, Page::Hashes)
                            | (1, Page::SymmetricEncryption)
                            | (2, Page::Keypairs)
                            | (3, Page::PublicKeyEncryption)
                            | (4, Page::Signatures)
                            | (5, Page::Verification)
                    );
                    view! {
                        <a class:active=active href=lesson.href>{format!("{} {}", lesson.number, lesson.short)}</a>
                    }
                }).collect_view()}
            </nav>
        </header>
    }
}

#[component]
fn IntroPage() -> impl IntoView {
    view! {
        <section class="home-hero">
            <div>
                <p class="eyebrow">"AN INTERACTIVE INTRODUCTION TO PRACTICAL CRYPTOGRAPHY"</p>
                <h1>"Cryptography is everywhere." <span>"Learn what it actually does."</span></h1>
                <p class="lede">
                    "Every secure website you visit, every locked phone, and much of the software you rely on depends on cryptography."
                </p>
                <p class="lede">
                    "The mathematics behind modern cryptography can be sophisticated. But you don't need to be a cryptographer to understand the tools it gives us."
                </p>
                <div class="hero-actions">
                    <a class="button primary" href="/hashes">"Start with hashes →"</a>
                    <a class="button ghost" href="#lessons">"See the lessons"</a>
                </div>
            </div>
            <aside class="definition-card">
                <p class="eyebrow">"THE GOAL"</p>
                <h2>"Use the primitives. Understand the guarantees."</h2>
                <p>"Generate hashes. Encrypt messages. Create keys. Sign data. Verify signatures."</p>
                <p>"Then ask the question that matters: what does each result—and what doesn't it—prove?"</p>
            </aside>
        </section>

        <section class="everywhere-section content-section">
            <div class="section-heading">
                <p class="eyebrow">"ALREADY IN YOUR LIFE"</p>
                <h2>"You use cryptography every day."</h2>
            </div>
            <div class="principles">
                <article><span>"01"</span><h3>"Secure connections"</h3><p>"HTTPS uses cryptography to protect the connection between your browser and a website."</p></article>
                <article><span>"02"</span><h3>"Protected devices"</h3><p>"Phones and computers use cryptography to protect stored data and credentials."</p></article>
                <article><span>"03"</span><h3>"Verifiable software"</h3><p>"Hashes and signatures help establish that software and data are the exact bytes you expected."</p></article>
            </div>
        </section>

        <section id="lessons" class="content-section lessons-section">
            <p class="eyebrow">"THE COURSE"</p>
            <h2>"Six small lessons. One idea at a time."</h2>
            <p class="section-copy">"Each lesson introduces a primitive from the user's point of view, lets you experiment with it, and makes its guarantees explicit."</p>
            <div class="lesson-grid">
                {LESSONS.iter().map(|lesson| view! {
                    <a class="lesson-card" href=lesson.href>
                        <span>{lesson.number}</span>
                        <h3>{lesson.title}</h3>
                        <p>{lesson.description}</p>
                        <b>"Open lesson →"</b>
                    </a>
                }).collect_view()}
            </div>
        </section>

        <section class="boundary-section">
            <div>
                <p class="eyebrow">"WHAT THIS ISN'T"</p>
                <h2>"You don't need to implement cryptography to understand it."</h2>
            </div>
            <p>"TryCrypto isn't a mathematics course and it isn't a guide to writing cryptographic algorithms. Your browser already provides carefully implemented cryptographic primitives. We'll use those primitives to understand what they are for, how they fit together, and where their guarantees stop."</p>
        </section>

        <section class="about-section content-section">
            <div>
                <p class="eyebrow">"ABOUT THE PROJECT"</p>
                <h2>"Hi, I'm Michael."</h2>
                <p class="section-copy">"I'm Michael Snoyman, a software engineer, engineering leader, open-source developer, and author. I built TryCrypto because I wanted a straightforward way to explain the cryptographic building blocks we rely on without requiring people to start with the mathematics."</p>
                <div class="text-links">
                    <a href="https://www.snoyman.com/">"About Michael ↗"</a>
                    <a href="https://github.com/snoyberg">"GitHub ↗"</a>
                    <a href="https://www.snoyman.com/blog/">"Blog & subscribe ↗"</a>
                </div>
            </div>
            <aside class="project-teaser">
                <p class="eyebrow">"WHERE THIS GETS INTERESTING"</p>
                <h3>"What can a signature really tell us?"</h3>
                <p>"Knowing how to verify a signature is one thing. Deciding what that signature means is much harder."</p>
                <p>"I'm working on a new protocol built heavily around cryptographic evidence and those questions of identity, provenance, and trust. I'll share more when it's ready."</p>
                <a href="https://www.snoyman.com/blog/">"Follow along on my blog →"</a>
            </aside>
        </section>
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
        <LessonIntro number="01" eyebrow="A FINGERPRINT FOR DATA" title="Hashes" summary="A cryptographic hash turns arbitrary data into a fixed-size value. The same input gives the same hash; change even one character and the output changes dramatically." />
        <section class="workbench">
            <div class="workbench-heading">
                <div><p class="eyebrow">"BROWSER WORKBENCH"</p><h2>"Try SHA-256 yourself."</h2></div>
                <p>"Everything happens locally in your browser. The text you enter here is not sent to a server."</p>
            </div>
            <div class="hash-tool">
                <label for="hash-input"><span>"INPUT"</span><textarea id="hash-input" prop:value=move || input.get() on:input=move |ev| set_input.set(event_target_value(&ev)) /></label>
                <div class="arrow">"↓"</div>
                <div class="output" aria-live="polite">
                    <span>"SHA-256"</span>
                    <code>{move || match error.get() { Some(err) => format!("WebCrypto error: {err}"), None => digest.get() }}</code>
                </div>
            </div>
        </section>
        <section class="lesson-explanation content-section">
            <p class="eyebrow">"WHAT DID THIS PROVE?"</p>
            <h2>"A hash proves less than people often assume."</h2>
            <p class="section-copy">"The digest gives us a stable fingerprint for these exact bytes. If two people independently hash identical data with SHA-256, they get the same result. Change the data and the digest changes."</p>
            <div class="principles">
                <article><span>"YES"</span><h3>"Stable identity for bytes"</h3><p>"A hash is useful for integrity checks, content addressing, and as an input to signatures."</p></article>
                <article><span>"NO"</span><h3>"No author identity"</h3><p>"The hash alone tells you nothing about who created, published, or endorsed the data."</p></article>
                <article><span>"NO"</span><h3>"No claim of truth"</h3><p>"False information hashes just as reliably as true information."</p></article>
            </div>
            <a class="next-lesson" href="/symmetric-encryption"><span>"NEXT"</span><b>"02 — Shared-secret encryption →"</b></a>
        </section>
    }
}

#[component]
fn LessonIntro(
    number: &'static str,
    eyebrow: &'static str,
    title: &'static str,
    summary: &'static str,
) -> impl IntoView {
    view! {
        <section class="lesson-hero">
            <div class="lesson-index">{number}</div>
            <div><p class="eyebrow">{eyebrow}</p><h1>{title}</h1><p class="lede">{summary}</p></div>
        </section>
    }
}

#[component]
fn ComingLesson(
    number: &'static str,
    title: &'static str,
    eyebrow: &'static str,
    summary: &'static str,
    points: &'static [&'static str],
) -> impl IntoView {
    view! {
        <LessonIntro number eyebrow title summary />
        <section class="coming-section">
            <div><p class="eyebrow">"LESSON IN DEVELOPMENT"</p><h2>"This workbench is coming next."</h2><p>"The route and lesson structure are in place. The interactive exercise will be added here without changing the overall course navigation."</p></div>
            <ol>{points.iter().map(|point| view! { <li>{*point}</li> }).collect_view()}</ol>
        </section>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <section class="not-found">
            <p class="eyebrow">"404"</p><h1>"That lesson isn't here."</h1><p class="lede">"The cryptography may be complicated. The navigation shouldn't be."</p><a class="button primary" href="/">"Back to the intro"</a>
        </section>
    }
}

#[component]
fn SiteFooter() -> impl IntoView {
    view! {
        <footer>
            <span>"TryCrypto — an educational project by Michael Snoyman."</span>
            <span><a href="https://github.com/veloxwarp/trycrypto">"Source on GitHub ↗"</a> " · Browser cryptography is for learning here, not production key management."</span>
        </footer>
    }
}
