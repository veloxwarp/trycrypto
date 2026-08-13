mod crypto;

use std::{cell::Cell, rc::Rc};

use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};
use wasm_bindgen_futures::spawn_local;

fn main() {
    mount_to_body(App);
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
        description: "Check that large data arrived unchanged without sending the whole thing back.",
    },
    Lesson {
        number: "02",
        short: "Encryption",
        title: "Shared-secret encryption",
        href: "/symmetric-encryption",
        description: "Protect data so that only someone who shares your secret can read it.",
    },
    Lesson {
        number: "03",
        short: "Keypairs",
        title: "Public/private keypairs",
        href: "/keypairs",
        description: "Separate something safe to publish from something only you must keep secret.",
    },
    Lesson {
        number: "04",
        short: "Public key",
        title: "Public-key encryption",
        href: "/public-key-encryption",
        description: "Send confidential data to someone even when you never shared a secret first.",
    },
    Lesson {
        number: "05",
        short: "Signatures",
        title: "Digital signatures",
        href: "/signatures",
        description: "Let anyone check that exact data was approved by the holder of a private key.",
    },
    Lesson {
        number: "06",
        short: "Verification",
        title: "Verification & identity",
        href: "/verification",
        description: "Separate what cryptography proves about keys and bytes from what we infer about people.",
    },
];

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <SiteHeader />
            <main>
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("index.html") view=HomePage />
                    <Route path=path!("intro") view=IntroLesson />
                    <Route path=path!("hashes") view=HashLesson />
                    <Route path=path!("symmetric-encryption") view=SymmetricEncryptionPage />
                    <Route path=path!("keypairs") view=KeypairsPage />
                    <Route path=path!("public-key-encryption") view=PublicKeyEncryptionPage />
                    <Route path=path!("signatures") view=SignaturesPage />
                    <Route path=path!("verification") view=VerificationPage />
                </Routes>
            </main>
            <SiteFooter />
        </Router>
    }
}

#[component]
fn SiteHeader() -> impl IntoView {
    view! {
        <header class="site-header">
            <A href="/" exact=true attr:class="brand" attr:aria-label="TryCrypto home">
                <span>"Try"<i>"Crypto"</i></span>
            </A>
            <nav aria-label="Lessons">
                <A href="/intro" exact=true>"Intro"</A>
                {LESSONS
                    .iter()
                    .map(|lesson| {
                        view! {
                            <A href=lesson.href exact=true>
                                {format!("{} {}", lesson.number, lesson.short)}
                            </A>
                        }
                    })
                    .collect_view()}
            </nav>
        </header>
    }
}

#[component]
fn HomePage() -> impl IntoView {
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
                    <A href="/intro" attr:class="button primary">"Start with the intro →"</A>
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

        <section id="lessons" class="content-section lessons-section">
            <p class="eyebrow">"THE COURSE"</p>
            <h2>"Start with a problem. Discover the cryptographic tool that solves it."</h2>
            <p class="section-copy">"TryCrypto is organized around practical questions: How can I tell whether a file changed? How can I send something privately? How can I prove that a key approved a message? Start with a short intro to bytes and hexadecimal, then use an interactive workbench in each lesson to answer the question yourself."</p>
            <div class="lesson-grid">
                {LESSONS
                    .iter()
                    .map(|lesson| {
                        view! {
                            <A href=lesson.href exact=true attr:class="lesson-card">
                                <span>{lesson.number}</span>
                                <h3>{lesson.title}</h3>
                                <p>{lesson.description}</p>
                                <b>"Open lesson →"</b>
                            </A>
                        }
                    })
                    .collect_view()}
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
                    <a href="https://www.snoyman.com/" target="_blank" rel="noopener noreferrer">"About Michael ↗"</a>
                    <a href="https://github.com/snoyberg" target="_blank" rel="noopener noreferrer">"GitHub ↗"</a>
                    <a href="https://www.snoyman.com/blog/" target="_blank" rel="noopener noreferrer">"Blog & subscribe ↗"</a>
                </div>
            </div>
            <aside class="project-teaser">
                <p class="eyebrow">"WHERE THIS GETS INTERESTING"</p>
                <h3>"What can a signature really tell us?"</h3>
                <p>"Knowing how to verify a signature is one thing. Deciding what that signature means is much harder."</p>
                <p>"I'm working on a new protocol built heavily around cryptographic evidence and those questions of identity, provenance, and trust. I'll share more when it's ready."</p>
                <a href="https://www.snoyman.com/blog/" target="_blank" rel="noopener noreferrer">"Follow along on my blog →"</a>
            </aside>
        </section>
    }
}

