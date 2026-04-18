<h1 align="center">character-voxel-gen</h1>
<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-blue">
  <img alt="Status" src="https://img.shields.io/badge/status-stable-brightgreen">
</p>

<br>

**character-voxel-gen** is a high-performance voxel dsl parser and obj exporter written in Rust, featuring token-based parsing, polygon detection, and 3D model generation.

## features

- lexer for parsing vdl files
- token-based parsing with support for points, lines, and comments
- obj file export with polygon detection and material generation
- command-line interface for file conversion
- memory-safe implementation with Rust's ownership system
- zero-cost abstractions for optimal performance

## installation

### prerequisites:

- rust 1.70+ 
- cargo

### from source:

```bash
git clone https://github.com/jvmdevelop/character-voxel-gen.git
cd character-voxel-gen
cargo build --release
```

### run from source:

```bash
cargo run -- <input.vdsl> <output.obj>
```

## usage

### command line interface

```bash
cargo run -- input.vdsl output.obj
```

### vdl format

the vdl format supports:
- points: `<index>: <x> <y> <z>`
- lines: `l: <start_index> <end_index>`
- comments: `c: <comment>` or `# <comment>`

Example:
```
0: 0 0 0
1: 10 0 0
2: 10 10 0
3: 0 10 0
l: 0 1
l: 1 2
l: 2 3
l: 3 0
```

## project structure

- `src/lexer/` - tokenizes vdl input
  - `mod.rs` - lexer trait and module exports
  - `static_lexer.rs` - static lexer implementation
- `src/model/` - defines token types and structures
  - `mod.rs` - model module exports
  - `token.rs` - generic token structure
  - `token_type.rs` - token type enum and factory
  - `point_token.rs` - point token implementation
  - `line_token.rs` - line token implementation
  - `comment_token.rs` - comment token implementation
- `src/parser/` - coordinates lexing and export
  - `mod.rs` - main parser implementation
- `src/exporter/` - converts tokens to obj format
  - `mod.rs` - exporter module exports
  - `obj_exporter.rs` - obj file exporter with polygon detection
- `src/main.rs` - command-line interface

## examples

convert a vdl file to obj:

```bash
cargo run -- examples/cube.vdsl output.obj
```

simple cube example:

```bash
echo "0: 0 0 0
1: 10 0 0
2: 10 10 0
3: 0 10 0
4: 0 0 10
5: 10 0 10
6: 10 10 10
7: 0 10 10
l: 0 1
l: 1 2
l: 2 3
l: 3 0
l: 4 5
l: 5 6
l: 6 7
l: 7 4
l: 0 4
l: 1 5
l: 2 6
l: 3 7" > cube.vdsl

cargo run -- cube.vdsl cube.obj
```

### token processing

1. **lexing**: splits input into lines, removes comments
2. **tokenization**: identifies token types (point, line, comment)
3. **parsing**: creates strongly-typed token objects
4. **export**: converts tokens to obj format with materials

## contributing

1. fork the repository
2. create a feature branch (`git checkout -b feature/amazing-feature`)
3. commit your changes (`git commit -m 'Add amazing feature'`)
4. push to the branch (`git push origin feature/amazing-feature`)
5. open a pull request

## license

MIT — see [LICENSE](LICENSE) for details.

## EOF
