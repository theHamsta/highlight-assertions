#![feature(path_file_prefix)]
#![allow(unused_unsafe)]

mod parse_assertions;

use crate::parse_assertions::parse_position_comments;
use clap::Parser;
use std::path::{Path, PathBuf};
use tree_sitter::Language;

/// Loads tree_sitter::Language from a share library with name <language>.so
/// returned language must not be used after the library has been dropped!
unsafe fn load_language(
    parser_file: &Path,
) -> anyhow::Result<(Language, Box<libloading::Library>)> {
    let lang_name = parser_file.file_prefix();
    unsafe {
        let lib = Box::new(libloading::Library::new(&*parser_file.to_string_lossy())?);
        let func: libloading::Symbol<unsafe extern "C" fn() -> Language> = lib.get(
            format!(
                "tree_sitter_{}",
                lang_name
                    .ok_or_else(|| anyhow::anyhow!("Failed to get file_prefix!"))?
                    .to_string_lossy()
                    .to_owned()
            )
            .as_ref(),
        )?;
        Ok((func(), lib))
    }
}

/// Reads the unit test format for highlighting of tree-sitter
/// https://tree-sitter.github.io/tree-sitter/syntax-highlighting#unit-testing to make it available for
/// unit test for https://github.com/nvim-treesitter/nvim-treesitter.
/// Output will be printed to stdout.
#[derive(clap::Parser)]
#[clap(version = "1.0", author = "Stephan Seitz <stephan.seitz@fau.de>")]
struct Args {
    /// Parser library to load (e.g. cpp.so from nvim-treesitter/parser)
    #[clap(short, long)]
    parser_file: PathBuf,

    /// Source file with highlight assertions following https://tree-sitter.github.io/tree-sitter/syntax-highlighting#unit-testing
    #[clap(short, long)]
    source_file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (language, _lib) = unsafe { load_language(&args.parser_file)? };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language)?;

    let assertions = parse_position_comments(
        &mut parser,
        language,
        std::fs::read_to_string(args.source_file)?.as_ref(),
    )?;
    println!("{}", serde_json::to_string(&assertions)?);
    Ok(())
}
