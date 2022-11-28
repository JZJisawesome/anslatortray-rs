# anslatortray-rs

A simple Rust library to translate from English to Pig Latin!

<a href="https://en.wikipedia.org/wiki/Pig_Latin">Wikipedia's definition of Pig Latin</a> is "a language game or argot in which words in English are altered, usually by adding a fabricated suffix or by moving the onset or initial consonant or consonant cluster of a word to the end of the word and adding a vocalic syllable to create such a suffix."

Essentially, the word is reorganized in an effort to hide its true meaning, which can be lots of fun!

The Anslatortray library can help out by converting any English text into Pig Latin quickly and easily. It is **incredibly fast** (see the Performance section below) and **requires no dependencies**!

You can translate multiple sentences, including numbers, punctuation, and spacing, with a single call to `anslatortray::translate()`.
The function handles edge cases quite well (words without vowels, one-letter words, contractions, etc.), though there is always room for improvement.

If you have suggestions for how the project could be improved, please visit the repository's issues page on <a href="https://github.com/JZJisawesome/anslatortray-rs/issues">Github</a> or <a href="https://gitlab.com/JZJisawesome/anslatortray-rs/-/issues">GitLab</a> or contact me directly :)

Be sure to check out the documentation at <https://docs.rs/anslatortray/latest/anslatortray/>!

# Building and Installation

If you wish to use the library in your crate, add anslatortray as a dependency and follow along with the examples below, or <a href="https://docs.rs/anslatortray/latest/anslatortray/">check out the documentation</a>.

If you wish to use the anslatortray standalone binary, clone `https://git.jekel.ca/JZJ/anslatortray.git`, do cargo build --release, and you'll find the binary in the target/release directory.

See the <a href="https://git.jekel.ca/JZJ/anslatortray-rs/wiki/Building-And-Installing">wiki</a> for more information.

# Library Examples

Try compiling this example code:

```rust
use anslatortray::translate;

//Prints "Ellohay orldway omfray ethay Anslatortray orfay Ustray!"
println!("{}", translate("Hello world from the Translator for Rust!"));
```

Anslatortray also supports using the "yay" suffix instead in special cases if you prefer that:

```rust
use anslatortray::translate_yay;

//Prints "Utbay Iyay eferpray ethay ayyay-ylestay igpay atinlay!"
println!("{}", translate_yay("But I prefer the yay-style pig latin!"));
```

It also supports Ferb Latin from Phineas and Ferb:

```rust
use anslatortray::translate_ferb;

//Prints "Erewherb's Erryperb?"
println!("{}", translate_ferb("Where's Perry?"));
```

If none of these suit your needs, you can also choose your own suffixes with `anslatortray::translate_with_style()`

# anslatortray CLI Tool Example Usage

There are several options supported by the `anslatortray` command

```
> anslatetray
Error: expected at least one string to translate
> anslatetray Hello World!
Ellohay Orldway!
> anslatetray A simple Rust library to translate from English to Pig Latin!
Away implesay Ustray ibrarylay otay anslatetray omfray Englishway otay Igpay Atinlay!
```

## anslatetray-file

Translates an input file and writes the results to an output file

```
> anslatetray-file
Error: expected two arguments, the input file to be translated and the file to output the translated text to
> anslatetray-file input_but_no_output_file_specified.txt
Error: expected two arguments, the input file to be translated and the file to output the translated text to
> anslatetray-file words_alpha.txt words_alpha_pig_latin.txt
Sucessful: took 90144337ns to translate
```

The last example uses words_alpha.txt from <https://github.com/dwyl/english-words>. See below for more information.

# Performance

Check out the <a href="https://git.jekel.ca/JZJ/anslatortray-rs/wiki/Performance">wiki page about Anslatortray's performance</a>!

Spoiler: It can translate one word in under 140ns on average in the default UTF-8 mode, and in under 100ns on average in ASCII-only mode :)

# Edge Cases

The English language is very inconsistent. Sentences begin with capitals, as do proper names, there are contractions, words-with-hyphens-in-between-them, and all sorts of punctuation!

Thankfully, anslatortray's heuristics do pretty well in most common situations! Take a look at test cases in the code suffixed with _edgecases to see the sort of situations anslatortray does well in!

As always, if you find a shortcoming with the library's behaviour, please submit an issue or contact me!

# Useful Links

<a href="https://git.jekel.ca/JZJ/anslatortray-rs">Click here to visit the Anslatortray for Rust Git Repository!</a>.

You can also visit the <a href="https://github.com/JZJisawesome/anslatortray-rs/issues">Github</a> or <a href="https://gitlab.com/JZJisawesome/anslatortray-rs/-/issues">GitLab</a> mirrors to leave issues!

Be sure to check out the documentation at <https://docs.rs/anslatortray/latest/anslatortray/> and the wiki at <https://git.jekel.ca/JZJ/anslatortray-rs/wiki>.

Anslatortray for Rust is a spiritual sucessor of my original <a href="https://git.jekel.ca/JZJ/anslatortray">Anslatortray</a> (for C++).

# Dependencies

None other than the standard libraries!

# Anslatortray Code and Documentation Licence

Copyright (c) 2022 John Jekel

MIT Licensed (see the LICENSE file for details)