#[component]
fn IntroLesson() -> impl IntoView {
    let (decimal_value, set_decimal_value) = signal(173_u16);
    let (hex_value, set_hex_value) = signal(String::from("AD"));
    let (hex_answer, set_hex_answer) = signal(String::new());
    let (hex_result, set_hex_result) = signal(Option::<bool>::None);
    let (decimal_answer, set_decimal_answer) = signal(String::new());
    let (decimal_result, set_decimal_result) = signal(Option::<bool>::None);
    let (show_quiz_nudge, set_show_quiz_nudge) = signal(false);

    let quiz_complete =
        Memo::new(move |_| hex_result.get() == Some(true) && decimal_result.get() == Some(true));

    view! {
        <LessonIntro
            number="INTRO"
            eyebrow="A FEW BASICS BEFORE WE START"
            title="Bytes & hexadecimal"
            summary="Cryptographic tools work with bytes, and those bytes are often displayed in hexadecimal. You only need a little of both to follow the rest of TryCrypto."
        />

        <section class="content-section primer-section">
            <div class="section-heading">
                <p class="eyebrow">"BYTES"</p>
                <h2>"A byte is a number from 0 through 255."</h2>
                <p class="section-copy">"That's 256 possible values. Cryptographic keys, hashes, ciphertext, and other data are ultimately made up of bytes, so you'll see these values throughout the lessons."</p>
            </div>

            <div class="section-heading">
                <p class="eyebrow">"HEXADECIMAL"</p>
                <h2>"A compact way to write byte values."</h2>
                <p class="section-copy">"The normal number system we use is decimal, or base 10. That means we have 10 possible digits: the numbers 0 through 9. Hexadecimal is base 16, so it needs six additional values. Beyond 0 through 9, it also includes A through F. A means 10, B means 11, and so on through F, which means 15. The letters may be uppercase or lowercase; A and a mean the same value."</p>
                <p class="section-copy">"In decimal, the number 54 means 5 tens and 4 ones, so its value is 5 × 10 + 4. Hex works the same way, except that in a two-digit hex number the left digit counts sixteens instead of tens. So A5 means 10 × 16 + 5, which is 165 in decimal."</p>
                <p class="section-copy">"Because two hex digits can represent 16 × 16 = 256 different values, one byte always fits neatly into exactly two hex digits, from 00 through FF."</p>
            </div>

            <table class="hex-table">
                <thead><tr><th>"Decimal"</th><th>"Hex"</th></tr></thead>
                <tbody>
                    <tr><td>"0"</td><td><code>"00"</code></td></tr>
                    <tr><td>"9"</td><td><code>"09"</code></td></tr>
                    <tr><td>"10"</td><td><code>"0A"</code></td></tr>
                    <tr><td>"15"</td><td><code>"0F"</code></td></tr>
                    <tr><td>"16"</td><td><code>"10"</code></td></tr>
                    <tr><td>"31"</td><td><code>"1F"</code></td></tr>
                    <tr><td>"165"</td><td><code>"A5"</code></td></tr>
                    <tr><td>"255"</td><td><code>"FF"</code></td></tr>
                </tbody>
            </table>
        </section>

        <section class="workbench">
            <div class="workbench-heading">
                <div>
                    <p class="eyebrow">"BYTE EXPLORER"</p>
                    <h2>"Try converting in both directions."</h2>
                </div>
                <p>"Use the explorer freely. You'll use the same tool for the quick check below."</p>
            </div>

            <div class="primer-grid">
                <div class="mini-workbench">
                    <p class="eyebrow">"DECIMAL → HEX"</p>
                    <label for="decimal-value">"Decimal value (0–255)"</label>
                    <input
                        id="decimal-value"
                        type="number"
                        min="0"
                        max="255"
                        prop:value=move || decimal_value.get().to_string()
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<u16>() {
                                set_decimal_value.set(value.min(255));
                            }
                        }
                    />
                    <div class="byte-result">
                        <span>"HEX"</span>
                        <code>{move || format!("{:02X}", decimal_value.get())}</code>
                    </div>
                </div>

                <div class="mini-workbench">
                    <p class="eyebrow">"HEX → DECIMAL"</p>
                    <label for="hex-value">"Hex value (00–FF)"</label>
                    <input
                        id="hex-value"
                        maxlength="2"
                        prop:value=move || hex_value.get()
                        on:input=move |ev| {
                            let value = event_target_value(&ev).to_ascii_uppercase();
                            if value.len() <= 2 && value.chars().all(|c| c.is_ascii_hexdigit()) {
                                set_hex_value.set(value);
                            }
                        }
                    />
                    <div class="byte-result">
                        <span>"DECIMAL"</span>
                        <code>{move || {
                            u8::from_str_radix(hex_value.get().trim(), 16)
                                .map(|value| value.to_string())
                                .unwrap_or_else(|_| "—".to_owned())
                        }}</code>
                    </div>
                </div>
            </div>
            <p class="microcopy">"Try values such as 10, 15, 16, 31, A5, AD, and FF and watch how the two representations correspond."</p>
        </section>

        <section id="intro-quiz" class="content-section planned-quiz">
            <p class="eyebrow">"QUICK CHECK"</p>
            <h2>"Use the explorer, then answer two questions."</h2>
            <p class="section-copy">"The goal isn't to memorize hex arithmetic. It's to get comfortable reading the representation and using the tool when you need it."</p>

            <div class="mini-workbench">
                <div class="tool-quiz">
                    <p class="eyebrow">"1 OF 2 · DECIMAL → HEX"</p>
                    <h3>"What is decimal 200 in hex?"</h3>
                    <div class="quiz-answer-row">
                        <input
                            aria-label="Hex value for decimal 200"
                            placeholder="Your answer"
                            prop:value=move || hex_answer.get()
                            on:input=move |ev| {
                                set_hex_answer.set(event_target_value(&ev));
                                set_hex_result.set(None);
                            }
                        />
                        <button
                            type="button"
                            on:click=move |_| {
                                set_hex_result.set(Some(hex_answer.get().trim().eq_ignore_ascii_case("C8")));
                            }
                        >"Check"</button>
                    </div>
                    <p class="quiz-feedback" aria-live="polite">
                        {move || match hex_result.get() {
                            Some(true) => "Correct. C8 is decimal 200.",
                            Some(false) => "Not quite. Put 200 into the decimal → hex explorer and check the result.",
                            None => "",
                        }}
                    </p>
                </div>

                <div class="tool-quiz">
                    <p class="eyebrow">"2 OF 2 · HEX → DECIMAL"</p>
                    <h3>"What is hex 7B in decimal?"</h3>
                    <div class="quiz-answer-row">
                        <input
                            aria-label="Decimal value for hex 7B"
                            placeholder="Your answer"
                            prop:value=move || decimal_answer.get()
                            on:input=move |ev| {
                                set_decimal_answer.set(event_target_value(&ev));
                                set_decimal_result.set(None);
                            }
                        />
                        <button
                            type="button"
                            on:click=move |_| {
                                set_decimal_result.set(Some(decimal_answer.get().trim() == "123"));
                            }
                        >"Check"</button>
                    </div>
                    <p class="quiz-feedback" aria-live="polite">
                        {move || match decimal_result.get() {
                            Some(true) => "Correct. 7B is decimal 123.",
                            Some(false) => "Not quite. Put 7B into the hex → decimal explorer and check the result.",
                            None => "",
                        }}
                    </p>
                </div>

                <p class="quiz-feedback" aria-live="polite">
                    {move || {
                        let completed = [hex_result.get(), decimal_result.get()]
                            .into_iter()
                            .filter(|result| *result == Some(true))
                            .count();
                        match completed {
                            0 => "Two quick checks to go.".to_owned(),
                            1 => "One down, one to go.".to_owned(),
                            _ => "Nice — intro complete. You're ready for hashes.".to_owned(),
                        }
                    }}
                </p>
            </div>
        </section>

        <section class="content-section">
            <Show
                when=move || quiz_complete.get()
                fallback=move || view! {
                    <div>
                        <a
                            href="#intro-quiz"
                            class="next-lesson"
                            on:click=move |_| set_show_quiz_nudge.set(true)
                        >
                            <span>"NEXT"</span>
                            <b>"01 — Hashes → · Quick check not finished"</b>
                        </a>
                        <Show when=move || show_quiz_nudge.get()>
                            <div class="precision-note">
                                <strong>"Almost there."</strong>
                                <p>"The quick check is meant to make the lesson stick. Finish the two questions above, or skip it if you'd rather keep moving."</p>
                                <A href="/hashes">"Skip quiz and continue to Hashes →"</A>
                            </div>
                        </Show>
                    </div>
                }
            >
                <A href="/hashes" attr:class="next-lesson">
                    <span>"NEXT"</span><b>"01 — Hashes →"</b>
                </A>
            </Show>
        </section>
    }
}

