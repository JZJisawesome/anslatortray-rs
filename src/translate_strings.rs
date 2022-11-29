/* translate_strings.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Contains functions for translating multiple sentences.
 *
*/

/* Imports */

use crate::translate_words::translate_word_with_style_reuse_buffers;
use crate::translate_words::translate_word_with_style_reuse_buffers_ascii;

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

///Translates a multi-word string (including punctuation) into Pig Latin (Faster, but ASCII-only)!
///
///Faster than [`translate()`], but requires that the string only contains ASCII characters or else it may panic.
///
///Uses the default suffix and special_case_suffix, "ay" and "way" respectively when calling [`translate_with_style_ascii()`].
///
///Equivalent to [`translate_way_ascii()`].
///
///# Examples
///
///```
///use anslatortray::translate_ascii;
///
///assert_eq!(translate_ascii("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate_ascii("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate_ascii("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!"
///);
///
///assert_eq!(translate_ascii("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate_ascii("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///
///assert_eq!(translate_ascii("Hello-world"), "Ellohay-orldway");
///assert_eq!(translate_ascii("Hyphens-are-difficult-aren't-they?"), "Yphenshay-areway-ifficultday-arenway't-eythay?");
///```
pub fn translate_ascii(english: &str) -> String {
    return translate_way_ascii(english);
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
    #[cfg(not(feature = "nightly-features"))]
    return translate_with_style(english, "ay", "way");
    #[cfg(feature = "nightly-features")]
    return translate_with_style_generic::<"ay", "way", "AY", "WAY">(english);
}

///Translates a multi-word string (including punctuation) into Pig Latin (way-style) (Faster, but ASCII-only)!
///
///Faster than [`translate_way()`], but requires that the string only contains ASCII characters or else it may panic.
///
///Uses the suffix and special_case_suffix "ay" and "way" respectively when calling [`translate_with_style_ascii()`].
///
///# Examples
///
///```
///use anslatortray::translate_way_ascii;
///
///assert_eq!(translate_way_ascii("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate_way_ascii("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyway Englishway exttay. Itway ancay evenway andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate_way_ascii("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgeway asescay. Atthay isway away ontractioncay, asway ellway asway away ordway erewhay ethay onlyway owelvay isway yway. Eatnay, allway atthay orksway!"
///);
///
///assert_eq!(translate_way_ascii("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifway away ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate_way_ascii("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///
///assert_eq!(translate_way_ascii("Hello-world"), "Ellohay-orldway");
///assert_eq!(translate_way_ascii("Hyphens-are-difficult-aren't-they?"), "Yphenshay-areway-ifficultday-arenway't-eythay?");
///```
pub fn translate_way_ascii(english: &str) -> String {
    return translate_with_style_ascii(english, "ay", "way");
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
    #[cfg(not(feature = "nightly-features"))]
    return translate_with_style(english, "ay", "yay");
    #[cfg(feature = "nightly-features")]
    return translate_with_style_generic::<"ay", "yay", "AY", "YAY">(english);
}

///Translates a multi-word string (including punctuation) into Pig Latin (yay-style) (Faster, but ASCII-only)!
///
///Faster than [`translate_yay()`], but requires that the string only contains ASCII characters or else it may panic.
///
///Uses the suffix and special_case_suffix "ay" and "yay" respectively when calling [`translate_with_style_ascii()`].
///
///# Examples
///
///```
///use anslatortray::translate_yay_ascii;
///
///assert_eq!(translate_yay_ascii("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate_yay_ascii("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyyay Englishyay exttay. Ityay ancay evenyay andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate_yay_ascii("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgeyay asescay. Atthay isyay ayay ontractioncay, asyay ellway asyay ayay ordway erewhay ethay onlyyay owelvay isyay yyay. Eatnay, allyay atthay orksway!"
///);
///
///assert_eq!(translate_yay_ascii("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifyay ayay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate_yay_ascii("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///
///assert_eq!(translate_yay_ascii("Hello-world"), "Ellohay-orldway");
///assert_eq!(translate_yay_ascii("Hyphens-are-difficult-aren't-they?"), "Yphenshay-areyay-ifficultday-arenyay't-eythay?");
///```
pub fn translate_yay_ascii(english: &str) -> String {
    return translate_with_style_ascii(english, "ay", "yay");
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
    #[cfg(not(feature = "nightly-features"))]
    return translate_with_style(english, "ay", "hay");
    #[cfg(feature = "nightly-features")]
    return translate_with_style_generic::<"ay", "hay", "AY", "HAY">(english);
}

