<h1 align="center">lwloader</h1>
<p align="center" >
  <img alt="Assembly" src="https://img.shields.io/badge/Assembly-6E4C13?logo=nasm&logoColor=white">
  <img alt="C" src="https://img.shields.io/badge/C-00599C?logo=c&logoColor=white">
  <img alt="Status" src="https://img.shields.io/badge/status-beta-yellow">
  <img alt="License" src="https://img.shields.io/badge/license-ISC-blue">
</p>

<br>

**lwloader** is a lightweight bootloader project written in Assembly and C, designed to boot a simple kernel and demonstrate low-level system programming concepts.

## Features

- Minimal bootloader implementation in x86 assembly
- Simple kernel written in C
- Custom linker script for memory layout
- Bootable disk image creation
- Educational low-level programming

## Installation

### From source:

```bash
git clone git@github.com:jvmdevelop/lwloader.git
cd lwloader
chmod +x run.sh
./run.sh
```

## Usage

The project includes a build script that assembles the bootloader, compiles the kernel, and creates a bootable disk image.

```bash
./run.sh
```

This will:
1. Assemble `bootloader.asm` using NASM
2. Compile `kernel.c` using GCC
3. Link the components using the custom linker script
4. Create a bootable disk image

## Project Structure

- `bootloader.asm` - Main bootloader code in x86 assembly
- `kernel.c` - Simple kernel implementation in C
- `linker.ld` - Custom linker script for memory layout
- `run.sh` - Build and execution script

## Requirements

- NASM (Netwide Assembler)
- GCC (with appropriate cross-compilation tools if needed)
- QEMU or similar x86 emulator for testing

## Examples

Build and run the bootloader:

```bash
./run.sh
```

The script will create a bootable image and can be run with QEMU for testing.

## Output

The build process creates a bootable disk image that can be used with virtualization software or written to physical media for testing on real hardware.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## License

ISC — see [LICENSE](LICENSE) for details.

## EOF
