<div align="center">
    <img src="../assets/logo.png?raw=true" style="width: 25%; height: auto;" alt="HADERS Logo">
    <h1>HADERS</h1>
    CLI app that generates zip bombs for you
    <br>
    HADERS [heɪ.di.ɑːr.es] - Hades + Rust
</div>

## Usage

```
./haders <mode> <size> <out_zip_file>

./haders nested 10tb funny.zip
```

### Modes

- `nested`
- `flat`

### Size Units

- `GB` - Giga Byte
- `TB` - Tera Byte (1024 GB)
- `PB` - Peta Byte (1024 TB)
- `EB` - Exa Byte (1024 PB)

## Requirements, when compiling yourself

- Computer
- Internet connection
- `Rust`
- `cargo` - Rust's package manager

## What is it zip bomb?

A zip bomb is a malicious archive file designed to exploit compression algorithms to create a disproportionately large file when decompressed.

## How nested zip bombs work?

It works by nesting multiple layers of compressed data within each other, taking advantage of the way compression algorithms work. When the file is extracted, the decompression process repeatedly expands the inner layers, consuming an excessive amount of system resources, often leading to system crashes or slowdowns. Zip bombs are typically used for malicious purposes, such as disrupting systems, overwhelming storage, or evading security measures.

## How nested zip bombs are being created?

1. Creates a dummy file of size (say 1 GB)
2. Compresses dummy file to a zip (deflated)
3. Makes `n` (say 10) copies of this zip file and adds these files to another zip archive
4. Repeats step 3 `x` number of times
5. Voila, zip bomb has been created

---

Author: [Tymoteusz Hołubowicz](https://github.com/t-holubowicz)
