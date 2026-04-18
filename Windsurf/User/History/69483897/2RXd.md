<h1 align="center">lwloader</h1>
<p align="center" >
  <img alt="Assembly" src="https://img.shields.io/badge/Assembly-6E4C13?logo=nasm&logoColor=white">
  <img alt="C" src="https://img.shields.io/badge/C-00599C?logo=c&logoColor=white">
  <img alt="Status" src="https://img.shields.io/badge/status-beta-yellow">
  <img alt="License" src="https://img.shields.io/badge/license-ISC-blue">
</p>

<br>

**lwloader** is a lightweight bootloader project written in assembly and C, designed to boot a simple kernel and demonstrate low-level system programming concepts.

## Features

- minimal bootloader implementation in x86 assembly
- simple kernel written in C
- custom linker script for memory layout
- bootable disk image creation
- educational low-level programming

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
1. assemble `bootloader.asm` using nasm
2. compile `kernel.c` using gcc
3. link the components using the custom linker script
4. create a bootable disk image

## Project Structure

- `bootloader.asm` - main bootloader code in x86 assembly
- `kernel.c` - simple kernel implementation in C
- `linker.ld` - custom linker script for memory layout
- `run.sh` - build and execution script

## Requirements

- nasm
- gcc
- qemu

## Examples

Build and run the bootloader:

```bash
./run.sh
```

The script will create a bootable image and can be run with qemu for testing.

## Output

The build process creates a bootable disk image that can be used with virtualization software or written to physical media for testing on real hardware.

## Contributing

1. fork the repository
2. create a feature branch
3. submit a pull request

## License

ISC — see [LICENSE](LICENSE) for details.

## EOF
