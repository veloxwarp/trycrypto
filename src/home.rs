use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="home-hero">
            <h1>"Cryptography is everywhere." <span>"Learn what it actually does."</span></h1>
            <p class="lede">"Every secure website you visit, every locked phone, and much of the software you rely on depends on cryptography."</p>
            <p class="lede">"TryCrypto lets you use cryptographic tools yourself and learn what they guarantee without needing to implement the algorithms."</p>
            <p class="course-intro"><strong>"Start learning."</strong> " Work through the lessons in order, beginning with bytes and hexadecimal."</p>
            <A href="/intro" attr:class="button primary">"Start the intro →"</A>
        </section>

        <section class="home-flow-section content-section">
            <h2>"Hi, I'm Michael."</h2>
            <p class="section-copy">"I'm Michael Snoyman, a software engineer, open-source developer, engineering leader, and author. I built TryCrypto as a straightforward introduction to the cryptographic building blocks we rely on."</p>
        </section>

        <section id="protocol-interest" class="home-flow-section content-section">
            <h2>"Cryptography can prove some facts. It can't prove everything."</h2>
            <p class="section-copy">"I'm working on a protocol that uses cryptographic evidence to help build networks of trust. I'll be sharing more information as the work develops."</p>
            <p class="section-copy">"If you'd like to be notified when there's something new, join the mailing list below. I'll only use it for relevant updates about this work, and I won't sell your information or share it with marketers."</p>
            <a href="https://www.snoyman.com/blog/" target="_blank" rel="noopener noreferrer">"Or follow updates on my blog ↗"</a>
        </section>

        <section class="home-start-section content-section">
            <h2>"Ready to try it? Start with the intro lesson."</h2>
            <A href="/intro" attr:class="button primary">"Start the intro →"</A>
        </section>
    }
}
