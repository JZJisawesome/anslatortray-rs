//!Anslatortray for Rust
//!
//!Welcome to the Anslatortray Documentation!
//!
//!A simple Rust library to translate from English to Pig Latin.
//!
//!The Anslatortray library can help out by converting any English text into Pig Latin quickly and easily. It is **incredibly fast** and **requires no dependencies**!
//!
//!You can translate multiple sentences, including numbers, punctuation, and spacing, with a single call to [`translate()`].
//!The function handles edge cases quite well (words without vowels, one-letter words, contractions, ALL CAPS, etc.), though there is always room for improvement.
//!
//!If you have suggestions for how the project could be improved, please visit the repository's issues page on <a href="https://github.com/JZJisawesome/anslatortray-rs/issues">Github</a> or <a href="https://gitlab.com/JZJisawesome/anslatortray-rs/-/issues">GitLab</a> or contact me directly :)
//!
//!# Building and Installation
//!
//!If you wish to use the library in your crate, add anslatortray as a dependency and follow along with the examples below, or check out the rest of the documentation.
//!
//!See the <a href="https://git.jekel.ca/JZJ/anslatortray-rs/wiki/Building-And-Installing">wiki</a> for more information.
//!
//!# Library Examples
//!
//!Try compiling this example code:
//!
//!```
//!use anslatortray::translate;
//!
//!//Prints "Ellohay orldway omfray ethay Anslatortray orfay Ustray!"
//!println!("{}", translate("Hello world from the Translator for Rust!"));
//!```
//!
//!Anslatortray also supports using the "yay" suffix instead in special cases if you prefer that:
//!
//!```
//!use anslatortray::translate_yay;
//!
//!//Prints "Utbay Iyay eferpray ethay ayyay-ylestay igpay atinlay!"
//!println!("{}", translate_yay("But I prefer the yay-style pig latin!"));
//!```
//!
//!It also supports Ferb Latin from Phineas and Ferb:
//!
//!```
//!use anslatortray::translate_ferb;
//!
//!//Prints "Erewherb's Erryperb?"
//!println!("{}", translate_ferb("Where's Perry?"));
//!```
//!
//!If none of these suit your needs, you can also choose your own suffixes with [`translate_with_style()`]
//!
//!If you want more speed, and you know your text contains only ASCII characters, check out [`translate_ascii()`]
//!
//!# Useful Links
//!<a href="https://git.jekel.ca/JZJ/anslatortray-rs">Click here to visit the Anslatortray for Rust Git Repository!</a>.
//!
//!You can also visit the <a href="https://github.com/JZJisawesome/anslatortray-rs/issues">Github</a> or <a href="https://gitlab.com/JZJisawesome/anslatortray-rs/-/issues">GitLab</a> mirrors to leave issues!
//!
//!Be sure to check out the <a href="https://crates.io/crates/anslatortray">crates.io page</a> and the wiki at <https://git.jekel.ca/JZJ/anslatortray-rs/wiki>.
//!
//!Anslatortray for Rust is a spiritual sucessor of my original <a href="https://git.jekel.ca/JZJ/anslatortray">Anslatortray</a> (for C++).
//!
//!# Dependencies
//!
//!None other than the standard libraries!
//!
//!# Anslatortray Code and Documentation Licence
//!
//!Copyright (c) 2022 John Jekel
//!
//!MIT Licensed (see the LICENSE file for details)

/* Nightly Features */

//Only enabled if the relevant Cargo feature is
#![cfg_attr(feature = "nightly-features-benches", feature(test))]
#![cfg_attr(feature = "nightly-features-generics", feature(adt_const_params))]
#![cfg_attr(feature = "nightly-features-generics", feature(generic_const_exprs))]

/* Imports */

mod helpers;
mod translate_strings;
mod translate_words;

pub use translate_strings::{translate, translate_way, translate_yay, translate_hay, translate_ferb, translate_with_style};
pub use translate_strings::{translate_ascii, translate_way_ascii, translate_yay_ascii, translate_hay_ascii, translate_ferb_ascii, translate_with_style_ascii};
