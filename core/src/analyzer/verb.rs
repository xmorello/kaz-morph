use super::Analyzer;
use crate::{phonology, types::*};

impl Analyzer {
    pub(super) fn try_verb(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
        self.try_short_agreement(word, results);

        let long_endings = self.strip_long_person(word);
        for (marked, person, number) in long_endings.into_iter().chain([(word, None, None)]) {
            let mut tenses = self.strip_past_narrative(marked);
            tenses.extend(self.strip_past_transitional(marked));
            tenses.extend(self.strip_future_indefinite(marked));
            tenses.extend(self.strip_future_goal(marked));
            tenses.extend(self.strip_present_definite(marked));
            tenses.extend(self.strip_present_progressive(marked));
            for (stem, tense) in tenses {
                let features = Features {
                    number,
                    person,
                    tense,
                    ..Features::default()
                };
                self.try_verb_stem(word, stem, &features, results);
            }
        }
    }

    fn try_short_agreement(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
        use Number::{Plural, Singular};
        use Person::{First, Second, SecondFormal};

        // In each pair the back-vowel spelling comes first. The unmarked third
        // person leaves both agreement fields empty, as in the existing past API.
        let endings = [
            (["", ""], None),
            (["м", "м"], Some((First, Singular))),
            (["ң", "ң"], Some((Second, Singular))),
            (["ңыз", "ңіз"], Some((SecondFormal, Singular))),
            (["қ", "к"], Some((First, Plural))),
            (["ңдар", "ңдер"], Some((Second, Plural))),
            (["ңыздар", "ңіздер"], Some((SecondFormal, Plural))),
        ];
        let markers = [
            (["ды", "ді"], Some(Tense::PastDefinite), None),
            (["ты", "ті"], Some(Tense::PastDefinite), None),
            (["са", "се"], None, Some(Mood::Conditional)),
        ];

        for (ending_forms, agreement) in endings {
            for (harmony, ending) in ending_forms.into_iter().enumerate() {
                let Some(before_ending) = word.strip_suffix(ending) else {
                    continue;
                };
                for (marker_forms, tense, mood) in markers {
                    let marker = marker_forms[harmony];
                    let Some(stem) = before_ending.strip_suffix(marker) else {
                        continue;
                    };
                    if !stem.is_empty() && phonology::is_valid_suffix(stem, marker) {
                        let (person, number) = agreement.unzip();
                        let features = Features {
                            number,
                            person,
                            tense,
                            mood,
                            ..Features::default()
                        };
                        self.try_verb_stem(word, stem, &features, results);
                    }
                }
            }
        }
    }

    fn try_verb_stem(
        &self,
        word: &str,
        inflected: &str,
        features: &Features,
        results: &mut Vec<MorphAnalysis>,
    ) {
        let negatives = self.strip_negation(inflected);
        for (voiced, negation) in negatives.into_iter().chain([(inflected, false)]) {
            let voices = self.strip_voice(voiced);
            for (stem, voice) in voices.into_iter().chain([(voiced, None)]) {
                // Stem-final voicing can only be reversed before a vowel, e.g.
                // жап + ыл in жабылса. Before -са the bare stem is жап.
                let variants = if word[stem.len()..].starts_with(phonology::is_vowel) {
                    self.stem_variants(stem)
                } else {
                    vec![stem.to_owned()]
                };
                for variant in variants {
                    if let Some(entries) = self.lexicon.lookup(&variant) {
                        for entry in entries.iter().filter(|e| e.pos == Pos::Verb) {
                            results.push(MorphAnalysis {
                                lemma: entry.lemma.clone(),
                                pos: Pos::Verb,
                                features: Features {
                                    voice,
                                    negation,
                                    ..features.clone()
                                },
                                score: 0.6,
                            });
                        }
                    }
                }
            }
        }
    }
}
