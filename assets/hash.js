(() => {
  async function digest(text) {
    const data = new TextEncoder().encode(text);
    const result = await crypto.subtle.digest("SHA-256", data);
    return Array.from(new Uint8Array(result), byte => byte.toString(16).padStart(2, "0")).join("");
  }

  function install() {
    if (location.pathname !== "/hashes") return;
    const input = document.getElementById("hash-input");
    const output = document.getElementById("hash-output");
    const host = document.getElementById("hash-exercise-content");
    if (!input || !output || !host) return;

    const update = async (text = input.value) => {
      const value = await digest(text);
      output.textContent = value;
      return value;
    };

    if (input.dataset.bound !== "true") {
      input.dataset.bound = "true";
      input.addEventListener("input", () => update());
      update();
    }

    if (host.dataset.bound === "true") return;
    host.dataset.bound = "true";
    host.innerHTML = `
      <div class="workbench-quiz" data-exercise="match">
        <h3>Which message matches this target fingerprint?</h3>
        <code class="target-hash">6c1e614182df466a0629118845873531184affe88aeb240ed834301a82908f47</code>
        <p>Each message and its SHA-256 are shown together. Copy a message into the workbench above if you want to calculate it yourself.</p>
        <div class="candidate-grid">
          <div>
            <div class="output"><span>MESSAGE A</span><code>Meet me at 10:30 by the north entrance.</code></div>
            <div class="output"><span>MESSAGE A · SHA-256</span><code data-candidate-hash="a">Not calculated yet</code></div>
          </div>
          <div>
            <div class="output"><span>MESSAGE B</span><code>Meet me at 10:30 by the south entrance.</code></div>
            <div class="output"><span>MESSAGE B · SHA-256</span><code data-candidate-hash="b">Not calculated yet</code></div>
          </div>
        </div>
        <div class="quiz-choice-row"><button type="button" data-match="a">A matches</button><button type="button" data-match="b">B matches</button></div>
        <p class="quiz-feedback" aria-live="polite"></p>
      </div>
      <div class="workbench-quiz" data-exercise="space">
        <h3>Do spaces and line breaks count as part of the input?</h3>
        <p>Click in the box below. Put the cursor after the period, then press the space bar a few times or press Enter to add a new line. Watch what happens to the fingerprint.</p>
        <label for="whitespace-exercise-input"><span>EDIT THIS MESSAGE</span><textarea id="whitespace-exercise-input">Backup finished successfully.</textarea></label>
        <div class="output"><span>THIS EXACT MESSAGE · SHA-256</span><code class="exercise-space-hash">Calculating…</code></div>
        <div class="quiz-choice-row"><button type="button" data-space-answer="same">The input stayed the same</button><button type="button" data-space-answer="different">The input changed</button></div>
        <p class="quiz-feedback" aria-live="polite"></p>
      </div>`;

    let matchDone = false;
    let spaceDone = false;
    const finish = () => {
      if (matchDone && spaceDone) document.getElementById("hash-complete")?.click();
    };

    const match = host.querySelector('[data-exercise="match"]');
    const messages = {
      a: "Meet me at 10:30 by the north entrance.",
      b: "Meet me at 10:30 by the south entrance.",
    };
    Object.entries(messages).forEach(async ([candidate, message]) => {
      match.querySelector(`[data-candidate-hash="${candidate}"]`).textContent = await digest(message);
    });
    match.querySelectorAll("[data-match]").forEach(button => {
      button.addEventListener("click", () => {
        const ok = button.dataset.match === "a";
        match.querySelector(".quiz-feedback").textContent = ok
          ? "Correct. Candidate A matches the target fingerprint."
          : "Try both candidates and compare their fingerprints.";
        if (ok) matchDone = true;
        finish();
      });
    });

    const space = host.querySelector('[data-exercise="space"]');
    const spaceInput = space.querySelector("textarea");
    const spaceHash = space.querySelector(".exercise-space-hash");
    let spaceEdited = false;
    const updateSpace = async () => {
      const value = await digest(spaceInput.value);
      spaceHash.textContent = value;
    };
    updateSpace();
    spaceInput.addEventListener("input", () => {
      spaceEdited = spaceInput.value !== "Backup finished successfully.";
      updateSpace();
    });
    space.querySelectorAll("[data-space-answer]").forEach(button => {
      button.addEventListener("click", () => {
        const ok = button.dataset.spaceAnswer === "different" && spaceEdited;
        space.querySelector(".quiz-feedback").textContent = ok
          ? "Right. Spaces and line breaks are characters too, even when they are easy to overlook. Cryptography uses the exact input."
          : spaceEdited
            ? "Compare the fingerprints. Those extra characters changed the input."
            : "First add a few spaces or press Enter in the message box, then compare the fingerprint.";
        if (ok) spaceDone = true;
        finish();
      });
    });
  }

  new MutationObserver(install).observe(document.body, { childList: true, subtree: true });
  install();
})();
