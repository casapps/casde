//! Compile-time asset embedding (AI.md PART 0 -> "Self-Contained Assets").
//!
//! `assets/` is a build-time source tree only — never installed alongside
//! the binary. Themes/icons/fonts land here as TODO.AI.md "Theming" work;
//! this establishes the embedding mechanism up front.

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;
