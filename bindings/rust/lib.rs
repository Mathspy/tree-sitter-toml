// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Modified from tree-sitter-rust under the permissive license of MIT.
// Based on the grammar of tree-sitter-toml, used under the permissive license of MIT
// Copyright © 2021, tree-sitter-toml authors.
// Copyright © 2021, tree-sitter-rust authors.
// See the LICENSE file in this repo for license details.
// ------------------------------------------------------------------------------------------------

//! This crate provides TOML grammar for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [language][language func] function to add this grammar to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! use tree_sitter::Parser;
//!
//! let code = r#"
//! [package]
//! name = "cargo"
//! version = "0.1.0"
//! edition = "2021"
//! "#;
//! let mut parser = Parser::new();
//! let language: tree_sitter::Language = tree_sitter_toml::LANGUAGE.into();
//! parser.set_language(&language).expect("Error loading TOML grammar");
//! let parsed = parser.parse(code, None);
//! # let parsed = parsed.unwrap();
//! # let root = parsed.root_node();
//! # assert!(!root.has_error());
//! ```
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [language func]: fn.language.html
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_toml() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_toml) };

/// The source of the TOML tree-sitter grammar description.
pub const GRAMMAR: &str = include_str!("../../grammar.js");

/// The syntax highlighting query for this language.
pub const HIGHLIGHT_QUERY: &str = include_str!("../../queries/highlights.scm");

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

#[cfg(test)]
mod tests {
    #[test]
    fn can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        let language: tree_sitter::Language = super::LANGUAGE.into();
        parser
            .set_language(&language)
            .expect("Error loading TOML grammar");
    }
}
