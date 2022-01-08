# highlight-assertions

Reads the unit test format for highlighting of tree-sitter
https://tree-sitter.github.io/tree-sitter/syntax-highlighting#unit-testing to make it available for
unit tests for https://github.com/nvim-treesitter/nvim-treesitter

The function parsing the file format is vendored from
https://github.com/tree-sitter/tree-sitter/blob/master/cli/src/query_testing.rs#L27-L124

## Usage

```
highlight-assertions 0.1.0

Stephan Seitz <stephan.seitz@fau.de>

Reads the unit test format for highlighting of tree-sitter https://tree-sitter.github.io/tree-
sitter/syntax-highlighting#unit-testing to make it available for unit test for
https://github.com/nvim-treesitter/nvim-treesitter. Output will be printed to stdout

USAGE:
    highlight-assertions [OPTIONS] --parser-file <PARSER_FILE> --source-file <SOURCE_FILE>

OPTIONS:
    -c, --comment-node <COMMENT_NODE>
            Name of comment node in the language at hand [default: comment]

    -h, --help
            Print help information

    -p, --parser-file <PARSER_FILE>
            Parser library to load (e.g. cpp.so from nvim-treesitter/parser)

    -s, --source-file <SOURCE_FILE>
            Source file with highlight assertions following https://tree-sitter.github.io/tree-
            sitter/syntax-highlighting#unit-testing

    -V, --version
            Print version information
```
Output will be printed as JSON to stdout.

## Building

This crate requires nightly rust due to https://github.com/rust-lang/rust/issues/86319

```
cargo +nightly install --path .
```