///Translates a multi-word string (including punctuation) into Pig Latin (hay-style) (Faster, but ASCII-only)!
///
///Faster than [`translate_hay()`], but requires that the string only contains ASCII characters or else it may panic.
///
///Uses the suffix and special_case_suffix "ay" and "hay" respectively when calling [`translate_with_style_ascii()`].
///
///# Examples
///
///```
///use anslatortray::translate_hay_ascii;
///
///assert_eq!(translate_hay_ascii("Hello world from the coolest Pig Latin translator!"), "Ellohay orldway omfray ethay oolestcay Igpay Atinlay anslatortray!");
///
///assert_eq!(translate_hay_ascii("This library can translate any English text. It can even handle multiple sentences!"),
///    "Isthay ibrarylay ancay anslatetray anyhay Englishhay exttay. Ithay ancay evenhay andlehay ultiplemay entencessay!"
///);
///
///assert_eq!(translate_hay_ascii("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlay's ytray omesay edgehay asescay. Atthay ishay ahay ontractioncay, ashay ellway ashay ahay ordway erewhay ethay onlyhay owelvay ishay yhay. Eatnay, allhay atthay orksway!"
///);
///
///assert_eq!(translate_hay_ascii("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwhay ifhay ahay ordway ashay onay owelsvay, ikelay isthay: bcdfghjklmnpqrstvwxzay"
///);
///
///assert_eq!(translate_hay_ascii("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcay, osay ethay euristicshay akemay ettypray oodgay uessesgay ithway atwhay eythay're edfay!"
///);
///
///assert_eq!(translate_hay_ascii("Hello-world"), "Ellohay-orldway");
///assert_eq!(translate_hay_ascii("Hyphens-are-difficult-aren't-they?"), "Yphenshay-arehay-ifficultday-arenhay't-eythay?");
///```
pub fn translate_hay_ascii(english: &str) -> String {
    return translate_with_style_ascii(english, "ay", "hay");
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
    #[cfg(not(feature = "nightly-features"))]
    return translate_with_style(english, "erb", "ferb");
    #[cfg(feature = "nightly-features")]
    return translate_with_style_generic::<"erb", "ferb", "ERB", "FERB">(english);
}