#[component]
fn HashLesson() -> impl IntoView {
    const CHALLENGE_A: &str = "Meet me at 10:30 by the north entrance.";
    const CHALLENGE_B: &str = "Meet me at 10:30 by the south entrance.";
    const CHALLENGE_HASH: &str = "6c1e614182df466a0629118845873531184affe88aeb240ed834301a82908f47";

    let (input, set_input) = signal(String::from("The quick brown fox jumps over the lazy dog"));
    let (digest, set_digest) = signal(String::from("Calculating…"));
    let (error, set_error) = signal(Option::<String>::None);
    let (challenge_result, set_challenge_result) = signal(Option::<bool>::None);
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
        <LessonIntro
            number="01"
            eyebrow="CHECK THAT DATA ARRIVED INTACT"
            title="Hashes"
            summary="A hash gives us a small fingerprint for data, even when the data itself is enormous."
        />

        <section class="motivation-section content-section">
            <p class="eyebrow">"WHY WOULD I WANT THIS?"</p>
            <h2>"I sent you a 10 GB file. Did you get exactly what I sent?"</h2>
            <div class="prose-grid">
                <p>"One solution is for you to send the entire 10 GB file back to me so I can compare the two copies. That works, but it's ridiculous: verifying the transfer costs another 10 GB transfer."</p>
                <p>"Instead, we can both hash our copies. SHA-256 turns any amount of data into a 32-byte result, commonly displayed as 64 hex digits. We compare those tiny results instead of retransmitting the file. If they match, we have extremely strong evidence that our copies contain exactly the same bytes."</p>
            </div>
            <aside class="precision-note">
                <strong>"Could two different files ever have the same SHA-256 hash?"</strong>
                <p>"Yes, mathematically they must: there are more possible files than possible 256-bit hashes. But SHA-256 is designed so that deliberately finding a useful match requires an infeasible amount of work. In practice, we treat that as impossible for this kind of integrity check."</p>
            </aside>
        </section>

        <section class="workbench">
            <div class="workbench-heading">
                <div><p class="eyebrow">"BROWSER WORKBENCH"</p><h2>"Try SHA-256 yourself."</h2></div>
                <p>"Change the input—even by one character—and compare the 64 hex digits below. Everything happens locally in your browser."</p>
            </div>
            <div class="hash-tool">
                <label for="hash-input">
                    <span>"INPUT"</span>
                    <textarea
                        id="hash-input"
                        prop:value=move || input.get()
                        on:input=move |ev| set_input.set(event_target_value(&ev))
                    />
                </label>
                <div class="arrow">"↓"</div>
                <div class="output" aria-live="polite">
                    <span>"SHA-256 · 32 BYTES · 64 HEX DIGITS"</span>
                    <code>{move || match error.get() {
                        Some(err) => format!("WebCrypto error: {err}"),
                        None => digest.get(),
                    }}</code>
                </div>
            </div>

            <div class="workbench-quiz">
                <p class="eyebrow">"USE THE WORKBENCH"</p>
                <h3>"Which message matches this hash?"</h3>
                <code class="target-hash">{CHALLENGE_HASH}</code>
                <p>"Load each candidate into the workbench above and compare its SHA-256 result with the target."</p>
                <div class="candidate-grid">
                    <div>
                        <span>"A"</span>
                        <p>{CHALLENGE_A}</p>
                        <button type="button" on:click=move |_| set_input.set(CHALLENGE_A.to_owned())>"Load A into workbench"</button>
                    </div>
                    <div>
                        <span>"B"</span>
                        <p>{CHALLENGE_B}</p>
                        <button type="button" on:click=move |_| set_input.set(CHALLENGE_B.to_owned())>"Load B into workbench"</button>
                    </div>
                </div>
                <div class="quiz-choice-row">
                    <span>"Which one matches?"</span>
                    <button type="button" on:click=move |_| set_challenge_result.set(Some(true))>"A"</button>
                    <button type="button" on:click=move |_| set_challenge_result.set(Some(false))>"B"</button>
                </div>
                <p class="quiz-feedback" aria-live="polite">
                    {move || match challenge_result.get() {
                        Some(true) => "Exactly. A hashes to the target value. Changing only “north” to “south” produces a completely different digest.",
                        Some(false) => "Not quite. Load both messages into the workbench and compare each SHA-256 result with the target hash.",
                        None => "",
                    }}
                </p>
            </div>
        </section>

        <section class="lesson-explanation content-section">
            <p class="eyebrow">"WHAT DID THIS PROVE?"</p>
            <h2>"The hash checks the bytes—not their source or their truth."</h2>
            <p class="section-copy">"If I give you a trusted SHA-256 digest and your file produces the same digest, that's excellent evidence that you have the exact file I meant. But the hash itself cannot tell you who created the file or who gave you the digest."</p>
            <div class="principles">
                <article><span>"YES"</span><h3>"Integrity"</h3><p>"A hash lets us cheaply compare large amounts of data and detect accidental or deliberate changes."</p></article>
                <article><span>"NO"</span><h3>"No authenticated source"</h3><p>"If an attacker can replace both a file and the hash you compare it against, they can simply provide the correct hash of their replacement."</p></article>
                <article><span>"NO"</span><h3>"No claim of truth"</h3><p>"False information hashes just as reliably as true information."</p></article>
            </div>

            <aside class="side-note">
                <p class="eyebrow">"ONE MORE PLACE HASHES SHOW UP"</p>
                <h3>"Proof of work"</h3>
                <p>"Hashes are also useful when we want a task that is expensive to perform but cheap to verify. Proof-of-work systems repeatedly vary data and hash it until they find a result meeting a difficult condition. Checking the winning hash is easy; finding it required lots of trial and error."</p>
            </aside>

            <A href="/symmetric-encryption" attr:class="next-lesson">
                <span>"NEXT"</span><b>"02 — Shared-secret encryption →"</b>
            </A>
        </section>
    }
}

