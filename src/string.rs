/* string.rs
 * By: John Jekel
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Translation functions operating on &str and String (the ones most users will want to use)
 *
*/

/* Imports */

use crate::byte_string::translate_with_style_lower_and_upper_suffixes as translate_byte_string_with_style_lower_and_upper_suffixes;

/* Functions */

///Translates a multi-word string (including punctuation) into Pig Latin!
///
///Uses the default suffix and special_case_suffix, "ay" and "way" respectively when calling [`translate_with_style()`].
///
///Equivalent to [`translate_way()`].
///
///# Examples
///
///```
///use anslatortray::translate;
///
///assert_eq!(translate("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!"
///);
///
///assert_eq!(translate("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///
///assert_eq!(translate("Hello-world"), "Ellohay-orldway");
///assert_eq!(translate("Hyphens-are-difficult-aren't-they?"), "Yphenshay-areway-ifficultday-arenway't-eythay?");
///```
pub fn translate(english: &str) -> String {
    return translate_way(english);
}

///Translates a multi-word string (including punctuation) into Pig Latin (way-style)!
///
///Uses the suffix and special_case_suffix "ay" and "way" respectively when calling [`translate_with_style()`].
///
///# Examples
///
///```
///use anslatortray::translate_way;
///
///assert_eq!(translate_way("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate_way("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate_way("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!"
///);
///
///assert_eq!(translate_way("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate_way("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///
///assert_eq!(translate_way("Hello-world"), "Ellohay-orldway");
///assert_eq!(translate_way("Hyphens-are-difficult-aren't-they?"), "Yphenshay-areway-ifficultday-arenway't-eythay?");
///```
pub fn translate_way(english: &str) -> String {
    return translate_with_style_lower_and_upper_suffixes(english, "ay", "way", "AY", "WAY");
}

///Translates a multi-word string (including punctuation) into Pig Latin (yay-style)!
///
///Uses the suffix and special_case_suffix "ay" and "yay" respectively when calling [`translate_with_style()`].
///
///# Examples
///
///```
///use anslatortray::translate_yay;
///
///assert_eq!(translate_yay("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate_yay("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate_yay("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgeyay asescay. Atthay isyay ayay ontractioncay, asyay ellway asyay ayay ordway erewhay ethay onlyyay owelvay isyay yyay. Eatnay, allyay atthay orksway!"
///);
///
///assert_eq!(translate_yay("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifyay ayay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate_yay("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///
///assert_eq!(translate_yay("Hello-world"), "Ellohay-orldway");
///assert_eq!(translate_yay("Hyphens-are-difficult-aren't-they?"), "Yphenshay-areyay-ifficultday-arenyay't-eythay?");
///```
pub fn translate_yay(english: &str) -> String {
    return translate_with_style_lower_and_upper_suffixes(english, "ay", "yay", "AY", "YAY");
}

///Translates a multi-word string (including punctuation) into Pig Latin (hay-style)!
///
///Uses the suffix and special_case_suffix "ay" and "hay" respectively when calling [`translate_with_style()`].
///
///# Examples
///
///```
///use anslatortray::translate_hay;
///
///assert_eq!(translate_hay("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate_hay("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyhay Englishhay exttay. Ithay ancay evenhay andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate_hay("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgehay asescay. Atthay ishay ahay ontractioncay, ashay ellway ashay ahay ordway erewhay ethay onlyhay owelvay ishay yhay. Eatnay, allhay atthay orksway!"
///);
///
///assert_eq!(translate_hay("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifhay ahay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate_hay("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///
///assert_eq!(translate_hay("Hello-world"), "Ellohay-orldway");
///assert_eq!(translate_hay("Hyphens-are-difficult-aren't-they?"), "Yphenshay-arehay-ifficultday-arenhay't-eythay?");
///```
pub fn translate_hay(english: &str) -> String {
    return translate_with_style_lower_and_upper_suffixes(english, "ay", "hay", "AY", "HAY");
}