///Translates a multi-word string (including punctuation) into Ferb Latin (Faster, but ASCII-only)!
///
///Faster than [`translate_hay()`], but requires that the string only contains ASCII characters or else it may panic.
///
///Uses the suffix and special_case_suffix "erb" and "ferb" respectively when calling [`translate_with_style_ascii()`].
///
///# Examples
///
///```
///use anslatortray::translate_ferb_ascii;
///
///assert_eq!(translate_ferb_ascii("Hello world from the coolest Ferb Latin translator!"), "Elloherb orldwerb omfrerb etherb oolestcerb Erbferb Atinlerb anslatortrerb!");
///
///assert_eq!(translate_ferb_ascii("This library can translate any English text. It can even handle multiple sentences!"),
///    "Istherb ibrarylerb ancerb anslatetrerb anyferb Englishferb extterb. Itferb ancerb evenferb andleherb ultiplemerb entencesserb!"
///);
///
///assert_eq!(translate_ferb_ascii("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!"),
///    "Etlerb's ytrerb omeserb edgeferb asescerb. Attherb isferb aferb ontractioncerb, asferb ellwerb asferb aferb ordwerb erewherb etherb onlyferb owelverb isferb yferb. Eatnerb, allferb attherb orkswerb!"
///);
///assert_eq!(translate_ferb_ascii("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz"),
///    "Atwherb ifferb aferb ordwerb asherb onerb owelsverb, ikelerb istherb: bcdfghjklmnpqrstvwxzerb"
///);
///assert_eq!(translate_ferb_ascii("Cool, so the heuristics make pretty good guesses with what they're fed!"),
///    "Oolcerb, oserb etherb euristicsherb akemerb ettyprerb oodgerb uessesgerb ithwerb atwherb eytherb're edferb!"
///);
///
///assert_eq!(translate_ferb_ascii("Hello-world"), "Elloherb-orldwerb");
///assert_eq!(translate_ferb_ascii("Hyphens-are-difficult-aren't-they?"), "Yphensherb-areferb-ifficultderb-arenferb't-eytherb?");
///```
pub fn translate_ferb_ascii(english: &str) -> String {
    return translate_with_style_ascii(english, "erb", "ferb");
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
    if english.is_empty() {
        return String::new();
    }

    //Convert the suffix and special_case_suffix we were provided to uppercase for words that are capitalized
    let mut suffix_upper = String::with_capacity(suffix_lower.len());
    for letter in suffix_lower.chars() {
        suffix_upper.push(letter.to_ascii_uppercase());
    }
    let mut special_case_suffix_upper = String::with_capacity(special_case_suffix_lower.len());
    for letter in special_case_suffix_lower.chars() {
        special_case_suffix_upper.push(letter.to_ascii_uppercase());
    }

    let mut pig_latin_string = String::with_capacity(english.len() * 2);//Plenty of headroom in case the words are very small or the suffixes are long
    let mut current_word = String::with_capacity(64);//Longer than all English words to avoid unneeded allocations (plus leaving room for leading and trailing extra characters)
    let mut contraction_suffix = String::with_capacity(64);
    let mut in_word: bool = false;
    let mut in_contraction_suffix: bool = false;

    //Buffers for improved performance (avoid repeated heap allocations)
    let mut starting_consonants_buffer = String::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

    for character in english.chars() {
        if in_word {
            if character.is_alphabetic() {
                //Save the character to translate once the word ends; we also keep apostrophes so that translate_word_with_style can handle contractions
                if in_contraction_suffix {
                    contraction_suffix.push(character);
                } else {
                    current_word.push(character);
                }
            } else if character == '\'' {
                in_contraction_suffix = true;
                contraction_suffix.push(character);
            } else {
                //The word ended, so translate the chararacters we've saved up until this point!
                in_word = false;
                translate_word_with_style_reuse_buffers (
                    current_word.as_str(),
                    suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper,
                    &mut pig_latin_string, &mut starting_consonants_buffer
                );

                //Push the contraction
                in_contraction_suffix = false;
                pig_latin_string.push_str(&contraction_suffix);
                contraction_suffix.truncate(0);//Faster than creating a new string

                //Append the symbol/whitespace we just got after the translated word
                pig_latin_string.push(character);
            }
        } else {//We are not currently in a word
            if character.is_alphabetic() {
                //If we see a letter, we are in a word, so save the character for now so we can translate the word later
                in_word = true;
                current_word.truncate(0);//Faster than creating a new string
                current_word.push(character);
            } else {
                //Otherwise copy symbols and whitespace as-is
                pig_latin_string.push(character);
            }
        }
    }
    //If we ended on a word, we translate it and push it to the end of the string
    if in_word {
        translate_word_with_style_reuse_buffers (
            current_word.as_str(),
            suffix_lower, special_case_suffix_lower, &suffix_upper, &special_case_suffix_upper,
            &mut pig_latin_string, &mut starting_consonants_buffer
        );
    }

    return pig_latin_string;
}


#[cfg(feature = "nightly-features")]
pub fn translate_with_style_generic <
    const SUFFIX_LOWER: &'static str, const SPECIAL_CASE_SUFFIX_LOWER: &'static str,
    const SUFFIX_UPPER: &'static str, const SPECIAL_CASE_SUFFIX_UPPER: &'static str
