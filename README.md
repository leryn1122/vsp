# README

🙈 The project is rebuilding !!

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

## Etymology

`vespera` originated from Latin [vespera](https://en.wiktionary.org/wiki/vespera), meaning **of or related to evening**.
It was mentioned in a long-word gossip, and it morphs into `vsp` after loss of vowels.

## Background

A toy language compiler during learning custom compiler and Rust lang.

## RoadMap

Long term:

| Command               | Alias (TODO*) | Intro                                 |
| --------------------- | ------------- | ------------------------------------- |
| `vsp compile`         | `vspc`        | Language compiler                     |
| `vsp debug`           | `vspd`        | Runtime debugger                      |
| `vsp help`            |               | Print help info                       |
| `vsp repl`            | `vspsh`     | REPL (Read-Eval-Print Loop) or shell  |
| `vsp tar`             | `vspx`        | Tool of Compression and Decompression |
| `vsp version`         |               | Print version info                    |

## Install

This project uses [rust](https://www.rust-lang.org/) and [cargo](https://npmjs.com). Go check them out if you don't have them locally installed.

### Prerequisites
- Rust runtime environment, version >=1.60.
- LLVM 14
- Python 3 or 2.7
- make
- static library
  - OpenSSL (libssl-dev or openssl-devel on most Unix distros)

```bash
git clone https://github.com/leryn1122/vsp.git
cd vsp

# If not Windows
make bootstrap

# If Windows, use powershell
.\make.ps1
```

## Usage

```plaintext
vsp compile <source> [ options [ params ... ] ... ]
```

```bash
vsp compile test.vsp

vsp compile test.vsp --verbose

vsp --version

vsp --help
```

## Related Efforts

Those repos are referenced on:

- [Xie-Jason/GloomScript](https://github.com/Xie-Jason/GloomScript)
- [douchuan/jvm](https://github.com/douchuan/jvm)
- [rhaiscript/rhai](https://github1s.com/rhaiscript/rhai)

## Maintainers

[@Leryn](https://github.com/leryn1122).

## License

[MIT](LICENSE) © Leryn
