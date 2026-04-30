# SECURITY-NOTES — crypt-files crypto audit

Audit date: 2026-04-30  
Scope: `src/main.rs`, `Cargo.toml`, README  
Branch: `prior-branch`

---

## CRITICAL

### C-1 — No encryption despite the name
**File:** `src/main.rs` (entire encode loop, lines 30–35)

The project is named "crypt-files" and the README implies concealment equals protection, but there is **zero cryptographic protection**. Data is embedded as raw plaintext bits using textbook LSB steganography. Any standard steganalysis tool (`steghide`, `zsteg`, `stegsolve`, manual LSB extraction) recovers the payload in seconds with no key required. Steganography is not encryption; it provides no confidentiality.

**Required fix:** Encrypt the payload (e.g. AES-256-GCM with a password-derived key via Argon2id) before embedding. Never conflate hiding with protecting.

---

### C-2 — No extraction functionality
**File:** `src/main.rs` — embed-only, no `extract` subcommand

The tool can embed data but **cannot retrieve it**. There is no decode/extract path. The embedded payload is effectively write-only and permanently inaccessible through the tool itself. A steganography tool without extraction is incomplete and unusable for its intended purpose.

**Required fix:** Implement an `extract` subcommand that reverses the embed operation.

---

### C-3 — No length header; payload boundary unknown
**File:** `src/main.rs:30–35`

No length field, magic bytes, or termination marker is stored alongside the payload. Even if extraction were implemented, the extractor cannot know where the hidden data ends — it would read into the unmodified carrier noise beyond the payload boundary, producing corrupted output.

**Required fix:** Prepend a fixed-size big-endian length field (e.g. u64, 64 bits = 8 LSBs) before the payload bits.

---

## HIGH

### H-1 — JPEG output silently destroys the payload
**File:** `src/main.rs:58–60` (`save_image`)

The save function preserves the original file extension verbatim (`format!("{}-formatted.{}", stem_str, ext_str)`). When the input image is a JPEG, the output is also saved as JPEG. JPEG is a lossy codec — re-encoding will corrupt the steganographic LSBs and make the payload unrecoverable. The README advertises JPEG support, compounding the problem.

**Required fix:** Always save as a lossless format (PNG). Reject or convert lossy inputs. Remove JPEG from the supported-input list unless handled specially.

---

### H-2 — No integrity protection
**File:** `src/main.rs` — entire embed path

No MAC, HMAC, or checksum covers the payload. A bit-flip (accidental or adversarial) in the carrier image silently corrupts the hidden data with no detection. When combined with encryption (C-1 fix), use an AEAD cipher (e.g. AES-256-GCM or ChaCha20-Poly1305) so integrity failures surface as authentication errors.

---

### H-3 — No access control; zero-knowledge extraction
**File:** `src/main.rs` — entire design

No password, key, or shared secret is required to embed or extract. Anyone who suspects LSB steganography in an image can recover the payload trivially. This removes any practical security boundary.

**Required fix:** Require a passphrase at embed time; derive a key via Argon2id; store only the ciphertext in the LSBs.

---

## MEDIUM

### M-1 — Debug print left in production code
**File:** `src/main.rs:37`

```rust
println!("{:08b}", img.as_raw()[0]);
```

This prints the first pixel's red channel in binary to stdout on every invocation. It is clearly debug code. It leaks internal state and is confusing to users.

**Required fix:** Remove the line.

---

### M-2 — No file metadata embedded (name, type, size)
**File:** `src/main.rs:30–35`

The raw file bytes are embedded with no filename, MIME type, or size metadata. Even if extraction worked, the receiver cannot know the original filename or extension, making correct reconstruction impossible without out-of-band communication.

**Required fix:** Embed a header (filename length, filename bytes, payload length, payload bytes) before encrypting.

---

### M-3 — Capacity check risks integer overflow on 32-bit targets
**File:** `src/main.rs:24`

```rust
if file_bytes.len() * 8 > data.len() {
```

`file_bytes.len() * 8` uses `usize` arithmetic. On a 32-bit target, any file ≥ 512 MiB causes silent wrapping overflow, bypassing the check and writing out-of-bounds. Rust debug builds panic on overflow; release builds wrap silently.

**Required fix:** Use `file_bytes.len().checked_mul(8).map_or(true, |bits| bits > data.len())` or cast to `u64` before multiplying.

---

## LOW

### L-1 — Bit ordering undocumented
**File:** `src/main.rs:72–83` (`byte_to_bits`)

`byte_to_bits` emits bits MSB-first (bit 7, bit 6, … bit 0). This ordering must be mirrored exactly in any extraction implementation. It is not documented anywhere, creating a silent interoperability trap.

**Required fix:** Add a spec comment or document the wire format (e.g. "MSB-first, big-endian byte stream").

---

### L-2 — Output file silently overwritten
**File:** `src/main.rs:67` (`image.save(&new_path)`)

If `<stem>-formatted.<ext>` already exists, it is silently overwritten. No warning is issued and no backup is made.

**Required fix:** Check for file existence before saving; error out or prompt the user.

---

## Summary table

| ID  | Severity | Finding |
|-----|----------|---------|
| C-1 | CRITICAL | No encryption — plaintext payload, trivially extractable |
| C-2 | CRITICAL | No extraction path — payload permanently inaccessible via the tool |
| C-3 | CRITICAL | No length header — payload boundary undetectable |
| H-1 | HIGH     | JPEG output destroys LSB payload via lossy recompression |
| H-2 | HIGH     | No integrity/authentication — silent payload corruption |
| H-3 | HIGH     | No key/password — zero access control |
| M-1 | MEDIUM   | Debug `println!` left in production (line 37) |
| M-2 | MEDIUM   | No filename/type metadata embedded |
| M-3 | MEDIUM   | Integer overflow in capacity check on 32-bit targets |
| L-1 | LOW      | Bit ordering (MSB-first) undocumented |
| L-2 | LOW      | Output file silently overwritten if it exists |
