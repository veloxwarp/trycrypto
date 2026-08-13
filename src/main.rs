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
                    <Route path=path!("complete") view=CompletionPage />
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
            <h1>"Cryptography is everywhere." <span>"Learn what it actually does."</span></h1>
            <p class="lede">
                "Every secure website you visit, every locked phone, and much of the software you rely on depends on cryptography."
            </p>
            <p class="lede">
                "The mathematics behind modern cryptography can be sophisticated. TryCrypto lets you use the tools yourself and learn what they guarantee—without needing to implement the algorithms."
            </p>

            <p class="course-intro">
                <strong>"Start learning."</strong>
                " The lessons below will help you learn the basics of how cryptography works, how it's used, and what its results actually prove."
            </p>
            <div class="course-list">
                <A href="/intro" exact=true attr:class="course-row course-row-start">
                    <span class="course-number">"INTRO"</span>
                    <strong>"Bytes & hexadecimal"</strong>
                    <span class="start-badge">"Start here →"</span>
                </A>
                {LESSONS
                    .iter()
                    .map(|lesson| {
                        view! {
                            <A
                                href=lesson.href
                                exact=true
                                attr:class="course-row"
                                attr:aria-label=format!("{} — {}", lesson.title, lesson.description)
                            >
                                <span class="course-number">{lesson.number}</span>
                                <strong>{lesson.title}</strong>
                                <span class="course-arrow">"→"</span>
                            </A>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        <section class="home-flow-section content-section">
            <p class="eyebrow">"About the project"</p>
            <h2>"Hi, I'm Michael."</h2>
            <p class="section-copy">"I'm Michael Snoyman, a software engineer, engineering leader, open-source developer, and author. I built TryCrypto because I wanted a straightforward way to explain the cryptographic building blocks we rely on without requiring people to start with the mathematics."</p>
            <div class="text-links">
                <a href="https://www.snoyman.com/" target="_blank" rel="noopener noreferrer">"About Michael ↗"</a>
                <a href="https://github.com/snoyberg" target="_blank" rel="noopener noreferrer">"GitHub ↗"</a>
                <a href="https://www.snoyman.com/blog/" target="_blank" rel="noopener noreferrer">"Blog ↗"</a>
            </div>
        </section>

        <section class="home-flow-section content-section">
            <p class="eyebrow">"Where this gets interesting"</p>
            <h2>"What can a signature really tell us?"</h2>
            <p class="section-copy">"Knowing how to verify a signature is one thing. Deciding what that signature means is much harder. I'm working on a new protocol built heavily around cryptographic evidence and those questions of identity, provenance, and trust. I'll share more when it's ready."</p>
            // TODO: Add a dedicated product-interest mailing-list signup form here before launch.
        </section>

        <section class="home-start-section content-section">
            <p class="eyebrow">"Ready to try it?"</p>
            <h2>"Start with the intro lesson."</h2>
            <p class="section-copy">"It takes just a few minutes to get comfortable with bytes and hexadecimal before moving on to hashes."</p>
            <A href="/intro" attr:class="button primary">"Start the intro →"</A>
        </section>
    }
}

#[component]
fn IntroLesson() -> impl IntoView {
    let (decimal_value, set_decimal_value) = signal(String::from("173"));
    let (hex_value, set_hex_value) = signal(String::from("AD"));
    let (hex_answer, set_hex_answer) = signal(String::new());
    let (hex_result, set_hex_result) = signal(Option::<bool>::None);
    let (decimal_answer, set_decimal_answer) = signal(String::new());
    let (decimal_result, set_decimal_result) = signal(Option::<bool>::None);

    let exercises_complete =
        Memo::new(move |_| hex_result.get() == Some(true) && decimal_result.get() == Some(true));

    view! {
        <LessonIntro
            number="INTRO"
            eyebrow="A few basics before we start"
            title="Bytes & hexadecimal"
            summary="Cryptographic tools work with bytes, and those bytes are often displayed in hexadecimal. You only need a little of both to follow the rest of TryCrypto."
        />

        <section class="content-section primer-section">
            <div class="section-heading">
                <p class="eyebrow">"Bytes"</p>
                <h2>"A byte is a number from 0 through 255."</h2>
                <p class="section-copy">"That's 256 possible values. Cryptographic keys, hashes, ciphertext, and other data are ultimately made up of bytes, so you'll see these values throughout the lessons."</p>
            </div>

            <div class="section-heading section-heading-spaced">
                <p class="eyebrow">"Hexadecimal"</p>
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

            <aside class="precision-note">
                <strong>"One idea, many algorithms."</strong>
                <p>"Hashing, encryption, and digital signatures each have multiple algorithms that perform the same general kind of job. In TryCrypto we'll use a few common modern examples—such as SHA-256 for hashing and AES-GCM for shared-secret encryption—so you can experiment with the concepts. You'll see other algorithm names in real software; the important thing here is understanding what kind of problem each tool solves and what it guarantees."</p>
            </aside>
        </section>

        <section class="workbench">
            <div class="workbench-heading">
                <div>
                    <p class="eyebrow">"Byte explorer"</p>
                    <h2>"Decimal and hex are two ways to show the same byte."</h2>
                </div>
                <p>"Type into either field. The other representation updates immediately."</p>
            </div>

            <div class="byte-converter mini-workbench">
                <label for="decimal-value">
                    <span>"Decimal (0–255)"</span>
                    <input
                        id="decimal-value"
                        type="text"
                        inputmode="numeric"
                        maxlength="3"
                        prop:value=move || decimal_value.get()
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            if value.chars().all(|c| c.is_ascii_digit()) {
                                set_decimal_value.set(value.clone());
                                if value.is_empty() {
                                    set_hex_value.set(String::new());
                                } else if let Ok(number) = value.parse::<u16>() {
                                    if number <= 255 {
                                        set_hex_value.set(format!("{number:02X}"));
                                    }
                                }
                            }
                        }
                    />
                </label>
                <span class="conversion-equals" aria-hidden="true">"="</span>
                <label for="hex-value">
                    <span>"Hex (00–FF)"</span>
                    <input
                        id="hex-value"
                        type="text"
                        maxlength="2"
                        prop:value=move || hex_value.get()
                        on:input=move |ev| {
                            let value = event_target_value(&ev).to_ascii_uppercase();
                            if value.len() <= 2 && value.chars().all(|c| c.is_ascii_hexdigit()) {
                                set_hex_value.set(value.clone());
                                if value.is_empty() {
                                    set_decimal_value.set(String::new());
                                } else if let Ok(number) = u8::from_str_radix(&value, 16) {
                                    set_decimal_value.set(number.to_string());
                                }
                            }
                        }
                    />
                </label>
            </div>
            <p class="microcopy">"Try values such as 10, 15, 16, 31, A5, AD, and FF and watch how the two representations correspond."</p>
        </section>

        <section id="intro-exercises" class="content-section planned-quiz">
            <p class="eyebrow">"Exercises"</p>
            <h2>"Use the explorer, then answer two questions."</h2>
            <p class="section-copy">"The goal isn't to memorize hex arithmetic. It's to get comfortable reading the representation and using the tool when you need it."</p>

            <div class="mini-workbench exercises-box">
                <div class="tool-quiz">
                    <p class="exercise-number">"1 of 2 · Decimal → hex"</p>
                    <h3>"What is decimal 200 in hex?"</h3>
                    <form
                        class="quiz-answer-row"
                        on:submit=move |ev| {
                            ev.prevent_default();
                            set_hex_result.set(Some(hex_answer.get().trim().eq_ignore_ascii_case("C8")));
                        }
                    >
                        <input
                            aria-label="Hex value for decimal 200"
                            placeholder="Your answer"
                            prop:value=move || hex_answer.get()
                            on:input=move |ev| {
                                set_hex_answer.set(event_target_value(&ev));
                                set_hex_result.set(None);
                            }
                        />
                        <button type="submit">"Check"</button>
                    </form>
                    <p class="quiz-feedback" aria-live="polite">
                        {move || match hex_result.get() {
                            Some(true) => "Correct. C8 is decimal 200.",
                            Some(false) => "Not quite. Put 200 into the decimal field above and check the hex value.",
                            None => "",
                        }}
                    </p>
                </div>

                <div class="tool-quiz">
                    <p class="exercise-number">"2 of 2 · Hex → decimal"</p>
                    <h3>"What is hex 7B in decimal?"</h3>
                    <form
                        class="quiz-answer-row"
                        on:submit=move |ev| {
                            ev.prevent_default();
                            set_decimal_result.set(Some(decimal_answer.get().trim() == "123"));
                        }
                    >
                        <input
                            aria-label="Decimal value for hex 7B"
                            placeholder="Your answer"
                            prop:value=move || decimal_answer.get()
                            on:input=move |ev| {
                                set_decimal_answer.set(event_target_value(&ev));
                                set_decimal_result.set(None);
                            }
                        />
                        <button type="submit">"Check"</button>
                    </form>
                    <p class="quiz-feedback" aria-live="polite">
                        {move || match decimal_result.get() {
                            Some(true) => "Correct. 7B is decimal 123.",
                            Some(false) => "Not quite. Put 7B into the hex field above and check the decimal value.",
                            None => "",
                        }}
                    </p>
                </div>

                <p class="exercise-progress" aria-live="polite">
                    {move || {
                        let completed = [hex_result.get(), decimal_result.get()]
                            .into_iter()
                            .filter(|result| *result == Some(true))
                            .count();
                        match completed {
                            0 => "Two exercises to go.".to_owned(),
                            1 => "One down, one to go.".to_owned(),
                            _ => "Both exercises complete.".to_owned(),
                        }
                    }}
                </p>
            </div>
        </section>

        <LessonEnd
            exercises_complete
            exercises_id="intro-exercises"
            next_href="/hashes"
            next_label="Continue to 01 — Hashes →"
        />
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
    let exercises_complete = Memo::new(move |_| challenge_result.get() == Some(true));

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
            eyebrow="Check that data arrived intact"
            title="Hashes"
            summary="A hash gives us a small fingerprint for data, even when the data itself is enormous."
        />

        <section class="motivation-section content-section">
            <p class="eyebrow">"Why would I want this?"</p>
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
                <div><p class="eyebrow">"Browser workbench"</p><h2>"Try SHA-256 yourself."</h2></div>
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
        </section>

        <section id="hash-exercises" class="content-section planned-quiz">
            <p class="eyebrow">"Exercises"</p>
            <h2>"Use the workbench, then answer."</h2>
            <div class="workbench-quiz">
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
            <p class="eyebrow">"What did this prove?"</p>
            <h2>"The hash checks the bytes—not their source or their truth."</h2>
            <p class="section-copy">"If I give you a trusted SHA-256 digest and your file produces the same digest, that's excellent evidence that you have the exact file I meant. But the hash itself cannot tell you who created the file or who gave you the digest."</p>
            <div class="principles">
                <article><span>"YES"</span><h3>"Integrity"</h3><p>"A hash lets us cheaply compare large amounts of data and detect accidental or deliberate changes."</p></article>
                <article><span>"NO"</span><h3>"No authenticated source"</h3><p>"If an attacker can replace both a file and the hash you compare it against, they can simply provide the correct hash of their replacement."</p></article>
                <article><span>"NO"</span><h3>"No claim of truth"</h3><p>"False information hashes just as reliably as true information."</p></article>
            </div>

            <aside class="side-note">
                <p class="eyebrow">"One more place hashes show up"</p>
                <h3>"Proof of work"</h3>
                <p>"Hashes are also useful when we want a task that is expensive to perform but cheap to verify. Proof-of-work systems repeatedly vary data and hash it until they find a result meeting a difficult condition. Checking the winning hash is easy; finding it required lots of trial and error."</p>
            </aside>
        </section>

        <LessonEnd
            exercises_complete
            exercises_id="hash-exercises"
            next_href="/symmetric-encryption"
            next_label="Continue to 02 — Shared-secret encryption →"
        />
    }
}

#[component]
fn SymmetricEncryptionPage() -> impl IntoView {
    let (exercise_done, set_exercise_done) = signal(false);
    let exercises_complete = Memo::new(move |_| exercise_done.get());

    view! {
        <LessonIntro
            number="02"
            title="Shared-secret encryption"
            eyebrow="One secret, two directions"
            summary="Encryption lets us transform readable data into ciphertext that only someone with the right secret can recover."
        />

        <section class="motivation-section content-section">
            <p class="eyebrow">"Why would I want this?"</p>
            <h2>"You and I already share a secret. How can I send you data that anyone may intercept, but only we can read?"</h2>
            <div class="prose-grid">
                <p>"Shared-secret encryption gives both of us the same encryption key. I use it to turn readable data into ciphertext; you use the same key to recover the original data. Someone who sees only the ciphertext should not be able to learn the message without the key."</p>
                <p>"The other person can also be future you. Imagine encrypting a backup before storing it in the cloud. You can protect it with a memorable password or passphrase today, then use the same passphrase to recover the backup years later."</p>
            </div>
            <aside class="precision-note">
                <strong>"A password is not normally used directly as an AES key."</strong>
                <p>"Human-chosen passwords are much easier to guess than random encryption keys. Instead, a password-based key derivation function takes the password or passphrase, a random salt, and deliberately expensive parameters and derives fixed-size key material for the encryption algorithm. The salt and parameters can be stored with the encrypted backup; the password is the part you must keep secret. We'll use the derived key with AES-GCM."</p>
            </aside>
        </section>

        <section class="coming-section">
            <div>
                <p class="eyebrow">"Browser workbench"</p>
                <h2>"Work in progress."</h2>
                <p>"The workbench will be built around both shared-key messaging and the password-protected backup scenario rather than around abstract definitions."</p>
            </div>
            <ol>
                <li>"Encrypt and decrypt with AES-GCM"</li>
                <li>"Derive encryption key material from a password or passphrase"</li>
                <li>"See why fresh nonces and salts matter"</li>
                <li>"Learn why modern encryption also detects tampering"</li>
            </ol>
        </section>

        <ReviewExercise
            description="The final exercises will have you derive a key from a passphrase, decrypt a protected backup, and determine which passphrase produces the valid plaintext. Then you'll change one input and observe what breaks."
            exercise_done
            set_exercise_done
        />

        <LessonEnd
            exercises_complete
            exercises_id="lesson-exercises"
            next_href="/keypairs"
            next_label="Continue to 03 — Public/private keypairs →"
        />
    }
}

#[component]
fn KeypairsPage() -> impl IntoView {
    view! {
        <ComingLesson
            number="03"
            title="Public/private keypairs"
            eyebrow="Two keys, different jobs"
            summary="Public-key cryptography separates material that is safe to publish from secret material that must remain under your control."
            problem="Shared-secret encryption is useful—but first we somehow had to share a secret. Can I publish something useful without publishing the secret that gives me control?"
            challenge="Generate a keypair in the workbench, then identify which operations still work when you keep only the public half and which require the private half."
            points=&["Generate a keypair in the browser", "Compare public and private material", "Understand what possession of each key permits"]
            next_href="/public-key-encryption"
            next_label="Continue to 04 — Public-key encryption →"
        />
    }
}

#[component]
fn PublicKeyEncryptionPage() -> impl IntoView {
    view! {
        <ComingLesson
            number="04"
            title="Public-key encryption"
            eyebrow="Encrypt for someone else"
            summary="A public key can let other people protect data for you without giving them the secret needed to decrypt it."
            problem="You need to send me confidential data, but we've never exchanged a shared secret. Can you encrypt something that only I can open?"
            challenge="Use two generated keypairs to encrypt for one recipient, then prove with the workbench that the other private key cannot decrypt the ciphertext."
            points=&["Encrypt using a recipient's public key", "Decrypt using the corresponding private key", "Contrast this with shared-secret encryption"]
            next_href="/signatures"
            next_label="Continue to 05 — Digital signatures →"
        />
    }
}

#[component]
fn SignaturesPage() -> impl IntoView {
    view! {
        <ComingLesson
            number="05"
            title="Digital signatures"
            eyebrow="Prove control of a key"
            summary="A signature binds a private key to exact data in a way that anyone with the public key can verify."
            problem="I publish a software release and you download it from somewhere else. How can you check that these exact bytes are the ones the holder of my private key approved?"
            challenge="Use the workbench to verify a signature, then change one character in the message and watch the same signature fail."
            points=&["Sign exact data with a private key", "Verify with the public key", "See how hashes and signatures fit together"]
            next_href="/verification"
            next_label="Continue to 06 — Verification & identity →"
        />
    }
}

#[component]
fn VerificationPage() -> impl IntoView {
    view! {
        <ComingLesson
            number="06"
            title="Verification & identity"
            eyebrow="What did we actually prove?"
            summary="Cryptography can give precise answers about keys and data. Connecting those answers to people, organizations, and truth requires additional evidence."
            problem="A signature verifies successfully against a public key. Does that prove Michael Snoyman signed the message? Not by itself."
            challenge="Use the verification workbench to establish exactly what a signature proves, then separate those facts from claims about who controls the key and whether the signed statement is true."
            points=&["Separate keys from identities", "Distinguish valid signatures from true statements", "Identify the trust assumptions outside the cryptography"]
            next_href="/complete"
            next_label="Finish TryCrypto →"
        />
    }
}

#[component]
fn CompletionPage() -> impl IntoView {
    view! {
        <LessonIntro
            number="DONE"
            eyebrow="You've reached the end"
            title="Now ask better questions."
            summary="Cryptography gives precise tools and precise guarantees. The interesting part is knowing where those guarantees stop."
        />

        <section class="content-section">
            <p class="eyebrow">"TryCrypto complete"</p>
            <h2>"From bytes to signatures—and then beyond the signatures."</h2>
            <p class="section-copy">"You've followed the path from representation and hashing through encryption, keypairs, signatures, and the harder questions of verification and identity. Revisit any lesson whenever you want to experiment with the tools again."</p>
            <div class="hero-actions">
                <A href="/" attr:class="button primary">"Back to the course"</A>
                <A href="/hashes" attr:class="button ghost">"Revisit hashes"</A>
            </div>
        </section>

        <section class="home-flow-section content-section">
            <p class="eyebrow">"Where this leads"</p>
            <h2>"What can a signature really tell us?"</h2>
            <p class="section-copy">"That's the question behind a new protocol I'm working on around cryptographic evidence, identity, provenance, and trust. I'll share more when there's something real to show."</p>
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
fn LessonEnd(
    exercises_complete: Memo<bool>,
    exercises_id: &'static str,
    next_href: &'static str,
    next_label: &'static str,
) -> impl IntoView {
    view! {
        <section class="lesson-end content-section">
            <Show
                when=move || exercises_complete.get()
                fallback=move || view! {
                    <div class="lesson-status">
                        <p class="eyebrow">"Exercises not completed"</p>
                        <h3>"Want to try them before moving on?"</h3>
                        <p>"They're optional, but they're where you use the lesson's tool for yourself instead of only reading about it."</p>
                        <div class="hero-actions">
                            <a class="button primary" href=format!("#{exercises_id}")>"Go to exercises ↑"</a>
                            <A href=next_href attr:class="button ghost">"Skip exercises and continue →"</A>
                        </div>
                    </div>
                }
            >
                <div class="lesson-status lesson-status-complete">
                    <p class="eyebrow">"Exercises complete"</p>
                    <h3>"Nice work. You're ready to move on."</h3>
                    <p>"You've used the lesson's tool yourself and completed the exercises."</p>
                    <A href=next_href attr:class="button primary">{next_label}</A>
                </div>
            </Show>
        </section>
    }
}

#[component]
fn ReviewExercise(
    description: &'static str,
    exercise_done: ReadSignal<bool>,
    set_exercise_done: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <section id="lesson-exercises" class="content-section planned-quiz">
            <p class="eyebrow">"Exercises"</p>
            <h2>"Use the workbench, then answer."</h2>
            <p class="section-copy">{description}</p>
            <div class="mini-workbench exercises-box">
                <p class="exercise-number">"Temporary review exercise"</p>
                <h3>"Complete this placeholder to review the lesson flow."</h3>
                <p class="section-copy">"This control is intentionally temporary. It will be replaced by a real workbench-driven exercise before this PR is merged."</p>
                <button type="button" on:click=move |_| set_exercise_done.set(true)>
                    "Mark exercise complete"
                </button>
                <p class="quiz-feedback" aria-live="polite">
                    {move || if exercise_done.get() {
                        "Exercise complete."
                    } else {
                        ""
                    }}
                </p>
            </div>
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
    next_href: &'static str,
    next_label: &'static str,
) -> impl IntoView {
    let (exercise_done, set_exercise_done) = signal(false);
    let exercises_complete = Memo::new(move |_| exercise_done.get());

    view! {
        <LessonIntro number eyebrow title summary />
        <section class="motivation-section content-section">
            <p class="eyebrow">"Why would I want this?"</p>
            <h2>{problem}</h2>
        </section>
        <section class="coming-section">
            <div>
                <p class="eyebrow">"Browser workbench"</p>
                <h2>"Work in progress."</h2>
                <p>"The interactive tool will be built around the scenario above rather than around abstract definitions."</p>
            </div>
            <ol>{points.iter().map(|point| view! { <li>{*point}</li> }).collect_view()}</ol>
        </section>
        <ReviewExercise description=challenge exercise_done set_exercise_done />
        <LessonEnd
            exercises_complete
            exercises_id="lesson-exercises"
            next_href
            next_label
        />
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
            <div class="footer-primary">
                <span>"TryCrypto — an educational project by "</span>
                <a href="https://www.snoyman.com/" target="_blank" rel="noopener noreferrer">"Michael Snoyman"</a>
                <span>"."</span>
            </div>
            <div class="footer-links">
                <a href="https://www.snoyman.com/" target="_blank" rel="noopener noreferrer">"Homepage ↗"</a>
                <a href="https://github.com/snoyberg" target="_blank" rel="noopener noreferrer">"GitHub ↗"</a>
                <a href="https://www.snoyman.com/blog/" target="_blank" rel="noopener noreferrer">"Blog ↗"</a>
                <a href="https://github.com/veloxwarp/trycrypto" target="_blank" rel="noopener noreferrer">"Source ↗"</a>
            </div>
            <p class="footer-note">"Browser cryptography is for learning here, not production key management."</p>
        </footer>
    }
}
