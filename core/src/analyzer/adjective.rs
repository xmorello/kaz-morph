use crate::types::*;
use super::{suffixes::strip_plural, Analyzer};

impl Analyzer {
    pub(super) fn try_adjective(&self, word: &str, results: &mut Vec<MorphAnalysis>) {
        let case_options = self.strip_case(word);
        let mut after_case: Vec<(&str, Option<Case>)> = case_options;
        after_case.push((word, None));

        for (rest1, case) in &after_case {
            let poss_options = self.strip_possessive(rest1);
            let mut after_poss: Vec<(&str, Option<Possession>)> = poss_options;
            after_poss.push((rest1, None));

            for (rest2, poss) in &after_poss {
                let plural = strip_plural(rest2).map(|stem| (stem, Some(Number::Plural)));
                for (stem, number) in plural.into_iter().chain([(*rest2, None)]) {
                    if stem.is_empty() { continue; }
                    if let Some(entries) = self.lexicon.lookup(stem) {
                        for e in entries {
                            if e.pos != Pos::Adjective {
                                continue;
                            }
                            let features = Features {
                                number,
                                case: *case,
                                possession: *poss,
                                ..Default::default()
                            };
                            results.push(MorphAnalysis {
                                lemma: e.lemma.clone(),
                                pos: Pos::Adjective,
                                features,
                                score: 0.5,
                            });
                        }
                    }
                }
            }
        }
    }
}