#[component]
fn SymmetricEncryptionPage() -> impl IntoView {
    view! {
        <ComingLesson
            number="02"
            title="Shared-secret encryption"
            eyebrow="ONE SECRET, TWO DIRECTIONS"
            summary="Encryption lets us transform readable data into ciphertext that only someone with the right secret can recover."
            problem="You and I already share a secret. How can I send you data that anyone may intercept, but only we can read?"
            challenge="Use the encryption workbench to determine which ciphertext was produced from a given message and secret, then change one input and observe what breaks."
            points=&["Encrypt and decrypt with AES-GCM", "See why fresh nonces matter", "Learn why modern encryption also detects tampering"]
        />
    }
}

#[component]
fn KeypairsPage() -> impl IntoView {
    view! {
        <ComingLesson
            number="03"
            title="Public/private keypairs"
            eyebrow="TWO KEYS, DIFFERENT JOBS"
            summary="Public-key cryptography separates material that is safe to publish from secret material that must remain under your control."
            problem="Shared-secret encryption is useful—but first we somehow had to share a secret. Can I publish something useful without publishing the secret that gives me control?"
            challenge="Generate a keypair in the workbench, then identify which operations still work when you keep only the public half and which require the private half."
            points=&["Generate a keypair in the browser", "Compare public and private material", "Understand what possession of each key permits"]
        />
    }
}

