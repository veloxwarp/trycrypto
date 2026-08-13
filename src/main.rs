mod course;
mod crypto;
mod curve;
mod hash_exercises;
mod hash_workbench;
mod hashes;
mod home;
mod intro;
mod intro_exercises;
mod key_math;
mod keypair_exercises;
mod keypairs;
mod public_key;
mod public_key_lab;
mod signatures;
mod signatures_lab;
mod symmetric;
mod verification;
mod verification_lab;

use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

pub use course::{LessonEnd, LessonIntro};

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <SiteHeader />
            <main>
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("") view=home::HomePage />
                    <Route path=path!("index.html") view=home::HomePage />
                    <Route path=path!("intro") view=intro::IntroLesson />
                    <Route path=path!("hashes") view=hashes::HashLesson />
                    <Route path=path!("symmetric-encryption") view=symmetric::SymmetricEncryptionLesson />
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
fn KeypairsPage() -> impl IntoView {
    view! {
        <keypairs::KeypairsLesson />
        <keypair_exercises::KeypairExercises />
    }
}

#[component]
fn PublicKeyEncryptionPage() -> impl IntoView {
    view! {
        <public_key::PublicKeyEncryptionIntro />
        <public_key_lab::PublicKeyLab />
    }
}

#[component]
fn SignaturesPage() -> impl IntoView {
    view! {
        <signatures::SignaturesIntro />
        <signatures_lab::SignaturesLab />
    }
}

#[component]
fn VerificationPage() -> impl IntoView {
    view! {
        <verification::VerificationIntro />
        <verification_lab::VerificationLab />
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
                <A href="/hashes" exact=true>"01 Hashes"</A>
                <A href="/symmetric-encryption" exact=true>"02 Shared key"</A>
                <A href="/keypairs" exact=true>"03 Public/private"</A>
                <A href="/public-key-encryption" exact=true>"04 Public key"</A>
                <A href="/signatures" exact=true>"05 Signatures"</A>
                <A href="/verification" exact=true>"06 Verification"</A>
            </nav>
        </header>
    }
}

#[component]
fn CompletionPage() -> impl IntoView {
    view! {
        <LessonIntro
            number="DONE"
            eyebrow=""
            title="Now ask better questions."
            summary="Cryptography gives precise tools and precise guarantees. The interesting part is knowing where those guarantees stop."
        />
        <section class="content-section">
            <h2>"You've reached the end of TryCrypto."</h2>
            <p class="section-copy">"You've gone from bytes and hashes through shared-key encryption, public/private keys, public-key encryption, signatures, and the harder question of what verification actually establishes."</p>
            <div class="hero-actions">
                <A href="/" attr:class="button primary">"Back to the course"</A>
                <A href="/hashes" attr:class="button ghost">"Revisit hashes"</A>
            </div>
        </section>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <section class="not-found">
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
