# Fastl

A fast(subjective, no benchmarks, ~~probably~~ false) STL file parser that determines
two things:

- The number of triangles in the file.
- The triangles surface area.

## Usage

1. Build the binary. `cargo build --release`

   - Optional: You can install the binary to make it available with `cargo install --path .`

2. Run the binary and supply the path to a file. Omit `./target/release/` if you've opted to isntall the binary.

   - `./target/release/fastl <StlFile>` (e.g. `./target/release/fastl Moon.stl`).

## Dependencies

None