> (
    english: &str,
) -> String {
    if english.is_empty() {
        return String::new();
    }

    let mut pig_latin_string = String::with_capacity(english.len() * 2);//Plenty of headroom in case the words are very small or the suffixes are long
    let mut current_word = String::with_capacity(64);//Longer than all English words to avoid unneeded allocations (plus leaving room for leading and trailing extra characters)
    let mut contraction_suffix = String::with_capacity(64);
    let mut in_word: bool = false;
    let mut in_contraction_suffix: bool = false;

    //Buffers for improved performance (avoid repeated heap allocations)
    let mut starting_consonants_buffer = String::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

    for character in english.chars() {
        if in_word {
            if character.is_alphabetic() {
                //Save the character to translate once the word ends; we also keep apostrophes so that translate_word_with_style can handle contractions
                if in_contraction_suffix {
                    contraction_suffix.push(character);
                } else {
                    current_word.push(character);
                }
            } else if character == '\'' {
                in_contraction_suffix = true;
                contraction_suffix.push(character);
            } else {
                //The word ended, so translate the chararacters we've saved up until this point!
                in_word = false;
                translate_word_with_style_reuse_buffers (
                    current_word.as_str(),
                    SUFFIX_LOWER, SPECIAL_CASE_SUFFIX_LOWER, SUFFIX_UPPER, SPECIAL_CASE_SUFFIX_UPPER,
                    &mut pig_latin_string, &mut starting_consonants_buffer
                );

                //Push the contraction
                in_contraction_suffix = false;
                pig_latin_string.push_str(&contraction_suffix);
                contraction_suffix.truncate(0);//Faster than creating a new string

                //Append the symbol/whitespace we just got after the translated word
                pig_latin_string.push(character);
            }
        } else {//We are not currently in a word
            if character.is_alphabetic() {
                //If we see a letter, we are in a word, so save the character for now so we can translate the word later
                in_word = true;
                current_word.truncate(0);//Faster than creating a new string
                current_word.push(character);
            } else {
                //Otherwise copy symbols and whitespace as-is
                pig_latin_string.push(character);
            }
        }
    }
    //If we ended on a word, we translate it and push it to the end of the string
    if in_word {
        translate_word_with_style_reuse_buffers (
            current_word.as_str(),
            SUFFIX_LOWER, SPECIAL_CASE_SUFFIX_LOWER, SUFFIX_UPPER, SPECIAL_CASE_SUFFIX_UPPER,
            &mut pig_latin_string, &mut starting_consonants_buffer
        );
    }

    return pig_latin_string;
}

