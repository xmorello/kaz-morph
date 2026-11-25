use super::suffixes::{stems_before_vowel, verb_harmony, VOWEL_DROP};
use super::Analyzer;
use crate::phonology::{self, VowelGroup};
use crate::types::{Features, MorphAnalysis, VerbForm};

// These lexemes change their stem specifically before -(ы/і)п.
const ALTERNATIONS: &[(&str, &str)] = &[
    ("жауып", "жап"),
    ("тауып", "тап"),
    ("теуіп", "теп"),
    ("кеуіп", "кеп"),
    ("сеуіп", "сеп"),
    ("шауып", "шап"),
    ("қауып", "қап"),
];

// Additional spellings coexist with the regular forms.
const VARIANTS: &[(&str, &str)] = &[
    ("ап", "ал"),
    ("боп", "бол"),
    ("кеп", "кел"),
    ("қап", "қал"),
    ("сап", "сал"),
    ("қып", "қыл"),
    ("ғып", "қыл"),
    ("қайтіп", "қайт"),
];

impl Analyzer {
    pub(super) fn try_converb(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
        let Some(before_p) = word.strip_suffix('п') else {
            return;
        };
        let features = Features {
            verb_form: Some(VerbForm::Converb),
            ..Features::default()
        };
        for &(surface, lemma) in ALTERNATIONS.iter().chain(VARIANTS) {
            if word == surface {
                self.push_verb_reading(lemma, &features, results);
            }
        }

        let long = before_p
            .strip_suffix('ы')
            .map(|s| (s, VowelGroup::Back))
            .or_else(|| before_p.strip_suffix('і').map(|s| (s, VowelGroup::Front)));
        let long_stem = long
            .filter(|(stem, harmony)| verb_harmony(stem) == Some(*harmony))
            .map(|(stem, _)| stem);
        if let Some(stem) = long_stem {
            for &(short, lemma) in VOWEL_DROP {
                if stem == short {
                    self.push_verb_reading(lemma, &features, results);
                }
            }
        }
        // таны-п and тан-ып have different stems but the same surface.
        if before_p.chars().next_back().is_some_and(vowel_before_p) {
            self.try_verb_voice(before_p, false, &features, results);
        }
        let Some(stem) = long_stem else { return };
        let Some(last) = stem.chars().next_back() else {
            return;
        };
        if vowel_before_p(last) {
            return;
        }
        for base in stems_before_vowel(stem) {
            if !has_alternation(&base) {
                self.try_verb_voice(&base, false, &features, results);
            }
        }
    }
}

fn vowel_before_p(c: char) -> bool {
    c == 'я' || (phonology::is_vowel(c) && !matches!(c, 'и' | 'у'))
}

fn has_alternation(stem: &str) -> bool {
    ALTERNATIONS
        .iter()
        .chain(VOWEL_DROP)
        .any(|&(_, lemma)| lemma == stem)
}
