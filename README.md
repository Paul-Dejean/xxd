# cxxd

cxxd is a Rust reimplementation of the Unix command-line tool `xxd`. It can generate a hex dump of a file or revert a hex dump back to binary.

## Features

- **Hex Dump:** Print a formatted hex dump of any file.
- **Revert Mode:** Convert a hex dump back into binary data.
- **Custom Options:**
  - **Endianness:** Use `-e` for little-endian output.
  - **Group Size:** Adjust byte grouping with `-g` (defaults to 2, or 4 with little-endian).
  - **Columns:** Set the number of columns with `-c` (default: 16).
  - **Length & Seek:** Specify the number of bytes to dump (`-l`) and the offset (`-s`).

## Installation

Ensure you have the Rust toolchain installed. Then, clone the repository and build:

```bash
git clone <repository_url>
cd cxxd
cargo build --release
```

## Usage

**Generate Hex Dump:**

```bash
./target/release/cxxd file.txt > file.hex
```

**Revert Hex Dump to Binary:**

```bash
./target/release/cxxd file.hex > revert.txt
```

**Display Help:**

```bash
./target/release/cxxd --help
```

## Author

Paul Dejean (pauldejeandev@gmail.com)
