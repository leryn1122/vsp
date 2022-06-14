# README

## TL;DR

A toy language compiler during learning custom compiler and Rust lang.

## Table of Contents

- [Background](#background)
- [RoadMap](#RoadMap)
- [Install](#install)
- [Usage](#usage)
- [Related Efforts](#related-efforts)
- [Maintainers](#maintainers)
- [License](#license)

## Background

A toy language compiler during learning custom compiler and modern C++ lang.

## RoadMap

Long term:

| Command   | Intro                                 |
| --------- | ------------------------------------- |
| `vspc`    | Compiler                              |
| `vspr`    | Runtime                               |
| `vsps`    | Process tool                          |
| `vspstk`  | Stack trace tool                      |
| `vsprepl` | REPL (Read-Eval-Print Loop)           |
| `vspx`    | Tool of Compression and Decompression |

## Install

This project uses C++, [CMake](https://gitlab.kitware.com/cmake/cmake),
[vcpkg](https://github.com/microsoft/vcpkg). Go check them out if you don't have them locally installed.

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

## Maintainers

[@Leryn](https://github.com/leryn1122).

## License

[MIT](LICENSE) © Leryn