///Translates a multi-word string (including punctuation) into a custom-styled play language (Faster, but ASCII-only)!
///
///Faster than [`translate_with_style()`], but requires that the string only contains ASCII characters or else it may panic.
///
///Pass the string you wish to translate, the suffix you wish to have appended to most words, and the suffix
///you wish to have appended in various special-cases (such as when a word is only one letter or starts with a vowel).
///
///NOTE: The suffixes must be entirely lower-case or weird results may occur.
///
///# Examples
///
///```
///use anslatortray::translate_with_style_ascii;
///
///let suffix = "ancy";
///let special_case_suffix = "fancy";
///assert_eq!(translate_with_style_ascii("Hello world from the coolest Pig Latin translator!", suffix, special_case_suffix),
///    "Ellohancy orldwancy omfrancy ethancy oolestcancy Igpancy Atinlancy anslatortrancy!"
///);
///
///assert_eq!(translate_with_style_ascii("This library can translate any English text. It can even handle multiple sentences!", suffix, special_case_suffix),
///    "Isthancy ibrarylancy ancancy anslatetrancy anyfancy Englishfancy exttancy. Itfancy ancancy evenfancy andlehancy ultiplemancy entencessancy!"
///);
///
///assert_eq!(translate_with_style_ascii("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", suffix, special_case_suffix),
///    "Etl".to_string() + suffix + "'s ytr" + suffix + " omes" + suffix + " edge" + special_case_suffix + " asesc" + suffix + ". Atth" + suffix + " is" + special_case_suffix + " a" +
///    special_case_suffix + " ontractionc" + suffix + ", as" + special_case_suffix + " ellw" + suffix + " as" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix +
///    " erewh" + suffix + " eth" + suffix + " only" + special_case_suffix + " owelv" + suffix + " is" + special_case_suffix + " y" + special_case_suffix + ". Eatn" + suffix + ", all" +
///    special_case_suffix + " atth" + suffix + " orksw" + suffix + "!"
///);
///
///assert_eq!(translate_with_style_ascii("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", suffix, special_case_suffix),
///    "Atwh".to_string() + suffix + " if" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix + " ash" + suffix + " on" + suffix + " owelsv" + suffix + ", ikel" + suffix + " isth" + suffix + ": bcdfghjklmnpqrstvwxz" + suffix
///);
///
///assert_eq!(translate_with_style_ascii("Cool, so the heuristics make pretty good guesses with what they're fed!", suffix, special_case_suffix),
///    "Oolc".to_string() + suffix + ", os" + suffix + " eth" + suffix + " euristicsh" + suffix + " akem" + suffix + " ettypr" + suffix + " oodg" + suffix + " uessesg" + suffix + " ithw" + suffix + " atwh" + suffix + " eyth" + suffix + "'re edf" + suffix + "!"
///);
///
///assert_eq!(translate_with_style_ascii("Hello-world", suffix, special_case_suffix), "Elloh".to_string() + suffix + "-orldw" + suffix);
///
///assert_eq!(translate_with_style_ascii("Hyphens-are-difficult-aren't-they?", suffix, special_case_suffix),
///    "Yphensh".to_string() + suffix + "-are" + special_case_suffix + "-ifficultd" + suffix + "-aren" + special_case_suffix + "'t-eyth" + suffix + "?"
///);
///```
pub fn translate_with_style_ascii(english: &str, suffix_lower: &str, special_case_suffix_lower: &str) -> String {
    if english.is_empty() {
        return String::new();
    }

    //TODO switch to fully operating on u8 slices/arrays/Vecs internally (converting from a string, then to a string at the end) in anslatortray 0.5.0

    let mut pig_latin_string = Vec::<u8>::with_capacity(english.len() * 2);//Plenty of headroom in case the words are very small or the suffixes are long

    //Convert the suffix and special_case_suffix we were provided to uppercase for words that are capitalized
    let mut suffix_upper = String::with_capacity(suffix_lower.len());
    for letter in suffix_lower.chars() {
        suffix_upper.push(letter.to_ascii_uppercase());
    }
    let mut special_case_suffix_upper = String::with_capacity(special_case_suffix_lower.len());
    for letter in special_case_suffix_lower.chars() {
        special_case_suffix_upper.push(letter.to_ascii_uppercase());
    }

    //Flags used to remember if we're currently processing a word, contraction, contraction suffix or neither
    let mut in_word: bool = false;
    let mut in_contraction_suffix: bool = false;

    //Buffer for improved performance (avoid repeated heap allocations)
    let mut starting_consonants_buffer = Vec::<u8>::with_capacity(64);//Longer than basically all English words to avoid unneeded allocations, plus the fact that this isn't the whole word

    //Indexes for improved performance (avoid copying characters to use as the english_word argument for translate_word_with_style_reuse_buffers)
    //However, this assumes each character is one byte, so this only works with ASCII strings
    let mut slice_start_index: usize = 0;//Inclusive
    let mut slice_end_index: usize = 0;//Exclusive

    for character in english.chars() {
        if in_word {
            if in_contraction_suffix {
                if character.is_alphabetic() {
                    //We never translate the contraction suffix of a word, so just copy remaining letters as-is
                } else {
                    //The contraction ended, and so too does the word
                    //We still want to copy the non-letter to the output though
                    in_contraction_suffix = false;
                    in_word = false;
                }

                pig_latin_string.push(character as u8);//Copy the character
                slice_start_index += 1;//Keep the slice start index up to speed for later use
            } else {
                if character.is_alphabetic() {
                    //This character is part of the word, so increment the slice_end_index to include it in the slice
                    slice_end_index += 1;
                } else {
                    //The word or first part of the contraction ended, so translate the word we've identified up until this point!
                    let word_slice: &str = &english[slice_start_index..slice_end_index];
                    translate_word_with_style_reuse_buffers_ascii (
                        word_slice.as_bytes(),
                        suffix_lower.as_bytes(), special_case_suffix_lower.as_bytes(), &suffix_upper.as_bytes(), &special_case_suffix_upper.as_bytes(),
                        &mut pig_latin_string, &mut starting_consonants_buffer
                    );

                    //Bring the slice_start_index to the end since we've finished the word and need it ready for the next one
                    slice_start_index = slice_end_index + 1;

                    //Append the symbol/whitespace we just got after the translated word
                    pig_latin_string.push(character as u8);

                    //If the symbol/whitespace we just got is an apostrophe, then this is a contraction suffix
                    if character == '\'' {
                        in_contraction_suffix = true;
                    } else {
                        in_word = false;//This wasn't a contraction, so we're done with the word
                    }
                }
            }
        } else {
            if character.is_alphabetic() {
                //If we see a letter, we are in a word, so set the slice_end_index to the character after the slice_start_index
                in_word = true;
                slice_end_index = slice_start_index + 1;
            } else {
                //Otherwise copy symbols and whitespace as-is
                pig_latin_string.push(character as u8);
                slice_start_index += 1;
            }
        }
    }
    //If we ended on a word (but not on a contraction suffix), we translate it and push it to the end of the string
    if in_word && !in_contraction_suffix {
        let word_slice: &str = &english[slice_start_index..slice_end_index];
        translate_word_with_style_reuse_buffers_ascii (
            word_slice.as_bytes(),
            suffix_lower.as_bytes(), special_case_suffix_lower.as_bytes(), &suffix_upper.as_bytes(), &special_case_suffix_upper.as_bytes(),
            &mut pig_latin_string, &mut starting_consonants_buffer
        );
    }

    return std::str::from_utf8(pig_latin_string.as_slice()).unwrap().to_string();
}

