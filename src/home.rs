use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="home-hero">
            <h1>"Cryptography is everywhere." <span>"Learn what it actually does."</span></h1>
            <p class="lede">"Every secure website you visit, every locked phone, and much of the software you rely on depends on cryptography."</p>
            <p class="lede">"The mathematics can be sophisticated. TryCrypto lets you use the tools yourself and learn what they guarantee without needing to implement the algorithms."</p>
            <p class="course-intro"><strong>"Start learning."</strong> " The lessons below cover the basics of how cryptography works, how it's used, and where its guarantees stop."</p>
            <div class="course-list">
                <A href="/bytes-and-hexadecimal" attr:class="course-row course-row-start"><span class="course-number">"INTRO"</span><strong>"Bytes & hexadecimal"</strong><span class="start-badge">"Start here →"</span></A>
                <A href="/hashes" attr:class="course-row"><span class="course-number">"01"</span><strong>"Hashes"</strong><span class="course-arrow">"→"</span></A>
                <A href="/shared-key-encryption" attr:class="course-row"><span class="course-number">"02"</span><strong>"Shared-key encryption"</strong><span class="course-arrow">"→"</span></A>
                <A href="/public-key" attr:class="course-row"><span class="course-number">"03"</span><strong>"Public key"</strong><span class="course-arrow">"→"</span></A>
                <A href="/public-key-encryption" attr:class="course-row"><span class="course-number">"04"</span><strong>"Public-key encryption"</strong><span class="course-arrow">"→"</span></A>
                <A href="/digital-signatures" attr:class="course-row"><span class="course-number">"05"</span><strong>"Digital signatures"</strong><span class="course-arrow">"→"</span></A>
                <A href="/verification-and-identity" attr:class="course-row"><span class="course-number">"06"</span><strong>"Verification & identity"</strong><span class="course-arrow">"→"</span></A>
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