///Translates a multi-word string (including punctuation) into Ferb Latin!
///
///Uses the suffix and special_case_suffix "erb" and "ferb" respectively when calling [`translate_with_style()`].
///
///# Examples
///
///```
///use anslatortray::translate_ferb;
///
///assert_eq!(translate_ferb("Hello world from the coolest Ferb Latin translator!"), "Elloherb orldwerb omfrerb etherb oolestcerb Erbferb Atinlerb anslatortrerb!");
///
///assert_eq!(translate_ferb("This library can translate any English text. It can even handle multiple sentences!"),
///    "Istherb ibrarylerb ancerb anslatetrerb anyferb Englishferb extterb. Itferb ancerb evenferb andleherb ultiplemerb entencesserb!"
///);
///
///assert_eq!(translate_ferb("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlerb's ytrerb omeserb edgeferb asescerb. Attherb isferb aferb ontractioncerb, asferb ellwerb asferb aferb ordwerb erewherb etherb onlyferb owelverb isferb yferb. Eatnerb, allferb attherb orkswerb!"
///);
///assert_eq!(translate_ferb("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwherb ifferb aferb ordwerb asherb onerb owelsverb, ikelerb istherb: bcdfghjklmnpqrstvwxzerb"
///);
///assert_eq!(translate_ferb("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcerb, oserb etherb euristicsherb akemerb ettyprerb oodgerb uessesgerb ithwerb atwherb eytherb're edferb!"
///);
///
///assert_eq!(translate_ferb("Hello-world"), "Elloherb-orldwerb");
///assert_eq!(translate_ferb("Hyphens-are-difficult-aren't-they?"), "Yphensherb-areferb-ifficultderb-arenferb't-eytherb?");
///```
pub fn translate_ferb(english: &str) -> String {
    return translate_with_style_lower_and_upper_suffixes(english, "erb", "ferb", "ERB", "FERB");
}

///Translates a multi-word string (including punctuation) into a custom-styled play language!
///
///Pass the string you wish to translate, the suffix you wish to have appended to most words, and the suffix
///you wish to have appended in various special-cases (such as when a word is only one letter or starts with a vowel).
///
///NOTE: The suffixes must be entirely lower-case or weird results may occur.
///
///# Examples
///
///```
///use anslatortray::translate_with_style;
///
///let suffix = "ancy";
///let special_case_suffix = "fancy";
///assert_eq!(translate_with_style("Hello world from the coolest Pig Latin translator!", suffix, special_case_suffix),
///    "Ellohancy orldwancy omfrancy ethancy oolestcancy Igpancy Atinlancy anslatortrancy!"
///);
///
///assert_eq!(translate_with_style("This library can translate any English text. It can even handle multiple sentences!", suffix, special_case_suffix),
///    "Isthancy ibrarylancy ancancy anslatetrancy anyfancy Englishfancy exttancy. Itfancy ancancy evenfancy andlehancy ultiplemancy entencessancy!"
///);
///
///assert_eq!(translate_with_style("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", suffix, special_case_suffix),
///    "Etl".to_string() + suffix + "'s ytr" + suffix + " omes" + suffix + " edge" + special_case_suffix + " asesc" + suffix + ". Atth" + suffix + " is" + special_case_suffix + " a" +
///    special_case_suffix + " ontractionc" + suffix + ", as" + special_case_suffix + " ellw" + suffix + " as" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix +
///    " erewh" + suffix + " eth" + suffix + " only" + special_case_suffix + " owelv" + suffix + " is" + special_case_suffix + " y" + special_case_suffix + ". Eatn" + suffix + ", all" +
///    special_case_suffix + " atth" + suffix + " orksw" + suffix + "!"
///);
///
///assert_eq!(translate_with_style("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", suffix, special_case_suffix),
///    "Atwh".to_string() + suffix + " if" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix + " ash" + suffix + " on" + suffix + " owelsv" + suffix + ", ikel" + suffix + " isth" + suffix + ": bcdfghjklmnpqrstvwxz" + suffix
///);
///
///assert_eq!(translate_with_style("Cool, so the heuristics make pretty good guesses with what they're fed!", suffix, special_case_suffix),
///    "Oolc".to_string() + suffix + ", os" + suffix + " eth" + suffix + " euristicsh" + suffix + " akem" + suffix + " ettypr" + suffix + " oodg" + suffix + " uessesg" + suffix + " ithw" + suffix + " atwh" + suffix + " eyth" + suffix + "'re edf" + suffix + "!"
///);
///
///assert_eq!(translate_with_style("Hello-world", suffix, special_case_suffix), "Elloh".to_string() + suffix + "-orldw" + suffix);
///
///assert_eq!(translate_with_style("Hyphens-are-difficult-aren't-they?", suffix, special_case_suffix),
///    "Yphensh".to_string() + suffix + "-are" + special_case_suffix + "-ifficultd" + suffix + "-aren" + special_case_suffix + "'t-eyth" + suffix + "?"
///);
///```
pub fn translate_with_style(english: &str, suffix_lower: &str, special_case_suffix_lower: &str) -> String {
    //Convert the suffix and special_case_suffix we were provided to uppercase for words that are capitalized
    let mut suffix_upper = String::with_capacity(suffix_lower.len());
    for letter in suffix_lower.chars() {
        suffix_upper.push(letter.to_ascii_uppercase());
    }
    let mut special_case_suffix_upper = String::with_capacity(special_case_suffix_lower.len());
    for letter in special_case_suffix_lower.chars() {
        special_case_suffix_upper.push(letter.to_ascii_uppercase());
    }

    return translate_with_style_lower_and_upper_suffixes (
        english,
        suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper
    );
}

