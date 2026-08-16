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

    decimal.addEventListener("input", () => {
      const value = decimal.value;
      if (value === "") {
        hex.value = "";
        error.textContent = "";
      } else if (!/^\d+$/.test(value) || Number(value) > 255) {
        hex.value = "";
        error.textContent = "Enter a decimal value from 0 through 255.";
      } else {
        hex.value = Number(value).toString(16).toUpperCase().padStart(2, "0");
        error.textContent = "";
      }
    });

    hex.addEventListener("input", () => {
      const value = hex.value.toUpperCase();
      hex.value = value;
      if (value === "") {
        decimal.value = "";
        error.textContent = "";
      } else if (!/^[0-9A-F]{1,2}$/.test(value)) {
        decimal.value = "";
        error.textContent = "Enter hexadecimal using 0–9 and A–F.";
      } else {
        decimal.value = String(parseInt(value, 16));
        error.textContent = "";
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
