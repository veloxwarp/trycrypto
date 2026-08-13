(() => {
  const subtle = crypto.subtle;
  const hex = b => Array.from(new Uint8Array(b), x => x.toString(16).padStart(2,"0")).join("");
  const bytes = h => new Uint8Array(h.match(/../g)?.map(x => parseInt(x,16)) || []);
  const rawHex = async k => hex(await subtle.exportKey("raw",k));
  const secretHex = async k => { const j=await subtle.exportKey("jwk",k); const s=j.d.replace(/-/g,"+").replace(/_/g,"/"); const p=s+"=".repeat((4-s.length%4)%4); return Array.from(atob(p),c=>c.charCodeAt(0).toString(16).padStart(2,"0")).join(""); };
  const keypair = async name => { const usages=name==="X25519"?["deriveKey"]:["sign","verify"]; const k=await subtle.generateKey({name},true,usages); return {...k,publicHex:await rawHex(k.publicKey),secretHex:await secretHex(k.privateKey)}; };
  const sharedKey = (priv,pub) => subtle.deriveKey({name:"X25519",public:pub},priv,{name:"AES-GCM",length:256},false,["encrypt","decrypt"]);
  const seal = async (pub,text) => { const eph=await keypair("X25519"),key=await sharedKey(eph.privateKey,pub),nonce=crypto.getRandomValues(new Uint8Array(12)),ct=await subtle.encrypt({name:"AES-GCM",iv:nonce},key,new TextEncoder().encode(text)); return {ephemeral:eph.publicKey,ephemeralHex:eph.publicHex,nonce:hex(nonce),ciphertext:hex(ct)}; };
  const open = async (priv,eph,nonce,ct) => { const key=await sharedKey(priv,eph),plain=await subtle.decrypt({name:"AES-GCM",iv:bytes(nonce)},key,bytes(ct)); return new TextDecoder().decode(plain); };
  const sign = async (priv,text) => hex(await subtle.sign("Ed25519",priv,new TextEncoder().encode(text)));
  const verify = async (pub,text,sig) => subtle.verify("Ed25519",pub,bytes(sig),new TextEncoder().encode(text));
  window.TryCryptoCurve={keypair,seal,open,sign,verify};
})();