//More efficient: Does not need to convert to upppercase at runtime
fn translate_with_style_lower_and_upper_suffixes (
    english: &str,
    suffix_lower: &str, special_case_suffix_lower: &str, suffix_upper: &str, special_case_suffix_upper: &str
) -> String {
    //Convert the string slices to byte slices and translate those (only ASCII letters are affected, non-letters or UTF-8 are preserved)
    let mut pig_latin_string_bytes = Vec::<u8>::with_capacity(english.len() * 2);//Plenty of headroom in case the words are very small or the suffixes are long
    translate_byte_string_with_style_lower_and_upper_suffixes (
        english.as_bytes(),
        suffix_lower.as_bytes(), special_case_suffix_lower.as_bytes(), suffix_upper.as_bytes(), special_case_suffix_upper.as_bytes(),
        &mut pig_latin_string_bytes
    );

    //This is safe since translate_byte_string_with_style does not touch any unicode bytes (it just copies them)
    return unsafe { String::from_utf8_unchecked(pig_latin_string_bytes) };
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;
    const SUFFIX_SPECIAL_CASE_SUFFIX_PAIRS: [(&str, &str); 9] = [
        ("ay", "way"), ("ay", "yay"), ("ay", "hay"), ("erb", "ferb"), ("ancy", "fancy"), ("orange", "porange"), ("anana", "banana"), ("atin", "latin"), ("ust", "rust")
    ];
    const SUFFIX_SPECIAL_CASE_SUFFIX_LOWER_UPPER_TUPLES: [(&str, &str, &str, &str); 9] = [
        ("ay", "way", "AY", "WAY"), ("ay", "yay", "AY", "YAY"), ("ay", "hay", "AY", "HAY"), ("erb", "ferb", "ERB", "FERB"),
        ("ancy", "fancy", "ANCY", "FANCY"), ("orange", "porange", "ORANGE", "PORANGE"),
        ("anana", "banana", "ANANA", "BANANA"), ("atin", "latin", "ATIN", "LATIN"), ("ust", "rust", "UST", "RUST"),
    ];

    #[test]
    fn test_translate_with_style() {
        for pair in SUFFIX_SPECIAL_CASE_SUFFIX_PAIRS {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_with_style("Hello world from the coolest Pig Latin translator!", suffix, special_case_suffix),
                "Elloh".to_string() + suffix + " orldw" + suffix + " omfr" + suffix + " eth" + suffix + " oolestc" + suffix + " Igp" + suffix + " Atinl" + suffix + " anslatortr" + suffix + "!"
            );

            assert_eq!(translate_with_style("This library can translate any English text. It can even handle multiple sentences!", suffix, special_case_suffix),
                "Isth".to_string() + suffix + " ibraryl" + suffix + " anc" + suffix + " anslatetr" + suffix + " any" + special_case_suffix + " English" + special_case_suffix + " extt" + suffix +
                ". It" + special_case_suffix + " anc" + suffix + " even" + special_case_suffix + " andleh" + suffix + " ultiplem" + suffix + " entencess" + suffix + "!"
            );
        }
    }

    #[test]
    fn test_translate_with_style_edgecases() {
        for pair in SUFFIX_SPECIAL_CASE_SUFFIX_PAIRS {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_with_style("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", suffix, special_case_suffix),
                "Etl".to_string() + suffix + "'s ytr" + suffix + " omes" + suffix + " edge" + special_case_suffix + " asesc" + suffix + ". Atth" + suffix + " is" + special_case_suffix + " a" +
                special_case_suffix + " ontractionc" + suffix + ", as" + special_case_suffix + " ellw" + suffix + " as" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix +
                " erewh" + suffix + " eth" + suffix + " only" + special_case_suffix + " owelv" + suffix + " is" + special_case_suffix + " y" + special_case_suffix + ". Eatn" + suffix + ", all" +
                special_case_suffix + " atth" + suffix + " orksw" + suffix + "!"
            );

            assert_eq!(translate_with_style("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", suffix, special_case_suffix),
                "Atwh".to_string() + suffix + " if" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix + " ash" + suffix + " on" + suffix + " owelsv" + suffix + ", ikel" + suffix + " isth" + suffix + ": bcdfghjklmnpqrstvwxz" + suffix
            );

            assert_eq!(translate_with_style("Cool, so the heuristics make pretty good guesses with what they're fed!", suffix, special_case_suffix),
                "Oolc".to_string() + suffix + ", os" + suffix + " eth" + suffix + " euristicsh" + suffix + " akem" + suffix + " ettypr" + suffix + " oodg" + suffix + " uessesg" + suffix + " ithw" + suffix + " atwh" + suffix + " eyth" + suffix + "'re edf" + suffix + "!"
            );

            assert_eq!(translate_with_style("Hello-world", suffix, special_case_suffix), "Elloh".to_string() + suffix + "-orldw" + suffix);

            assert_eq!(translate_with_style("Hyphens-are-difficult-aren't-they?", suffix, special_case_suffix),
                "Yphensh".to_string() + suffix + "-are" + special_case_suffix + "-ifficultd" + suffix + "-aren" + special_case_suffix + "'t-eyth" + suffix + "?"
            );
        }
    }

    #[test]
    fn test_translate_with_style_uppercase() {
        for pair in SUFFIX_SPECIAL_CASE_SUFFIX_LOWER_UPPER_TUPLES {
            let suffix_lower = pair.0;
            let special_case_suffix_lower = pair.1;
            let suffix_upper = pair.2;
            let special_case_suffix_upper = pair.3;

            assert_eq!(translate_with_style("HELLO WORLD!", suffix_lower, special_case_suffix_lower),
                "ELLOH".to_string() + suffix_upper + " ORLDW" + suffix_upper + "!"
            );

            assert_eq!(translate_with_style("ISN't THIS COOL?", suffix_lower, special_case_suffix_lower),
                "ISN".to_string() + special_case_suffix_upper + "'t ISTH" + suffix_upper + " OOLC" + suffix_upper + "?"
            );

            assert_eq!(translate_with_style("What ABOUT a MIX?", suffix_lower, special_case_suffix_lower),
                "Atwh".to_string() + suffix_lower + " ABOUT" + special_case_suffix_upper + " a" + special_case_suffix_lower + " IXM" + suffix_upper + "?"
            );

            assert_eq!(translate_with_style("Luke, I am your father!", suffix_lower, special_case_suffix_lower),//We don't want to capitalize single-letter words
                "Ukel".to_string() + suffix_lower + ", I" + special_case_suffix_lower+ " am" + special_case_suffix_lower + " oury" + suffix_lower + " atherf" + suffix_lower + "!"
            );
        }
    }
}

