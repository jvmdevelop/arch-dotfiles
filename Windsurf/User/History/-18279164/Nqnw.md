<h1 align="center">character-voxel-gen</h1>
<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-blue">
  <img alt="Status" src="https://img.shields.io/badge/status-stable-brightgreen">
</p>

<br>

**character-voxel-gen** is a high-performance voxel DSL parser and OBJ exporter written in Rust, featuring token-based parsing, polygon detection, and 3D model generation.

## Features

- lexer for parsing VDL (Voxel Description Language) files
- token-based parsing with support for points, lines, and comments
- obj file export with polygon detection and material generation
- command-line interface for file conversion
- memory-safe implementation with Rust's ownership system
- zero-cost abstractions for optimal performance

## Installation

### Prerequisites:

- rust 1.70+ 
- cargo

### From source:

```bash
git clone https://github.com/jvmdevelop/character-voxel-gen.git
cd character-voxel-gen
cargo build --release
```

### Run from source:

```bash
cargo run -- <input.vdsl> <output.obj>
```

## Usage

### Command Line Interface

```bash
cargo run -- input.vdsl output.obj
```

### VDL Format

The VDL format supports:
- Points: `<index>: <x> <y> <z>`
- Lines: `l: <start_index> <end_index>`
- Comments: `c: <comment>` or `# <comment>`

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

## Project Structure

- `src/lexer/` - Tokenizes VDL input
  - `mod.rs` - Lexer trait and module exports
  - `static_lexer.rs` - Static lexer implementation
- `src/model/` - Defines token types and structures
  - `mod.rs` - Model module exports
  - `token.rs` - Generic token structure
  - `token_type.rs` - Token type enum and factory
  - `point_token.rs` - Point token implementation
  - `line_token.rs` - Line token implementation
  - `comment_token.rs` - Comment token implementation
- `src/parser/` - Coordinates lexing and export
  - `mod.rs` - Main parser implementation
- `src/exporter/` - Converts tokens to OBJ format
  - `mod.rs` - Exporter module exports
  - `obj_exporter.rs` - OBJ file exporter with polygon detection
- `src/main.rs` - Command-line interface

## Examples

Convert a VDL file to OBJ:

```bash
cargo run -- examples/cube.vdsl output.obj
```

Create a simple cube:

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

## Dependencies

- `thiserror` - Error handling
- `std::collections` - HashMap and HashSet for adjacency lists
- `std::fs` - File system operations
- `std::io` - I/O operations

## Algorithm Details

### Polygon Detection

The exporter uses a cycle detection algorithm to identify closed polygons:
1. Builds adjacency list from line tokens
2. Performs DFS to find all cycles
3. Filters to simple 4-vertex polygons
4. Validates polygon edges exist in original data
5. Generates OBJ faces for valid polygons

### Token Processing

1. **Lexing**: Splits input into lines, removes comments
2. **Tokenization**: Identifies token types (point, line, comment)
3. **Parsing**: Creates strongly-typed token objects
4. **Export**: Converts tokens to OBJ format with materials

## Performance

- Memory-safe with no garbage collection
- Linear time complexity for lexing O(n)
- Efficient polygon detection with adjacency lists
- Minimal memory allocations with Rust's ownership system

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT — see [LICENSE](LICENSE) for details.

## EOF
