use leptos::prelude::*;

#[component]
pub fn HashWorkbench() -> impl IntoView {
    view! {
        <section class="workbench">
            <div class="workbench-heading">
                <h2>"Try SHA-256 yourself."</h2>
                <p>"Change any character, space, or newline and watch the fingerprint change."</p>
            </div>
            <div class="hash-tool">
                <label for="hash-input"><span>"INPUT"</span></label>
                <div class="paste-input-row" data-pasteable="hash input"><textarea id="hash-input">"The quick brown fox jumps over the lazy dog"</textarea></div>
                <div class="arrow">"↓"</div>
                <div class="output" aria-live="polite">
                    <span>"SHA-256 · 32 BYTES · 64 HEX DIGITS"</span>
                    <code id="hash-output">"Calculating…"</code>
                </div>
            </div>
        </section>
    }
}