#[component]
fn PublicKeyEncryptionPage() -> impl IntoView {
    view! {
        <ComingLesson
            number="04"
            title="Public-key encryption"
            eyebrow="ENCRYPT FOR SOMEONE ELSE"
            summary="A public key can let other people protect data for you without giving them the secret needed to decrypt it."
            problem="You need to send me confidential data, but we've never exchanged a shared secret. Can you encrypt something that only I can open?"
            challenge="Use two generated keypairs to encrypt for one recipient, then prove with the workbench that the other private key cannot decrypt the ciphertext."
            points=&["Encrypt using a recipient's public key", "Decrypt using the corresponding private key", "Contrast this with shared-secret encryption"]
        />
    }
}

#[component]
fn SignaturesPage() -> impl IntoView {
    view! {
        <ComingLesson
            number="05"
            title="Digital signatures"
            eyebrow="PROVE CONTROL OF A KEY"
            summary="A signature binds a private key to exact data in a way that anyone with the public key can verify."
            problem="I publish a software release and you download it from somewhere else. How can you check that these exact bytes are the ones the holder of my private key approved?"
            challenge="Use the workbench to verify a signature, then change one character in the message and watch the same signature fail."
            points=&["Sign exact data with a private key", "Verify with the public key", "See how hashes and signatures fit together"]
        />
    }
}

