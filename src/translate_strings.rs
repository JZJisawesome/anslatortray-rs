/* translate_strings.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Contains functions for translating multiple sentences.
 *
*/

/* Imports */

use crate::translate_words::translate_word_with_style;

/* Functions */

///Translates a multi-word string (including punctuation) into Pig Latin!
///
///Uses the default suffix and special_case_suffix, "ay" and "way" respectively when calling [`translate_word_with_style()`].
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
///assert_eq!(translate("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
///    "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay"
///);
///
///assert_eq!(translate("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///```
pub fn translate(english: &str) -> String {
    return translate_way(english);
}

///Translates a multi-word string (including punctuation) into Pig Latin (way-style)!
///
///Uses the suffix and special_case_suffix "ay" and "way" respectively when calling [`translate_word_with_style()`].
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
///assert_eq!(translate_way("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
///    "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay"
///);
///
///assert_eq!(translate_way("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///```
pub fn translate_way(english: &str) -> String {
    return translate_with_style(english, "ay", "way");
}

///Translates a multi-word string (including punctuation) into Pig Latin (yay-style)!
///
///Uses the suffix and special_case_suffix "ay" and "yay" respectively when calling [`translate_word_with_style()`].
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
///assert_eq!(translate_yay("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
///    "Atwhay ifyay ayay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay"
///);
///
///assert_eq!(translate_yay("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///```
pub fn translate_yay(english: &str) -> String {
    return translate_with_style(english, "ay", "yay");
}

///Translates a multi-word string (including punctuation) into Ferb Latin!
///
///Uses the suffix and special_case_suffix "erb" and "ferb" respectively when calling [`translate_word_with_style()`].
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
///assert_eq!(translate_ferb("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
///    "Atwherb ifferb aferb ordwerb asherb onerb owelsverb, ikelerb istherb: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZerb"
///);
///assert_eq!(translate_ferb("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcerb, oserb etherb euristicsherb akemerb ettyprerb oodgerb uessesgerb ithwerb atwherb eytherb're edferb!"
///);
///```
pub fn translate_ferb(english: &str) -> String {
    return translate_with_style(english, "erb", "ferb");
}

