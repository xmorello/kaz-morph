use kaz_morph::{Analyzer, Features, Mood, Number, Person, Pos, Tense, Voice};

fn assert_verb(analyzer: &Analyzer, word: &str, lemma: &str, features: &Features) {
    let results = analyzer.analyze(word);
    assert!(
        results
            .iter()
            .any(|r| r.pos == Pos::Verb && r.lemma == lemma && r.features == *features),
        "expected {lemma} {features:?} for {word}, got {results:?}"
    );
}

#[test]
fn past_and_conditional_take_the_same_overt_agreement() {
    use Number::{Plural, Singular};
    use Person::{First, Second, SecondFormal};

    let cases = [
        ("бар", "барды", "барса", None, None),
        ("кел", "келдім", "келсем", Some(First), Some(Singular)),
        ("бар", "бардың", "барсаң", Some(Second), Some(Singular)),
        ("айт", "айттық", "айтсақ", Some(First), Some(Plural)),
        ("іш", "іштік", "ішсек", Some(First), Some(Plural)),
        ("бар", "бардыңдар", "барсаңдар", Some(Second), Some(Plural)),
        ("кел", "келдіңдер", "келсеңдер", Some(Second), Some(Plural)),
        (
            "бар",
            "бардыңыз",
            "барсаңыз",
            Some(SecondFormal),
            Some(Singular),
        ),
        (
            "кел",
            "келдіңіз",
            "келсеңіз",
            Some(SecondFormal),
            Some(Singular),
        ),
        (
            "айт",
            "айттыңыздар",
            "айтсаңыздар",
            Some(SecondFormal),
            Some(Plural),
        ),
        (
            "іш",
            "іштіңіздер",
            "ішсеңіздер",
            Some(SecondFormal),
            Some(Plural),
        ),
    ];
    for analyzer in [Analyzer::new(), Analyzer::full()] {
        for (lemma, past, conditional, person, number) in cases {
            for (word, tense, mood) in [
                (past, Some(Tense::PastDefinite), None),
                (conditional, None, Some(Mood::Conditional)),
            ] {
                let features = Features {
                    person,
                    number,
                    tense,
                    mood,
                    ..Features::default()
                };
                assert_verb(&analyzer, word, lemma, &features);
            }
        }
    }
}

#[test]
fn short_agreement_preserves_voice_and_negation() {
    let analyzer = Analyzer::new();
    for (word, tense, mood) in [
        ("жазылмадыңыз", Some(Tense::PastDefinite), None),
        ("жазылмасаңыз", None, Some(Mood::Conditional)),
    ] {
        let features = Features {
            person: Some(Person::SecondFormal),
            number: Some(Number::Singular),
            voice: Some(Voice::Passive),
            negation: true,
            tense,
            mood,
            ..Features::default()
        };
        assert_verb(&analyzer, word, "жаз", &features);
    }
}

#[test]
fn conditional_and_polite_endings_require_a_compatible_marker() {
    let analyzer = Analyzer::new();
    let conditional = (None, Some(Mood::Conditional));
    let past = (Some(Tense::PastDefinite), None);
    let narrative = (Some(Tense::PastNarrative), None);
    for (lemma, word, form) in [
        ("бар", "барсамын", conditional),
        ("кел", "келсесіңдер", conditional),
        ("бар", "барсең", conditional),
        ("бар", "барсак", conditional),
        ("кел", "келсеңыздар", conditional),
        ("бар", "бардыңіз", past),
        ("кел", "келдіңіздар", past),
        ("бар", "барғанңыз", narrative),
    ] {
        let results = analyzer.analyze(word);
        assert!(
            !results.iter().any(|r| {
                r.pos == Pos::Verb
                    && r.lemma == lemma
                    && (r.features.tense, r.features.mood) == form
            }),
            "unexpected verb reading of {word} as {lemma}: {results:?}"
        );
    }
}

#[test]
fn conditional_analysis_preserves_a_dictionary_homograph() {
    let analyzer = Analyzer::full();
    let results = analyzer.analyze("бөксе");
    assert!(results
        .iter()
        .any(|r| r.lemma == "бөксе" && r.pos == Pos::Noun));
    assert!(results.iter().any(|r| {
        r.lemma == "бөк" && r.pos == Pos::Verb && r.features.mood == Some(Mood::Conditional)
    }));
}

#[test]
fn conditional_restores_stem_voicing_only_before_a_vowel() {
    let analyzer = Analyzer::with_lexicon(kaz_morph::lexicon::Lexicon::from_tsv("жап\tVerb"));
    let conditional = Features {
        mood: Some(Mood::Conditional),
        ..Features::default()
    };
    assert_verb(&analyzer, "жапса", "жап", &conditional);
    let results = analyzer.analyze("жабса");
    assert!(!results
        .iter()
        .any(|r| r.lemma == "жап" && r.features.mood == Some(Mood::Conditional)));
    let passive = Features {
        voice: Some(Voice::Passive),
        ..conditional
    };
    assert_verb(&analyzer, "жабылса", "жап", &passive);
}
