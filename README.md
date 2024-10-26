# Loudness Addict

Measure integrated loudness in LUFS and RMS standards.

## Motivation

This project aims to deepen my Rust language skills combined with my interest in audio engineering.

## Features

## Example Output

```
> cargo run -- testsamples/beat.wav testsamples/sine.wav
    Compiling loudness-addict v0.1.0 (F:\Dev\loudness-addict)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.65s
    Running `target\debug\loudness-addict.exe testsamples/beat.wav testsamples/sine.wav`
Loudness Addict 0.1.0

Parameters
["target\\debug\\loudness-addict.exe", "testsamples/beat.wav", "testsamples/sine.wav"]

testsamples/beat.wav Int. RMS: -9.18
testsamples/sine.wav Int. RMS: -15.05
```

## Technical Requirements

Developed using Rust `v1.82.0`.

## How to Use

### Run with Rust and Cargo

First, you need to have [Rust](https://www.rust-lang.org/tools/install) installed on your computer.

Open a terminal:

- Windows: run `Windows Terminal`, `cmd`, or `PowerShell`.
- Mac: run `Terminal`.
- Linux: run `Terminal`.

In the project root folder, run:

```
cargo run -- testsamples/beat.wav testsamples/sine.wav
```

## Research

### Rust Crates

* [bs1770 - Loudness analysis conforming to ITU-R BS.1770-4](https://crates.io/crates/bs1770)
* [hound - A wav encoding and decoding library](https://crates.io/crates/hound)

### Standards

* [Recommendation ITU-R BS.1770-5 (11/2023) BS Series: Broadcasting service (sound) Algorithms to measure audio programme loudness and true-peak audio level](https://www.itu.int/dms_pubrec/itu-r/rec/bs/R-REC-BS.1770-5-202311-I!!PDF-E.pdf)
* [Loudness Metering: ‘EBU MODE’ Metering To Supplement EBU R 128 LOUDNESS NORMALIZATION](https://tech.ebu.ch/files/live/sites/tech/files/shared/tech/tech3341.pdf)

## License

* The binary program uses [MIT license](LICENSE-MIT).
* The crates use [Apache 2.0 licenses](LICENSE-APACHE).

## History

I started the project on 19th October, 2024.
