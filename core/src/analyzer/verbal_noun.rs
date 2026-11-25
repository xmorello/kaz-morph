use super::suffixes::{stems_before_vowel, verb_harmony, VOWEL_DROP};
use super::Analyzer;
use crate::phonology::VowelGroup;
use crate::types::{Case, Features, MorphAnalysis, VerbForm};

impl Analyzer {
    pub(super) fn try_verbal_noun(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
        let (form, dative) = [("ға", VowelGroup::Back), ("ге", VowelGroup::Front)]
            .into_iter()
            .find_map(|(ending, harmony)| {
                word.strip_suffix(ending).map(|base| (base, Some(harmony)))
            })
            .unwrap_or((word, None));
        let features = Features {
            case: dative.map(|_| Case::Dative),
            verb_form: Some(VerbForm::VerbalNoun),
            ..Features::default()
        };
        let agrees = |stem: &str| dative.is_none() || verb_harmony(stem) == dative;
        let add_lexeme = |lemma: &str, out: &mut Vec<MorphAnalysis>| {
            if agrees(lemma) {
                self.push_verb_reading(lemma, &features, out);
            }
        };
        if let Some(base) = form.strip_suffix('у') {
            if base.is_empty() || base.ends_with(['й', 'и', 'ы', 'і']) {
                return;
            }
            for stem in stems_before_vowel(base) {
                if agrees(&stem) && !VOWEL_DROP.iter().any(|&(_, lemma)| lemma == stem) {
                    self.try_verb_stem(&stem, false, &features, results);
                }
            }
            for &(short, lemma) in VOWEL_DROP {
                if base == short {
                    add_lexeme(lemma, results);
                }
            }
            // The final high vowel disappears in оқу (оқы); тану allows both тан and таны.
            add_lexeme(&format!("{base}ы"), results);
            add_lexeme(&format!("{base}і"), results);
        } else if let Some(base) = form.strip_suffix('ю') {
            if base.is_empty() {
                return;
            }
            if base.ends_with('и') {
                add_lexeme(base, results); // кию, қию
            }
            let glide = format!("{base}й");
            add_lexeme(&glide, results); // қою, сүю
            add_lexeme(&format!("{glide}ы"), results); // баю from байы
            add_lexeme(&format!("{glide}і"), results);
        }
    }
}