///Translates a multi-word string (including punctuation) into a custom-styled play language!
///
///Pass the string you wish to translate, the suffix you wish to have appended to most words, and the suffix
///you wish to have appended in various special-cases (such as when a word is only one letter or starts with a vowel).
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
///assert_eq!(translate_with_style("This library can translate any English text. It can even handle multiple sentences!", suffix, special_case_suffix),
///    "Isthancy ibrarylancy ancancy anslatetrancy anyfancy Englishfancy exttancy. Itfancy ancancy evenfancy andlehancy ultiplemancy entencessancy!"
///);
/////TODO
/////NOTE that this function used in an standalone fashion is not currently tested, and is thus considered experimental
///```
pub fn translate_with_style(english: &str, suffix: &str, special_case_suffix: &str) -> String {
    if english.is_empty() {
        return "".to_string();
    }

    //TODO perhaps make this multithreaded?

    let mut pig_latin_string: String = "".to_string();

    //TODO hyphens are a problem

    let mut first_iteration: bool = true;
    for word in english.split(&[' ', '\t', '\n']) {
        if !first_iteration { pig_latin_string.push(' '); }//Seperate words by spaces regardless of how they were seperated before//TODO this could be done better
        pig_latin_string.push_str(translate_word_with_style(word, suffix, special_case_suffix).as_str());
        first_iteration = false;
    }

    return pig_latin_string;
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_and_translate_way() {
        assert_eq!(translate("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");

        assert_eq!(translate("This library can translate any English text. It can even handle multiple sentences!"),
            "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!"
        );
    }

    #[test]
    fn test_translate_and_translate_way_edgecases() {
        assert_eq!(translate("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
            "Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!"
        );

        assert_eq!(translate("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
            "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay"
        );

        assert_eq!(translate("Cool, so the heuristics make pretty good guesses with what they're fed!"),
            "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
        );

        assert_eq!(translate("Hello-world"), "Ellohay-orldway");
        assert_eq!(translate("Hyphens-are-difficult-aren't-they?"), "Yphenshay-areway-ifficultday-arenway't-eythay?");
    }

    #[test]
    fn test_translate_yay() {
        assert_eq!(translate_yay("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");

        assert_eq!(translate_yay("This library can translate any English text. It can even handle multiple sentences!"),
            "Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!"
        );
    }

    #[test]
    fn test_translate_yay_edgecases() {
        assert_eq!(translate_yay("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
            "Etlay's ytray omesay edgeyay asescay. Atthay isyay ayay ontractioncay, asyay ellway asyay ayay ordway erewhay ethay onlyyay owelvay isyay yyay. Eatnay, allyay atthay orksway!"
        );

        assert_eq!(translate_yay("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
            "Atwhay ifyay ayay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZay"
        );

        assert_eq!(translate_yay("Cool, so the heuristics make pretty good guesses with what they're fed!"),
            "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
        );

        assert_eq!(translate_yay("Hello-world"), "Ellohay-orldway");
        assert_eq!(translate_yay("Hyphens-are-difficult-aren't-they?"), "Yphenshay-areyay-ifficultday-arenyay't-eythay?");
    }

    #[test]
    fn test_translate_ferb() {
        assert_eq!(translate_ferb("Hello world from the coolest Ferb Latin translator!"), "Elloherb orldwerb omfrerb etherb oolestcerb Erbferb Atinlerb anslatortrerb!");

        assert_eq!(translate_ferb("This library can translate any English text. It can even handle multiple sentences!"),
            "Istherb ibrarylerb ancerb anslatetrerb anyferb Englishferb extterb. Itferb ancerb evenferb andleherb ultiplemerb entencesserb!"
        );
    }

    #[test]
    fn test_translate_ferb_edgecases() {
        assert_eq!(translate_ferb("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
            "Etlerb's ytrerb omeserb edgeferb asescerb. Attherb isferb aferb ontractioncerb, asferb ellwerb asferb aferb ordwerb erewherb etherb onlyferb owelverb isferb yferb. Eatnerb, allferb attherb orkswerb!"
        );

        assert_eq!(translate_ferb("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ"),
            "Atwherb ifferb aferb ordwerb asherb onerb owelsverb, ikelerb istherb: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZerb"
        );

        assert_eq!(translate_ferb("Cool, so the heuristics make pretty good guesses with what they're fed!"),
            "Oolcerb, oserb etherb euristicsherb akemerb ettyprerb oodgerb uessesgerb ithwerb atwherb eytherb're edferb!"
        );
    }

    #[test]
    fn test_translate_with_style() {
        let suffix_special_case_suffix_pairs = [("ancy", "fancy"), ("orange", "porange"), ("anana", "banana"), ("atin", "latin"), ("ust", "rust")];

        for pair in suffix_special_case_suffix_pairs {
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
        let suffix_special_case_suffix_pairs = [("ancy", "fancy"), ("orange", "porange"), ("anana", "banana"), ("atin", "latin"), ("ust", "rust")];

        for pair in suffix_special_case_suffix_pairs {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_with_style("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", suffix, special_case_suffix),
                "Etl".to_string() + suffix + "'s ytr" + suffix + " omes" + suffix + " edge" + special_case_suffix + " asesc" + suffix + ". Atth" + suffix + " is" + special_case_suffix + " a" +
                special_case_suffix + " ontractionc" + suffix + ", as" + special_case_suffix + " ellw" + suffix + " as" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix +
                " erewh" + suffix + " eth" + suffix + " only" + special_case_suffix + " owelv" + suffix + " is" + special_case_suffix + " y" + special_case_suffix + ". Eatn" + suffix + ", all" +
                special_case_suffix + " atth" + suffix + " orksw" + suffix + "!"
            );

            assert_eq!(translate_with_style("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ", suffix, special_case_suffix),
                "Atwh".to_string() + suffix + " if" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix + " ash" + suffix + " on" + suffix + " owelsv" + suffix + ", ikel" + suffix + " isth" + suffix + ": bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ" + suffix
            );

            assert_eq!(translate_with_style("Cool, so the heuristics make pretty good guesses with what they're fed!", suffix, special_case_suffix),
                "Oolc".to_string() + suffix + ", os" + suffix + " eth" + suffix + " euristicsh" + suffix + " akem" + suffix + " ettypr" + suffix + " oodg" + suffix + " uessesg" + suffix + " ithw" + suffix + " atwh" + suffix + " eyth" + suffix + "'re edf" + suffix + "!"
            );

            assert_eq!(translate_with_style("Hello-world", suffix, special_case_suffix), "Ello".to_string() + suffix + "-orldw" + suffix);
            assert_eq!(translate_with_style("Hyphens-are-difficult-aren't-they?", suffix, special_case_suffix), "Yphenshay-areyay-ifficultday-arenyay't-eythay?");
        }
    }
}

/* Benches */

#[cfg_attr(feature = "nightly-features", cfg(test))]
#[cfg(feature = "nightly-features")]
mod benches {
    extern crate test;
    use test::Bencher;
    use super::*;

    #[bench]
    fn translate_project_description(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate("A simple Rust library to translate from English to Pig Latin!");
        });
    }

    #[bench]
    fn translate_yay_project_description(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_yay("A simple Rust library to translate from English to Pig Latin!");
        });
    }

    #[bench]
    fn translate_ferb_project_description(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_ferb("A simple Rust library to translate from English to Pig Latin!");
        });
    }

    #[bench]
    fn translate_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
        });
    }

    #[bench]
    fn translate_yay_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_yay("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
        });
    }

    #[bench]
    fn translate_ferb_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String {
            return translate_ferb("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
        });
    }
}
