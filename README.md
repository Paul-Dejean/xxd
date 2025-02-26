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

To build and run `cwc`, you need to have [Rust](https://www.rust-lang.org/) installed on your system.

1. Clone the repository:

   ```bash
   git clone git@github.com:Paul-Dejean/xxd.git
   cd xxd
   ```

2. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

3. (Optional) Install the binary globally:

   ```bash
   cargo install --path .
   ```

## Usage

Once built, you can run cwc from the command line. Below are some example usages:

**Generate Hex Dump:**

```bash
./target/release/cxxd file.txt > file.hex
```

Or if installed globally:

```bash
cxxd file.txt > file.hex
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
