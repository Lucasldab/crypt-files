# Crypt-Files

Crypt-Files is a Rust command-line tool for hiding files inside images using **LSB (Least Significant Bit) steganography**. You can embed any type of file — images, audio, documents, or executables — into an image without noticeably altering its appearance.

---

## Features

- Load PNG or JPG images.
- Modify the LSB of each RGB channel to store file data.
- Supports hiding any type of file.

---

## Usage

```bash
cargo run -- <image_path> [file_to_hide]
