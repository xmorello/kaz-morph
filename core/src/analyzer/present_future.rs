use super::suffixes::{long_person_endings, verb_harmony};
use super::Analyzer;
use crate::phonology::{self, VowelGroup};
use crate::types::{Features, MorphAnalysis, Person, Tense};

impl Analyzer {
    pub(super) fn try_present_future(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
        use VowelGroup::{Back, Front};

        let endings = long_person_endings()
            .map(|(ending, harmony, person, number)| (ending, harmony, person, Some(number)))
            .chain([
                ("ды", Back, Person::Third, None),
                ("ді", Front, Person::Third, None),
            ]);
        for (ending, harmony, person, number) in endings {
            let Some(before_ending) = word.strip_suffix(ending) else {
                continue;
            };
            let Some((offset, marker)) = before_ending.char_indices().next_back() else {
                continue;
            };
            let base = &before_ending[..offset];
            let Some(last) = base.chars().next_back() else {
                continue;
            };
            let vowel_final =
                last == 'я' || (phonology::is_vowel(last) && !matches!(last, 'и' | 'у'));
            let features = Features {
                number,
                person: Some(person),
                tense: Some(Tense::PresentDefinite),
                ..Features::default()
            };
            let mut analyze = |stem: &str, vowel_follows: bool| {
                if verb_harmony(stem) == Some(harmony) {
                    self.try_verb_stem(stem, vowel_follows, &features, results);
                }
            };
            match marker {
                'й' if vowel_final && !matches!(last, 'ы' | 'і') => analyze(base, false),
                // оқы + й → оқи; бекі + й → беки.
                'и' => {
                    let high_vowel = if harmony == Back { 'ы' } else { 'і' };
                    analyze(&format!("{base}{high_vowel}"), false);
                }
                // қой + а → қоя; қи + а → қия. Front-vowel сүй retains й in сүйе.
                'я' if harmony == Back => {
                    if last == 'и' {
                        analyze(base, false);
                    }
                    analyze(&format!("{base}й"), false);
                }
                'а' | 'е' => {
                    let low_vowel = if harmony == Back { 'а' } else { 'е' };
                    if marker != low_vowel
                        || vowel_final
                        || (harmony == Back && matches!(last, 'й' | 'и'))
                    {
                        continue;
                    }
                    // жап → жаба, but clusters keep the stop, as in қырқ → қырқа.
                    let after_vowel = base.chars().rev().nth(1).is_some_and(phonology::is_vowel);
                    if !after_vowel || phonology::devoiced_to_voiced(last).is_none() {
                        analyze(base, after_vowel);
                    }
                }
                _ => {}
            }
        }
    }
}
