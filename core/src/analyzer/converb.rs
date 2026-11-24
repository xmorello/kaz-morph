use super::suffixes::verb_harmony;
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
    ("қорқып", "қорық"),
    ("үркіп", "үрік"),
    ("таусып", "тауыс"),
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

        // таны-п and тан-ып have different stems but the same surface.
        if before_p.chars().next_back().is_some_and(vowel_before_p) {
            self.try_verb_voice(before_p, false, &features, results);
        }
        let (stem, harmony) = if let Some(stem) = before_p.strip_suffix('ы') {
            (stem, VowelGroup::Back)
        } else if let Some(stem) = before_p.strip_suffix('і') {
            (stem, VowelGroup::Front)
        } else {
            return;
        };
        let mut chars = stem.chars().rev();
        let Some(last) = chars.next() else { return };
        if vowel_before_p(last) || verb_harmony(stem) != Some(harmony) {
            return;
        }
        let after_vowel = chars.next().is_some_and(phonology::is_vowel);
        let stop_voices = after_vowel && phonology::devoiced_to_voiced(last).is_some();
        if !stop_voices && !has_alternation(stem) {
            self.try_verb_voice(stem, false, &features, results);
        }
        if after_vowel {
            if let Some(stop) = phonology::voiced_to_devoiced(last) {
                let mut restored = stem.to_owned();
                restored.pop();
                restored.push(stop);
                if !has_alternation(&restored) {
                    self.push_verb_reading(&restored, &features, results);
                }
            }
        }
    }
}

fn vowel_before_p(c: char) -> bool {
    c == 'я' || (phonology::is_vowel(c) && !matches!(c, 'и' | 'у'))
}

fn has_alternation(stem: &str) -> bool {
    ALTERNATIONS.iter().any(|&(_, lemma)| lemma == stem)
}
