(() => {
  const KIT_UID = "036ef99a3d";
  const KIT_SRC = "https://snoyberg.kit.com/036ef99a3d/index.js";

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

        // Revert before Leptos' normal input handler sees the value, so the
        // paired hex field never becomes stale or contradictory.
        input.value = lastValid;
        error.hidden = false;
        input.setAttribute("aria-invalid", "true");
      },
      true,
    );
  }

  function installKitSignup() {
    const section = Array.from(document.querySelectorAll("section.home-flow-section")).find(
      (candidate) => candidate.querySelector("h2")?.textContent?.trim() === "What can a signature really tell us?",
    );

    if (!section || section.querySelector(".kit-signup-host")) return;

    const host = document.createElement("div");
    host.className = "kit-signup-host";

    const copy = document.createElement("p");
    copy.textContent =
      "Interested in the protocol I'm building around these ideas? Join the list and I'll let you know when there's something worth showing.";
    host.appendChild(copy);

    const script = document.createElement("script");
    script.async = true;
    script.dataset.uid = KIT_UID;
    script.src = KIT_SRC;
    host.appendChild(script);

    section.appendChild(host);
  }

  function enhance() {
    installDecimalGuard();
    installKitSignup();
  }

  enhance();

  // Leptos swaps route content client-side, so re-apply enhancements whenever
  // the rendered page changes. Both installers are idempotent.
  const observer = new MutationObserver(enhance);
  observer.observe(document.body, { childList: true, subtree: true });
})();
