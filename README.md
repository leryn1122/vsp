# README

## TL;DR

A toy language compiler during learning custom compiler and modern C++ lang.

## Table of Contents

- [Background](#background)
- [RoadMap](#RoadMap)
- [Install](#install)
- [Usage](#usage)
- [Related Efforts](#related-efforts)
- [Maintainers](#maintainers)
- [License](#license)

## Etymology

`vsp` originated from Latin [vespera](https://en.wiktionary.org/wiki/vespera), meaning **of or related to evening**. It was formally called `vesperace`, but it was too long.

## Background

A toy language compiler during learning custom compiler and modern C++ lang.

## RoadMap

Long term:

| Command   | Intro                                 |
| --------- | ------------------------------------- |
| `vspc`    | Vesperace Language Compiler           |
| `vspr`    | Runtime Launcher                      |
| `vsps`    | Process Status Tool                   |
| `vspstk`  | Stack Trace Tool                      |
| `vsprepl` | REPL (Read-Eval-Print Loop)           |
| `vspx`    | Tool of Compression and Decompression |

## Install

This project uses
  - C++ as develop language
  - [g++](https://github.com/gcc-mirror/gcc) or [clang](https://github.com/llvm-mirror/clang): The choice of compiler is not compulsory, both `g++` and `clang` is available.
  - [CMake](https://gitlab.kitware.com/cmake/cmake): Advanced build tools.
  - [vcpkg](https://github.com/microsoft/vcpkg) (Optional): Package manager.
  - [upx](https://github.com/upx/upx) (Optional): Packer for executables.

Go check them out if you don't have them locally installed.

```bash
# If using apt
sudo yum install -y cmake

# If using apt
sudo apt install -y cmake
```

```bash
git clone https://github.com/leryn1122/vsp-cpp.git

# or simply using `build.sh`
cmake . && make
```

## Usage

```plaintext
vspc <source> [ options [ params ... ] ... ]
```

```bash
vspc test.vsp

vspc test.vsp --verbose

vspc --version

vspc --help
```

###

Lexer: String => Path => File => TokenStream
Parser: TokenStream => AstNode
Syntax: AstNode =>

## Related Efforts

Those repos are referenced on:

- [Xie-Jason/GloomScript](https://github.com/Xie-Jason/GloomScript)
- [douchuan/jvm](https://github.com/douchuan/jvm)
- [rhaiscript/rhai](https://github1s.com/rhaiscript/rhai)

The project was developped in Rust in the early stages:

- [leryn1122/vsp-rust](https://github.com/leryn1122/vsp-rust)

## Maintainers

[@Leryn](https://github.com/leryn1122).

## License

[MIT](LICENSE) © Leryn
