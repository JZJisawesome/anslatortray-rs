# anslatortray-rs

A simple Rust library to translate from English to Pig Latin!

<a href="https://en.wikipedia.org/wiki/Pig_Latin">Wikipedia's definition of Pig Latin</a> is "a language game or argot in which words in English are altered, usually by adding a fabricated suffix or by moving the onset or initial consonant or consonant cluster of a word to the end of the word and adding a vocalic syllable to create such a suffix."

Essentially, the word is reorganized in an effort to hide its true meaning, which can be lots of fun!

The Anslatortray library can help out by converting any English text into Pig Latin quickly and easily.

You can translate multiple sentences, including numbers, punctuation, and spacing, with a single call to `anslatortray::translate()`.
The function handles edge cases quite well (words without vowels, one-letter words, contractions, etc.), though there is always room for improvement.

If you have suggestions for how the project could be improved, please visit the repository's issues page on <a href="https://github.com/JZJisawesome/anslatortray-rs/issues">Github</a> or <a href="https://gitlab.com/JZJisawesome/anslatortray-rs/-/issues">GitLab</a> or contact me directly :)

Be sure to check out the documentation at <https://docs.rs/anslatortray/latest/anslatortray/>!

# Library Examples

Try compiling this example code:

```
use anslatortray::translate;

//Prints "Ellohay orldway omfray ethay Anslatortray orfay Ustray!"
println!("{}", translate("Hello world from the Translator for Rust!"));
```

Anslatortray also supports using the "yay" suffix instead in special cases if you prefer that:

```
use anslatortray::translate_yay;

//Prints "Utbay Iyay eferpray ethay ayyay-ylestay igpay atinlay!"
println!("{}", translate_yay("But I prefer the yay-style pig latin!"));
```

It also supports Ferb Latin from Phineas and Ferb:

```
use anslatortray::translate_ferb;

//Prints "Erewherb's Erryperb?"
println!("{}", translate_ferb("Where's Perry?"));
```

If none of these suit your needs, you can also choose your own suffixes with `anslatortray::translate_with_style()`

# Included Programs Example Usage

## anslatetray

Translates command line arguments passed to it

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
Sucessful: took 62204293ns to translate
```

The last example uses words_alpha.txt from <https://github.com/dwyl/english-words>. See below for more information.

# Performance

On my dated system with dual Intel(R) Xeon(R) E5-2670 v2 CPUs, the translate() function can process one word every 156.886 nanoseconds on average.
I tested this by feeding the words_alpha.txt file from <https://github.com/dwyl/english-words> to anslatetray-file 10 times, calculating the average runtime,
and dividing by 370105 (the number of words in the file). The times do not including loading from and writing to the disk.

```
> for run in {1..10}; do anslatetray-file words_alpha.txt words_alpha_pig_latin.txt; done
Sucessful: took 62204293ns to translate
Sucessful: took 62058042ns to translate
Sucessful: took 61924286ns to translate
Sucessful: took 61322023ns to translate
Sucessful: took 55860108ns to translate
Sucessful: took 56094747ns to translate
Sucessful: took 55327630ns to translate
Sucessful: took 55683902ns to translate
Sucessful: took 54908469ns to translate
Sucessful: took 55269018ns to translate
```

This is quite fast, but it could be faster :). Both the speed and the quality of translation are priorities for me, and I'm working to improve them both!

# Useful Links

<a href="https://git.jekel.ca/JZJ/anslatortray-rs">Click here to visit the Anslatortray for Rust Git Repository!</a>.

You can also visit the <a href="https://github.com/JZJisawesome/anslatortray-rs/issues">Github</a> or <a href="https://gitlab.com/JZJisawesome/anslatortray-rs/-/issues">GitLab</a> mirrors to leave issues!

Be sure to check out the documentation at <https://docs.rs/anslatortray/latest/anslatortray/>!

Anslatortray for Rust is a spiritual sucessor of my original <a href="https://git.jekel.ca/JZJ/anslatortray">Anslatortray</a> (for C++).

# Dependencies

None other than the standard libraries!

# Anslatortray Code and Documentation Licence

MIT License

Copyright (c) 2022 John Jekel

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