/* Benches */

#[cfg_attr(feature = "nightly-features-benches", cfg(test))]
#[cfg(feature = "nightly-features-benches")]
mod benches {
    extern crate test;
    use test::Bencher;
    use super::*;

    const PROJECT_DESCRIPTION: &str = "A simple Rust library to translate from English to Pig Latin!";
    const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    #[bench]
    fn way_project_description(b: &mut Bencher) {
        b.iter(|| -> String { return translate(PROJECT_DESCRIPTION); });
    }

    #[bench]
    fn yay_project_description(b: &mut Bencher) {
        b.iter(|| -> String { return translate_yay(PROJECT_DESCRIPTION); });
    }

    #[bench]
    fn hay_project_description(b: &mut Bencher) {
        b.iter(|| -> String { return translate_hay(PROJECT_DESCRIPTION); });
    }

    #[bench]
    fn ferb_project_description(b: &mut Bencher) {
        b.iter(|| -> String { return translate_ferb(PROJECT_DESCRIPTION); });
    }

    #[bench]
    fn way_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String { return translate(LOREM_IPSUM); });
    }

    #[bench]
    fn yay_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String { return translate_yay(LOREM_IPSUM); });
    }

    #[bench]
    fn hay_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String { return translate_hay(LOREM_IPSUM); });
    }

    #[bench]
    fn ferb_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String { return translate_ferb(LOREM_IPSUM); });
    }
}
