# Anslatortray for Rust

A simple Rust library to translate from English to Pig Latin!

<a href="https://en.wikipedia.org/wiki/Pig_Latin">Wikipedia's definition of Pig Latin</a> is "a language game or argot in which words in English are altered, usually by adding a fabricated suffix or by moving the onset or initial consonant or consonant cluster of a word to the end of the word and adding a vocalic syllable to create such a suffix."

Essentially, the word is reorganized in an effort to hide its true meaning, which can be lots of fun!

# A Quick Example

After adding Anslatortray as a dependency in your crate, try compiling this example code:

```rust
use anslatortray::translate;

fn main() {
    //Prints "Ellohay orldway omfray ethay Anslatortray orfay Ustray!"
    println!("{}", translate("Hello world from the Translator for Rust!"));
}
```

# Tell me more!

The Anslatortray library can help out by converting any English text into Pig Latin quickly and easily. It is **incredibly fast** (see the Performance section below) and **requires no dependencies**!

You can translate multiple sentences, including numbers, punctuation, and spacing, with a single call to `anslatortray::translate()`.
The function handles edge cases quite well (words without vowels, one-letter words, contractions, ALL CAPS, etc.), though there is always room for improvement.

If you have suggestions for how the project could be improved, please visit the repository's issues page on <a href="https://github.com/JZJisawesome/anslatortray-rs/issues">Github</a> or <a href="https://gitlab.com/JZJisawesome/anslatortray-rs/-/issues">GitLab</a> or contact me directly :)

Be sure to check out the documentation at <https://docs.rs/anslatortray/latest/anslatortray/>!

# Building and Installation

If you wish to use the library in your crate, add anslatortray as a dependency and <a href="https://docs.rs/anslatortray/latest/anslatortray/">check out the documentation</a>.

If you wish to use the `anslatortray` standalone binary (shown in the next section), clone `https://git.jekel.ca/JZJ/anslatortray.git`, do `cargo build --release`, and you'll find the binary in the target/release directory.

See the <a href="https://git.jekel.ca/JZJ/anslatortray-rs/wiki/Building-And-Installing">wiki</a> for more information.

# anslatortray CLI Tool Usage

There are several options supported by the `anslatortray` command:

```
> anslatortray --help
Anslatortray: frontend for the Anslatortray for Rust library

Options:
--help            Print this helpful text!
--interactive     Start an interactive translation session
--file            Translate a file (requires two arguments, the file to translate and the destination)
--benchmark-file  Benchmark translating a file (requires two arguments, the file to translate and the number of iterations to perform)
--translate-args  Translates all remaining arguments provided and outputs them to stdout
--stdin-to-stdout Translates input from stdin directly to stdout

Avehay away oodgay ayday!
```

You can start an interactive session by specifying --interactive (or no arguments at all):

```
> anslatortray --interactive
Anslatortray: frontend for the Anslatortray for Rust library

Starting interactive mode!
Type what you'd like to translate and then press enter, or press Ctrl+C to exit...

anslatortray> The fitness gram pacer test is a multi-stage areobic endurance test that...
Ethay itnessfay amgray acerpay esttay isway away ultimay-agestay areobicway enduranceway esttay atthay...

anslatortray> ^C
>
```

You can also pipe text into the command for use in scripting:

```
> echo "Testing pipes" | anslatortray --stdin-to-stdout > test_pipes.txt
Anslatortray: frontend for the Anslatortray for Rust library

> cat test_pipes.txt
Estingtay ipespay
```

If you'd like, you can even translate a text file:

```
> echo "Test file" > test_file.txt && cat test_file.txt
Test file
> anslatortray --file test_file.txt output_file.txt
Anslatortray: frontend for the Anslatortray for Rust library

Sucessful: took 3540ns to translate
> cat output_file.txt
Esttay ilefay
```

See <a href="https://git.jekel.ca/JZJ/anslatortray-rs/wiki/Using-the-anslatortray-binary">this wiki page</a> for more!

# Performance

Check out the <a href="https://git.jekel.ca/JZJ/anslatortray-rs/wiki/Performance">wiki page about Anslatortray's performance</a>!

Spoiler: `anslatortray::translate()` can process one word in under **50ns** on average!

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
