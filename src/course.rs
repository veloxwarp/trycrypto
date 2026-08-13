use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn LessonIntro(
    number: &'static str,
    eyebrow: &'static str,
    title: &'static str,
    summary: &'static str,
) -> impl IntoView {
    let _ = eyebrow;
    view! {
        <section class="lesson-hero">
            <div class="lesson-index">{number}</div>
            <h1>{title}</h1>
            <p class="lede">{summary}</p>
        </section>
    }
}

#[component]
pub fn LessonEnd(
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
                        <h3>"Exercises not completed"</h3>
                        <p>"They're optional, but using the tool yourself is the best way to make the lesson stick."</p>
                        <div class="hero-actions">
                            <a class="button primary" href=format!("#{exercises_id}")>"Go to exercises ↑"</a>
                            <A href=next_href attr:class="button ghost">"Skip exercises and continue →"</A>
                        </div>
                    </div>
                }
            >
                <div class="lesson-status lesson-status-complete">
                    <h3>"Exercises complete — you're ready to continue."</h3>
                    <p>"Nice work. You've used the lesson's tool yourself."</p>
                    <A href=next_href attr:class="button primary">{next_label}</A>
                </div>
            </Show>
        </section>
    }
}
