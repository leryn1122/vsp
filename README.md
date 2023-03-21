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

`vsp` originated from Latin [vespera](https://en.wiktionary.org/wiki/vespera), meaning **of or related to evening**.
 It was formally called `vesperace`, but it was too long.

## Background

A toy language compiler during learning custom compiler and Rust lang.

## RoadMap

Long term:

| Command               | Alias (TODO*) | Intro                                 |
| --------------------- | ------------- | ------------------------------------- |
| `vsp compile`         | `vspc`        | Language compiler                     |
| `vsp debug`           | `vspd`        | Runtime debugger                      |
| `vsp help`            |               | Print help info                       |
| `vsp ps`              | `vsps`        | Process status tool                   |
| `vsp repl`            | `vsprepl`     | REPL (Read-Eval-Print Loop) or shell  |
| `vsp run`             | `vspr`        | Runtime Launcher                      |
| `vsp stack`           | `vspstk`      | Stack trace tool                      |
| `vsp tar`             | `vspx`        | Tool of Compression and Decompression |
| `vsp version`         |               | Print version info                    |

## Install

This project uses [rust](https://www.rust-lang.org/) and [cargo](https://npmjs.com). Go check them out if you don't have them locally installed.

```bash
git clone https://github.com/leryn1122/vsp.git
cd vsp

# If linux
./start.sh

# If Windows, use powershell
.\start.ps1

# If you install python
python3 start.py
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