///TODO description (same as translate_with_style_ascii, but exposes the raw byte strings/etc)
/*
pub fn translate_with_style_ascii_byte(english: &[u8], suffix_lower: &[u8], special_case_suffix_lower: &[u8], pig_latin_buffer_to_push_to: &mut Vec<u8>) {
    todo!();
}


//TODO this would be better but we can't have some const generics depending on others
#[cfg(feature = "nightly-features")]
pub fn translate_with_style_ascii_byte_generic <
    const SUFFIX_LEN: usize, const SUFFIX_LOWER: [u8],
    const SPECIAL_CASE_SUFFIX_LEN: usize, const SPECIAL_CASE_SUFFIX_LOWER: [u8]
> (
    english: &[u8],
    pig_latin_buffer_to_push_to: &mut Vec<u8>
) {
    todo!();
}
*/


//TODO also add an ascii_byte_generic that combines all 3
//TODO also add a regular generic version for UTF-8

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

            assert_eq!(translate_with_style("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ", suffix, special_case_suffix),
                "Atwh".to_string() + suffix + " if" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix + " ash" + suffix + " on" + suffix + " owelsv" + suffix + ", ikel" + suffix + " isth" + suffix + ": bcdfghjklmnpqrstvwxzBCDFGHJKLMNPQRSTVWXZ" + suffix
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

    #[test]
    fn test_translate_with_style_ascii() {
        for pair in SUFFIX_SPECIAL_CASE_SUFFIX_PAIRS {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_with_style_ascii("Hello world from the coolest Pig Latin translator!", suffix, special_case_suffix),
                "Elloh".to_string() + suffix + " orldw" + suffix + " omfr" + suffix + " eth" + suffix + " oolestc" + suffix + " Igp" + suffix + " Atinl" + suffix + " anslatortr" + suffix + "!"
            );

            assert_eq!(translate_with_style_ascii("This library can translate any English text. It can even handle multiple sentences!", suffix, special_case_suffix),
                "Isth".to_string() + suffix + " ibraryl" + suffix + " anc" + suffix + " anslatetr" + suffix + " any" + special_case_suffix + " English" + special_case_suffix + " extt" + suffix +
                ". It" + special_case_suffix + " anc" + suffix + " even" + special_case_suffix + " andleh" + suffix + " ultiplem" + suffix + " entencess" + suffix + "!"
            );
        }
    }

    #[test]
    fn test_translate_with_style_ascii_edgecases() {
        for pair in SUFFIX_SPECIAL_CASE_SUFFIX_PAIRS {
            let suffix = pair.0;
            let special_case_suffix = pair.1;

            assert_eq!(translate_with_style_ascii("Let's try some edge cases. That is a contraction, as well as a word where the only vowel is y. Neat, all that works!", suffix, special_case_suffix),
                "Etl".to_string() + suffix + "'s ytr" + suffix + " omes" + suffix + " edge" + special_case_suffix + " asesc" + suffix + ". Atth" + suffix + " is" + special_case_suffix + " a" +
                special_case_suffix + " ontractionc" + suffix + ", as" + special_case_suffix + " ellw" + suffix + " as" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix +
                " erewh" + suffix + " eth" + suffix + " only" + special_case_suffix + " owelv" + suffix + " is" + special_case_suffix + " y" + special_case_suffix + ". Eatn" + suffix + ", all" +
                special_case_suffix + " atth" + suffix + " orksw" + suffix + "!"
            );

            assert_eq!(translate_with_style_ascii("What if a word has no vowels, like this: bcdfghjklmnpqrstvwxz", suffix, special_case_suffix),
                "Atwh".to_string() + suffix + " if" + special_case_suffix + " a" + special_case_suffix + " ordw" + suffix + " ash" + suffix + " on" + suffix + " owelsv" + suffix + ", ikel" + suffix + " isth" + suffix + ": bcdfghjklmnpqrstvwxz" + suffix
            );

            assert_eq!(translate_with_style_ascii("Cool, so the heuristics make pretty good guesses with what they're fed!", suffix, special_case_suffix),
                "Oolc".to_string() + suffix + ", os" + suffix + " eth" + suffix + " euristicsh" + suffix + " akem" + suffix + " ettypr" + suffix + " oodg" + suffix + " uessesg" + suffix + " ithw" + suffix + " atwh" + suffix + " eyth" + suffix + "'re edf" + suffix + "!"
            );

            assert_eq!(translate_with_style_ascii("Hello-world", suffix, special_case_suffix), "Elloh".to_string() + suffix + "-orldw" + suffix);

            assert_eq!(translate_with_style_ascii("Hyphens-are-difficult-aren't-they?", suffix, special_case_suffix),
                "Yphensh".to_string() + suffix + "-are" + special_case_suffix + "-ifficultd" + suffix + "-aren" + special_case_suffix + "'t-eyth" + suffix + "?"
            );
        }
    }

    #[test]
    fn test_translate_with_style_ascii_uppercase() {
        for pair in SUFFIX_SPECIAL_CASE_SUFFIX_LOWER_UPPER_TUPLES {
            let suffix_lower = pair.0;
            let special_case_suffix_lower = pair.1;
            let suffix_upper = pair.2;
            let special_case_suffix_upper = pair.3;

            assert_eq!(translate_with_style_ascii("HELLO WORLD!", suffix_lower, special_case_suffix_lower),
                "ELLOH".to_string() + suffix_upper + " ORLDW" + suffix_upper + "!"
            );

            assert_eq!(translate_with_style_ascii("ISN't THIS COOL?", suffix_lower, special_case_suffix_lower),
                "ISN".to_string() + special_case_suffix_upper + "'t ISTH" + suffix_upper + " OOLC" + suffix_upper + "?"
            );

            assert_eq!(translate_with_style_ascii("What ABOUT a MIX?", suffix_lower, special_case_suffix_lower),
                "Atwh".to_string() + suffix_lower + " ABOUT" + special_case_suffix_upper + " a" + special_case_suffix_lower + " IXM" + suffix_upper + "?"
            );

            assert_eq!(translate_with_style_ascii("Luke, I am your father!", suffix_lower, special_case_suffix_lower),//We don't want to capitalize single-letter words
                "Ukel".to_string() + suffix_lower + ", I" + special_case_suffix_lower+ " am" + special_case_suffix_lower + " oury" + suffix_lower + " atherf" + suffix_lower + "!"
            );
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

    #[bench]
    fn ascii_way_project_description(b: &mut Bencher) {
        b.iter(|| -> String { return translate_ascii(PROJECT_DESCRIPTION); });
    }

    #[bench]
    fn ascii_yay_project_description(b: &mut Bencher) {
        b.iter(|| -> String { return translate_yay_ascii(PROJECT_DESCRIPTION); });
    }

    #[bench]
    fn ascii_hay_project_description(b: &mut Bencher) {
        b.iter(|| -> String { return translate_hay_ascii(PROJECT_DESCRIPTION); });
    }

    #[bench]
    fn ascii_ferb_project_description(b: &mut Bencher) {
        b.iter(|| -> String { return translate_ferb_ascii(PROJECT_DESCRIPTION); });
    }

    #[bench]
    fn ascii_way_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String { return translate_ascii(LOREM_IPSUM); });
    }

    #[bench]
    fn ascii_yay_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String { return translate_yay_ascii(LOREM_IPSUM); });
    }

    #[bench]
    fn ascii_hay_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String { return translate_hay_ascii(LOREM_IPSUM); });
    }

    #[bench]
    fn ascii_ferb_lorem_ipsum(b: &mut Bencher) {
        b.iter(|| -> String { return translate_ferb_ascii(LOREM_IPSUM); });
    }
}
