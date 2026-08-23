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
      for (const local of document.querySelectorAll(".exercise-hash-output")) {
        local.textContent = value;
      }
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
        <p>Try both candidates. The current result appears here, so you don't need to scroll back to the workbench.</p>
        <button type="button" data-candidate="a">Try candidate A</button>
        <button type="button" data-candidate="b">Try candidate B</button>
        <div class="output"><span>CURRENT SHA-256</span><code class="exercise-hash-output"></code></div>
        <div class="quiz-choice-row"><button type="button" data-match="a">A matches</button><button type="button" data-match="b">B matches</button></div>
        <p class="quiz-feedback" aria-live="polite"></p>
      </div>
      <div class="workbench-quiz" data-exercise="space">
        <h3>Does invisible whitespace count as different input?</h3>
        <p>Compare the clean sentence with a trailing space or newline.</p>
        <button type="button" data-space="clean">Clean text</button>
        <button type="button" data-space="space">Trailing space</button>
        <button type="button" data-space="newline">Trailing newline</button>
        <div class="output"><span>CURRENT SHA-256</span><code class="exercise-hash-output"></code></div>
        <div class="quiz-choice-row"><button type="button" data-space-answer="same">Same contents</button><button type="button" data-space-answer="different">Different contents</button></div>
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
    match.querySelectorAll("[data-candidate]").forEach(button => {
      button.addEventListener("click", () => {
        input.value = messages[button.dataset.candidate];
        update(input.value);
      });
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
    const values = {
      clean: "Backup finished successfully.",
      space: "Backup finished successfully. ",
      newline: "Backup finished successfully.\n",
    };
    space.querySelectorAll("[data-space]").forEach(button => {
      button.addEventListener("click", () => {
        input.value = values[button.dataset.space];
        update(input.value);
      });
    });
    space.querySelectorAll("[data-space-answer]").forEach(button => {
      button.addEventListener("click", () => {
        const ok = button.dataset.spaceAnswer === "different";
        space.querySelector(".quiz-feedback").textContent = ok
          ? "Right. A space or newline is part of the exact input."
          : "Compare the fingerprints. Cryptography depends on the exact input.";
        if (ok) spaceDone = true;
        finish();
      });
    });
  }

  new MutationObserver(install).observe(document.body, { childList: true, subtree: true });
  install();
})();
