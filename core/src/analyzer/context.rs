use super::Analyzer;
use crate::types::{MorphAnalysis, Pos, VerbForm};

impl Analyzer {
    /// Analyze one tokenized sentence, returning one list of readings per input token.
    /// Pass punctuation as separate tokens to preserve boundaries. The first reading
    /// is preferred in context. Each reading retains its original word-level score.
    ///
    /// ```
    /// use kaz_morph::Analyzer;
    ///
    /// let analyzer = Analyzer::full();
    /// let readings = analyzer.analyze_tokens(&["жазып", "алды", "."]);
    /// assert_eq!(readings[1][0].lemma, "ал");
    /// assert!(readings[2].is_empty());
    /// ```
    pub fn analyze_tokens(&self, tokens: &[&str]) -> Vec<Vec<MorphAnalysis>> {
        let mut sentence = Vec::with_capacity(tokens.len());
        let mut follows_converb = false;
        for token in tokens {
            let mut readings = self.analyze(token);
            if follows_converb {
                prefer_finite_auxiliary(&mut readings);
            }
            follows_converb = readings
                .iter()
                .any(|r| r.pos == Pos::Verb && r.features.verb_form == Some(VerbForm::Converb));
            sentence.push(readings);
        }
        sentence
    }
}

fn prefer_finite_auxiliary(readings: &mut [MorphAnalysis]) {
    let Some(index) = readings.iter().position(|r| {
        r.pos == Pos::Verb
            && r.features.verb_form.is_none()
            && (r.features.tense.is_some() || r.features.mood.is_some())
            && matches!(
                r.lemma.as_str(),
                "ал" | "бер" | "бол" | "жат" | "жүр" | "кел" | "отыр" | "тұр" | "қал"
            )
    }) else {
        return;
    };
    if index == 0 {
        return;
    }
    // In алып келген, келген can still be a participle.
    let lemma = &readings[index].lemma;
    if readings
        .iter()
        .any(|r| r.pos == Pos::Verb && &r.lemma == lemma && r.features.verb_form.is_some())
    {
        return;
    }
    readings[..=index].rotate_right(1);
}
