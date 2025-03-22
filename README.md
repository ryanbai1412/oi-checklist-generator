# OI Checklist Generator

A tool to automatically generate a checklist of OI problems.

Examples of generated files can be found
for [a blank checklist](https://ryanbai1412.github.io/oi-checklist-generator/examples/blank), [rainboy](https://ryanbai1412.github.io/oi-checklist-generator/examples/rainboy), and [me](https://ryanbai1412.github.io/oi-checklist-generator/examples/skittles1412).

## Installation

### Installing using prebuilt binaries

Navigate to the [release page](https://github.com/ryanbai1412/oi-checklist-generator/releases/latest), and either use one of the shell scripts or download the binary directly and add it to your system PATH.

### Installing from source

[Install rust](https://rustup.rs), then run `cargo install --git https://github.com/ryanbai1412/oi-checklist-generator.git`

## Usage

```
Usage: oicg [OPTIONS]

Options:
    --dmoj <DMOJ_USERNAME>  DMOJ username
    --ojuz <OJUZ_USERNAME>  oj.uz username
-o, --output <OUTPUT>       Output location (defaults to oi-checklist.html in the Documents folder)
    --open                  Open the output file in the default web browser
-h, --help                  Print help
```

Example: `oicg --dmoj my_dmoj_username --ojuz my_ojuz_username -o /path/to/output/file --open`
