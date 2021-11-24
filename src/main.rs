#![allow(unused_unsafe)]

mod parse_assertions;

use crate::parse_assertions::parse_position_comments;
use clap::Parser;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tree_sitter::Language;

/// implementation of Path::file_prefix (nightly)
fn os_str_as_u8_slice(s: &OsStr) -> &[u8] {
    unsafe { &*(s as *const OsStr as *const [u8]) }
}
unsafe fn u8_slice_as_os_str(s: &[u8]) -> &OsStr {
    // SAFETY: see the comment of `os_str_as_u8_slice`
    unsafe { &*(s as *const [u8] as *const OsStr) }
}

fn split_file_at_dot(file: &OsStr) -> (&OsStr, Option<&OsStr>) {
    let slice = os_str_as_u8_slice(file);
    if slice == b".." {
        return (file, None);
    }

    // The unsafety here stems from converting between &OsStr and &[u8]
    // and back. This is safe to do because (1) we only look at ASCII
    // contents of the encoding and (2) new &OsStr values are produced
    // only from ASCII-bounded slices of existing &OsStr values.
    let i = match slice[1..].iter().position(|b| *b == b'.') {
        Some(i) => i + 1,
        None => return (file, None),
    };
    let before = &slice[..i];
    let after = &slice[i + 1..];
    unsafe { (u8_slice_as_os_str(before), Some(u8_slice_as_os_str(after))) }
}

/// Loads tree_sitter::Language from a share library with name <language>.so
/// returned language must not be used after the library has been dropped!
unsafe fn load_language(
    parser_file: &Path,
) -> anyhow::Result<(Language, Box<libloading::Library>)> {
    //TODO: use once stable
    //let lang_name = parser_file.file_prefix();
    let lang_name = parser_file
        .file_name()
        .map(split_file_at_dot)
        .and_then(|(before, _after)| Some(before));
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