#[component]
fn VerificationPage() -> impl IntoView {
    view! {
        <ComingLesson
            number="06"
            title="Verification & identity"
            eyebrow="WHAT DID WE ACTUALLY PROVE?"
            summary="Cryptography can give precise answers about keys and data. Connecting those answers to people, organizations, and truth requires additional evidence."
            problem="A signature verifies successfully against a public key. Does that prove Michael Snoyman signed the message? Not by itself."
            challenge="Use the verification workbench to establish exactly what a signature proves, then separate those facts from claims about who controls the key and whether the signed statement is true."
            points=&["Separate keys from identities", "Distinguish valid signatures from true statements", "Identify the trust assumptions outside the cryptography"]
        />
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
    problem: &'static str,
    challenge: &'static str,
    points: &'static [&'static str],
) -> impl IntoView {
    view! {
        <LessonIntro number eyebrow title summary />
        <section class="motivation-section content-section">
            <p class="eyebrow">"WHY WOULD I WANT THIS?"</p>
            <h2>{problem}</h2>
        </section>
        <section class="coming-section">
            <div>
                <p class="eyebrow">"LESSON IN DEVELOPMENT"</p>
                <h2>"The workbench will make you solve the problem."</h2>
                <p>"The interactive exercise will be built around the scenario above rather than around abstract definitions."</p>
            </div>
            <ol>{points.iter().map(|point| view! { <li>{*point}</li> }).collect_view()}</ol>
        </section>
        <section class="planned-quiz content-section">
            <p class="eyebrow">"PLANNED CHALLENGE"</p>
            <h2>"Use the tool, then answer."</h2>
            <p class="section-copy">{challenge}</p>
        </section>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <section class="not-found">
            <p class="eyebrow">"404"</p>
            <h1>"That lesson isn't here."</h1>
            <p class="lede">"The cryptography may be complicated. The navigation shouldn't be."</p>
            <A href="/" exact=true attr:class="button primary">"Back home"</A>
        </section>
    }
}

#[component]
fn SiteFooter() -> impl IntoView {
    view! {
        <footer>
            <span>"TryCrypto — an educational project by Michael Snoyman."</span>
            <span>
                <a href="https://github.com/veloxwarp/trycrypto" target="_blank" rel="noopener noreferrer">"Source on GitHub ↗"</a>
                " · Browser cryptography is for learning here, not production key management."
            </span>
        </footer>
    }
}
