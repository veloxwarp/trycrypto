(() => {
  const KIT_SRC = "https://snoyberg.kit.com/036ef99a3d/index.js";

  function loadScript(src) {
    const id = `trycrypto-${src.replace(/[^a-z0-9]/gi, "-")}`;
    if (document.getElementById(id)) return;
    const script = document.createElement("script");
    script.id = id;
    script.src = src;
    document.head.appendChild(script);
  }

  function installByteConverter() {
    const decimal = document.getElementById("decimal-value");
    const hex = document.getElementById("hex-value");
    if (!decimal || !hex || decimal.dataset.trycryptoBound === "true") return;
    decimal.dataset.trycryptoBound = "true";

    let error = document.getElementById("byte-converter-error");
    if (!error) {
      error = document.createElement("p");
      error.id = "byte-converter-error";
      error.className = "field-error";
      decimal.closest(".byte-converter")?.after(error);
    }
    error.setAttribute("aria-live", "polite");
    decimal.setAttribute("aria-describedby", error.id);
    hex.setAttribute("aria-describedby", error.id);

    const showError = (message, invalidInput) => {
      error.textContent = message;
      decimal.setAttribute("aria-invalid", String(invalidInput === decimal));
      hex.setAttribute("aria-invalid", String(invalidInput === hex));
    };

    decimal.addEventListener("input", () => {
      const value = decimal.value;
      if (value === "") {
        hex.value = "";
        showError("", null);
      } else if (!/^\d+$/.test(value) || Number(value) > 255) {
        hex.value = "";
        showError("Enter a decimal value from 0 through 255.", decimal);
      } else {
        hex.value = Number(value).toString(16).toUpperCase().padStart(2, "0");
        showError("", null);
      }
    });

    hex.addEventListener("input", () => {
      const value = hex.value.toUpperCase();
      hex.value = value;
      if (value === "") {
        decimal.value = "";
        showError("", null);
      } else if (!/^[0-9A-F]{1,2}$/.test(value)) {
        decimal.value = "";
        showError("Enter hexadecimal using 0–9 and A–F.", hex);
      } else {
        decimal.value = String(parseInt(value, 16));
        showError("", null);
      }
    });
  }

  function installKitSignup() {
    if (location.pathname !== "/") return;
    const section = document.getElementById("protocol-interest");
    if (!section || section.querySelector(".kit-signup-host")) return;
    const host = document.createElement("div");
    host.className = "kit-signup-host";
    const script = document.createElement("script");
    script.async = true;
    script.dataset.uid = "036ef99a3d";
    script.src = KIT_SRC;
    host.appendChild(script);
    section.appendChild(host);
  }

  function explainNonce() {
    if (location.pathname !== "/symmetric-encryption") return;
    const plaintext = document.getElementById("encrypt-plaintext");
    const box = plaintext?.closest(".mini-workbench");
    if (!box || box.querySelector(".nonce-explanation")) return;

    const nonceOutput = box.querySelector(".output");
    const label = nonceOutput?.querySelector("span");
    if (label) label.textContent = "NONCE · FRESH RANDOM 12 BYTES · NOT SECRET";

    const note = document.createElement("p");
    note.className = "section-copy nonce-explanation";
    note.textContent = "The nonce is a separate one-time value generated for this encryption. It is not derived from the plaintext and does not need to be secret. Store it with the ciphertext; AES-GCM needs the same nonce to decrypt, and a fresh nonce must be used for each encryption with a given key.";
    nonceOutput?.before(note);
  }

  function enhance() {
    installByteConverter();
    installKitSignup();
    explainNonce();
    if (location.pathname === "/hashes") loadScript("/assets/hash.js");
  }

  enhance();
  new MutationObserver(enhance).observe(document.body, { childList: true, subtree: true });
})();
