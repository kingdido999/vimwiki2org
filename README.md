# Vimwiki2Org

This is an experimental cli program to convert vimwiki files to org mode files. Currently it does not convert internal links correctly, so use at your own risk.

## Prerequisite

- [Install pandoc](https://pandoc.org/installing.html)
- [Install Rust nightly](https://rust-lang.github.io/rustup/installation/index.html#installing-nightly)

## Usage

The program takes a source folder with `.wiki` files and converts them to `.org` files in the specified destination folder.

``` sh
cargo run -- <src> <dest>
```

For example:

``` sh
cargo run -- ~/wiki ~/org
```
