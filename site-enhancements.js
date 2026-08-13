(() => {
  const KIT_UID = "036ef99a3d";
  const KIT_SRC = "https://snoyberg.kit.com/036ef99a3d/index.js";

  function loadScript(src) {
    const id = `trycrypto-${src.replace(/[^a-z0-9]/gi, "-")}`;
    const existing = document.getElementById(id);
    if (existing) {
      return existing.dataset.loaded === "true"
        ? Promise.resolve()
        : new Promise((resolve) => existing.addEventListener("load", resolve, { once: true }));
    }

    return new Promise((resolve, reject) => {
      const script = document.createElement("script");
      script.id = id;
      script.src = src;
      script.onload = () => {
        script.dataset.loaded = "true";
        resolve();
      };
      script.onerror = reject;
      document.head.appendChild(script);
    });
  }

  function installLessonLabs() {
    const path = window.location.pathname;
    if (path === "/keypairs") {
      loadScript("/assets/curve.js")
        .then(() => loadScript("/assets/lesson3.js"))
        .then(() => loadScript("/assets/lesson3-exercises.js"));
    } else if (path === "/public-key-encryption") {
      loadScript("/assets/curve.js")
        .then(() => loadScript("/assets/lesson4-ui.js"))
        .then(() => loadScript("/assets/lesson4-actions.js"))
        .then(() => loadScript("/assets/lesson4-exercises.js"));
    }
  }

  function installDecimalGuard() {
    const input = document.getElementById("decimal-value");
    if (!input || input.dataset.trycryptoGuarded === "true") return;

    input.dataset.trycryptoGuarded = "true";
    let lastValid = input.value;

    const error = document.createElement("p");
    error.className = "field-error";
    error.id = "decimal-value-error";
    error.hidden = true;
    error.textContent = "Enter a decimal value from 0 through 255.";
    input.insertAdjacentElement("afterend", error);
    input.setAttribute("aria-describedby", error.id);

    input.addEventListener(
      "input",
      () => {
        const value = input.value;
        const valid = value === "" || (/^\d+$/.test(value) && Number(value) <= 255);
        if (valid) {
          lastValid = value;
          error.hidden = true;
          input.removeAttribute("aria-invalid");
          return;
        }

        input.value = lastValid;
        error.hidden = false;
        input.setAttribute("aria-invalid", "true");
      },
      true,
    );
  }

  function refineHomepage() {
    if (window.location.pathname !== "/") return;
    const section = document.getElementById("protocol-interest") ||
      Array.from(document.querySelectorAll("section.home-flow-section")).find(
        (candidate) => candidate.querySelector("h2")?.textContent?.trim() === "What can a signature really tell us?",
      );
    if (!section) return;

    section.id = "protocol-interest";
    if (section.dataset.refined !== "true") {
      section.dataset.refined = "true";
      const heading = section.querySelector("h2");
      if (heading) heading.textContent = "Cryptography can prove some facts. It can't prove everything.";

      const first = section.querySelector(".section-copy");
      if (first) {
        first.textContent = "I'm working on a protocol that uses cryptographic evidence to help build networks of trust. I'll be sharing more information as the work develops.";
      }

      const privacy = document.createElement("p");
      privacy.className = "section-copy";
      privacy.textContent = "If you'd like to be notified when there's something new, sign up below. I'll only send relevant updates about this work, and I won't sell your information or share it with marketers.";
      section.appendChild(privacy);

      const blog = document.createElement("p");
      blog.className = "section-copy";
      blog.innerHTML = 'Alternatively, <a href="https://www.snoyman.com/blog/" target="_blank" rel="noopener noreferrer">follow updates on my blog ↗</a>';
      section.appendChild(blog);
    }

    if (section.querySelector(".kit-signup-host")) return;
    const host = document.createElement("div");
    host.className = "kit-signup-host";
    const script = document.createElement("script");
    script.async = true;
    script.dataset.uid = KIT_UID;
    script.src = KIT_SRC;
    host.appendChild(script);
    section.appendChild(host);
  }

  function refineIntro() {
    if (window.location.pathname !== "/intro") return;
    for (const note of document.querySelectorAll(".precision-note")) {
      if (note.querySelector("strong")?.textContent?.includes("One idea, many algorithms")) {
        note.remove();
      }
    }
  }

  function refineHashes() {
    if (window.location.pathname !== "/hashes") return;
    const motivation = document.querySelector(".motivation-section");
    if (motivation && motivation.dataset.refined !== "true") {
      motivation.dataset.refined = "true";
      const prose = motivation.querySelector(".prose-grid");
      if (prose) {
        prose.innerHTML = `
          <p>Comparing a huge restored backup with the original byte by byte is possible, but inconvenient. We want a small value that depends on all of the data.</p>
          <p>A cryptographic hash reads the entire input and produces a fixed-size fingerprint. The same input produces the same fingerprint. Change the input—even slightly—and the fingerprint changes.</p>
          <p>That gives us a practical backup check: save the fingerprint when the backup is created, then calculate the fingerprint again after restoring it. Matching fingerprints are extremely strong evidence that the contents stayed exactly the same. The same technique works for a file received from someone else, provided the expected fingerprint came from somewhere you trust.</p>`;
      }
    }

    if (!document.getElementById("sha256-explainer")) {
      const explanation = document.querySelector(".lesson-explanation");
      if (explanation) {
        const section = document.createElement("section");
        section.id = "sha256-explainer";
        section.className = "content-section";
        section.innerHTML = `
          <h2>What's SHA-256?</h2>
          <p class="section-copy">SHA-256 is the hash algorithm used in this lesson. It produces 256 bits of output: 32 bytes, displayed here as 64 hexadecimal digits.</p>
          <p class="section-copy">It is one of many hash algorithms. Throughout TryCrypto we'll choose common algorithms for the exercises, but there are alternatives to each kind of cryptographic tool we use.</p>`;
        explanation.before(section);
      }
    }

    const source = document.querySelector(".hash-tool .output code");
    if (!source) return;
    const quizzes = document.querySelectorAll("#hash-exercises .workbench-quiz");
    for (const quiz of quizzes) {
      if (quiz.querySelector(".local-hash-preview")) continue;
      const preview = document.createElement("div");
      preview.className = "output local-hash-preview";
      preview.innerHTML = '<span>CURRENT WORKBENCH SHA-256</span><code></code>';
      const choices = quiz.querySelector(".quiz-choice-row");
      if (choices) choices.before(preview); else quiz.appendChild(preview);
    }

    const mirror = () => {
      for (const code of document.querySelectorAll(".local-hash-preview code")) {
        code.textContent = source.textContent;
      }
    };
    mirror();
    if (source.dataset.mirroring !== "true") {
      source.dataset.mirroring = "true";
      new MutationObserver(mirror).observe(source, { childList: true, subtree: true, characterData: true });
    }
  }

  function explainNonce() {
    if (window.location.pathname !== "/symmetric-encryption") return;
    const plaintext = document.getElementById("encrypt-plaintext");
    const box = plaintext?.closest(".mini-workbench");
    if (!box || box.querySelector(".nonce-explanation")) return;

    const outputs = box.querySelectorAll(".output");
    const nonceOutput = outputs[0];
    const label = nonceOutput?.querySelector("span");
    if (label) label.textContent = "NONCE · FRESH RANDOM 12 BYTES · NOT SECRET";

    const note = document.createElement("p");
    note.className = "section-copy nonce-explanation";
    note.textContent = "The nonce is a separate one-time value generated for this encryption. It isn't derived from the plaintext and it doesn't need to be secret. Keep it with the ciphertext; AES-GCM needs the same nonce to decrypt, and a fresh nonce must be used each time with a given key.";
    if (nonceOutput) nonceOutput.before(note);
  }

  function enhance() {
    installDecimalGuard();
    refineHomepage();
    refineIntro();
    refineHashes();
    explainNonce();
    installLessonLabs();
  }

  enhance();
  const observer = new MutationObserver(enhance);
  observer.observe(document.body, { childList: true, subtree: true });
})();
