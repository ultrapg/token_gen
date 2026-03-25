# TokenGen
*TokenGen is a high-performance Rust command-line tool designed to generate cryptographically secure random tokens of any size. It features a real-time progress bar and memory-efficient streaming, making it ideal for generating exceptionally large tokens without exhausting system RAM.*

### Features

- Secure Randomness: Uses the rand crate's thread_rng for cryptographically secure byte generation.

- Memory Efficient: Processes data in 8KB chunks, allowing you to generate gigabytes of data with a minimal memory footprint.

- Multiple Encodings: Supports both Hex and Base64 output formats.

- Progress Tracking: Integrated with indicatif to show a beautiful progress bar, elapsed time, and estimated time of arrival (ETA).

- Flexible Output: Stream tokens directly to your terminal or save them to a specific file.

---

### Installation


**Clone the repository:**
```
git clone https://github.com/ultrapg/token_gen.git
cd token_gen
```

**Build for release:**
```
cargo build --release
```

---

### Usage

You can run the tool using cargo run -- or by executing the compiled binary directly.

Basic Command Structure

```token_gen [OPTIONS]```


**Options**

-b, --bits <BITS>: Total number of bits to generate (default: 1024).

-a, --algo <ALGO>: Encoding algorithm to use: hex (default) or base64.

-o, --output <PATH>: Optional file path to save the token. If omitted, the token is printed to stdout.

**Examples**

1. Generate a standard 256-bit Hex token

```
token_gen --bits 256
```


2. Generate a massive 1GB Base64 token and save it to a file

```
token_gen --bits 8589934592 --algo base64 --output huge_token.txt
```


3. Generate a 4096-bit token for a security key

```
token_gen -b 4096 -a hex > security.key
```

---

### Dependencies

- clap: For robust command-line argument parsing.

- rand: For secure random number generation.

indicatif: For the interactive progress bar.

hex & base64: For efficient data encoding.

