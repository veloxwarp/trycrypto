use crate::{LessonEnd, curve};
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn SignaturesLab() -> impl IntoView {
    let (pair, set_pair) = signal(Option::<curve::KeyPair>::None);
    let (message, set_message) = signal("Release 1.0 is approved.".to_owned());
    let (signing_private, set_signing_private) = signal(String::new());
    let (signed_message, set_signed_message) = signal(String::new());
    let (signature, set_signature) = signal(String::new());
    let (verify_public, set_verify_public) = signal(String::new());
    let (verify_message, set_verify_message) = signal(String::new());
    let (verify_signature, set_verify_signature) = signal(String::new());
    let (feedback, set_feedback) = signal(String::new());
    let (signed_seen, set_signed_seen) = signal(false);
    let (valid_seen, set_valid_seen) = signal(false);
    let (changed_seen, set_changed_seen) = signal(false);
    let complete = Memo::new(move |_| signed_seen.get() && valid_seen.get() && changed_seen.get());

    view! {
        <section class="workbench">
            <div class="workbench-heading">
                <div><h2>"Sign, then verify the exact message."</h2></div>
                <p>"Generate a signing keypair, then copy its values through the signing and verification steps yourself."</p>
            </div>

            <div class="mini-workbench">
                <p class="exercise-number">"Signing key"</p>
                <button
                    type="button"
                    class="button primary"
                    on:click=move |_| spawn_local(async move {
                        match curve::generate_ed25519().await {
                            Ok(value) => {
                                set_pair.set(Some(value));
                                set_signature.set(String::new());
                                set_signed_message.set(String::new());
                                set_signed_seen.set(false);
                                set_valid_seen.set(false);
                                set_changed_seen.set(false);
                                set_feedback.set("Generated a fresh Ed25519 signing keypair.".to_owned());
                            }
                            Err(_) => set_feedback.set("Couldn't generate an Ed25519 keypair in this browser.".to_owned()),
                        }
                    })
                >"Generate signing keypair"</button>
                <div class="output">
                    <span>"PUBLIC KEY · 32 BYTES · SHAREABLE"</span>
                    <code>{move || pair.get().map(|p|p.public_hex).unwrap_or_else(||"—".to_owned())}</code>
                </div>
                <div class="output">
                    <span>"PRIVATE KEY · 32 BYTES · KEEP SECRET"</span>
                    <code>{move || pair.get().map(|p|p.private_hex).unwrap_or_else(||"—".to_owned())}</code>
                </div>
            </div>

            <div class="workbench-quiz">
                <p class="exercise-number">"Step 1 · Sign"</p>
                <h3>"Sign an exact message."</h3>
                <div class="mini-workbench">
                    <label for="signing-private-key">"Private key"</label>
                    <div class="paste-input-row" data-pasteable="private key"><input id="signing-private-key" maxlength="64" prop:value=move||signing_private.get() on:input=move|ev|set_signing_private.set(event_target_value(&ev)) /></div>
                    <label for="signature-message">"Message"</label>
                    <input
                        id="signature-message"
                        prop:value=move||message.get()
                        on:input=move|ev|set_message.set(event_target_value(&ev))
                    />
                    <button
                        type="button"
                        class="button primary"
                        disabled=move||pair.get().is_none()
                        on:click=move |_| {
                            if let Some(p)=pair.get().filter(|p|p.private_hex==signing_private.get().trim()){
                                let key=p.private;
                                let msg=message.get();
                                let remembered=msg.clone();
                                spawn_local(async move{
                                    match curve::sign(&key,&msg).await {
                                        Ok(sig)=>{
                                            set_signature.set(sig);
                                            set_signed_message.set(remembered);
                                            set_signed_seen.set(true);
                                            set_valid_seen.set(false);
                                            set_changed_seen.set(false);
                                            set_feedback.set("Signed the exact message above.".to_owned());
                                        }
                                        Err(_)=>set_feedback.set("Signing failed.".to_owned()),
                                    }
                                });
                            } else { set_feedback.set("Copy the generated private key into the field above.".to_owned()); }
                        }
                    >"Sign message"</button>
                    <div class="output"><span>"SIGNED MESSAGE"</span><code>{move||{let value=signed_message.get();if value.is_empty(){"—".to_owned()}else{value}}}</code></div>
                    <div class="output">
                        <span>"SIGNATURE · 64 BYTES"</span>
                        <code>{move||{let value=signature.get();if value.is_empty(){"—".to_owned()}else{value}}}</code>
                    </div>
                </div>
            </div>
        </section>

        <section id="signature-exercises" class="content-section planned-quiz">
            <h2>"Exercises"</h2>
            <p class="section-copy">"Create a signature yourself, verify the signed message, then change the message and verify again."</p>
            <div class="mini-workbench exercise-checklist">
                <p>{move||if signed_seen.get(){"✓ Sign a message"}else{"○ Sign a message"}}</p>
                <p>{move||if valid_seen.get(){"✓ Verify the exact signed message"}else{"○ Verify the exact signed message"}}</p>
                <p>{move||if changed_seen.get(){"✓ Change the message and observe verification fail"}else{"○ Change the message and verify again"}}</p>
            </div>

            <div class="workbench-quiz">
                <p class="exercise-number">"Step 2 · Verify"</p>
                <h3>"Verify the signed message."</h3>
                <div class="mini-workbench">
                    <label for="verification-public-key">"Public key"</label><div class="paste-input-row" data-pasteable="public key"><input id="verification-public-key" maxlength="64" prop:value=move||verify_public.get() on:input=move|ev|set_verify_public.set(event_target_value(&ev)) /></div>
                    <label for="verification-message">"Message"</label><div class="paste-input-row" data-pasteable="signed message"><input id="verification-message" prop:value=move||verify_message.get() on:input=move|ev|set_verify_message.set(event_target_value(&ev)) /></div>
                    <label for="verification-signature">"Signature"</label><div class="paste-input-row" data-pasteable="signature"><textarea id="verification-signature" prop:value=move||verify_signature.get() on:input=move|ev|set_verify_signature.set(event_target_value(&ev))></textarea></div>
                <button
                    type="button"
                    disabled=move||!signed_seen.get()
                    on:click=move |_| {
                        if let Some(p)=pair.get().filter(|p|p.public_hex==verify_public.get().trim()){
                            let key=p.public;
                            let msg=verify_message.get();
                            let sig=verify_signature.get();
                            let expected=signed_message.get();
                            spawn_local(async move{
                                match curve::verify(&key,&msg,&sig).await {
                                    Ok(true)=>{
                                        if msg==expected { set_valid_seen.set(true); }
                                        set_feedback.set("Valid signature for this exact message.".to_owned());
                                    }
                                    Ok(false)=>set_feedback.set("Signature does not match this message.".to_owned()),
                                    Err(_)=>set_feedback.set("Verification failed.".to_owned()),
                                }
                            });
                        } else { set_feedback.set("Copy the generated public key into the field above.".to_owned()); }
                    }
                >"Verify"</button>
                </div>
            </div>

            <div class="workbench-quiz">
                <p class="exercise-number">"Step 3 · Change the message"</p>
                <h3>"Change even one character and verify again."</h3>
                <div class="hero-actions">
                    <button type="button" disabled=move||!signed_seen.get() on:click=move |_|set_verify_message.update(|m|m.push('!'))>"Add one character"</button>
                    <button
                        type="button"
                        disabled=move||!signed_seen.get()
                        on:click=move |_| {
                            if let Some(p)=pair.get().filter(|p|p.public_hex==verify_public.get().trim()){
                                let key=p.public;
                                let msg=verify_message.get();
                                let sig=verify_signature.get();
                                let expected=signed_message.get();
                                spawn_local(async move{
                                    match curve::verify(&key,&msg,&sig).await {
                                        Ok(false) if msg!=expected=>{
                                            set_changed_seen.set(true);
                                            set_feedback.set("Changed message: the original signature is no longer valid.".to_owned());
                                        }
                                        Ok(true)=>set_feedback.set("This is still the exact message that was signed. Change it first.".to_owned()),
                                        Ok(false)=>set_feedback.set("Verification failed, but change the signed message itself to complete this exercise.".to_owned()),
                                        Err(_)=>set_feedback.set("Verification failed.".to_owned()),
                                    }
                                });
                            } else { set_feedback.set("Copy the generated public key into the verification field first.".to_owned()); }
                        }
                    >"Verify changed message"</button>
                </div>
            </div>
            <p class="quiz-feedback" aria-live="polite">{move||feedback.get()}</p>
        </section>

        <LessonEnd exercises_complete=complete exercises_id="signature-exercises" next_href="/verification-and-identity" next_label="Continue to 06 — Verification & identity →" />
    }
}
