# highlight-assertions

Reads the unit test format for highlighting of tree-sitter
https://tree-sitter.github.io/tree-sitter/syntax-highlighting#unit-testing to make it available for
unit test for https://github.com/nvim-treesitter/nvim-treesitter

## Usage

```
highlight-assertions 1.0

Stephan Seitz <stephan.seitz@fau.de>

USAGE:
    highlight-assertions --parser-file <PARSER_FILE> --source-file <SOURCE_FILE>

OPTIONS:
    -h, --help                         Print help information
    -p, --parser-file <PARSER_FILE>    Parser library to load (e.g. cpp.so from nvim-treesitter/parser)
    -s, --source-file <SOURCE_FILE>    Source file with highlight assertions following https://tree-
                                       sitter.github.io/tree-sitter/syntax-highlighting#unit-testing
    -V, --version                      Print version information
```
Output will be printed as JSON to stdout.
