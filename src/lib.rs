//!Anslatortray for Rust: A simple Rust library to translate from English to Pig Latin.
//!
//!Welcome to the Anslatortray Documentation!
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
//!If you want even more speed than the regular translation functions bring to the table, check out the [`byte_string`] module.
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

/* Imports */

pub mod byte_string;
mod string;

pub use string::{translate, translate_way, translate_yay, translate_hay, translate_ferb, translate_with_style};
