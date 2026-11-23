use kaz_morph::{
    lexicon::Lexicon, Analyzer, Case, Features, Number, Person, Pos, Tense, VerbForm, Voice,
};

fn participle(number: Option<Number>, case: Option<Case>) -> Features {
    Features {
        number,
        case,
        verb_form: Some(VerbForm::PastParticiple),
        ..Features::default()
    }
}

fn assert_verb(analyzer: &Analyzer, word: &str, lemma: &str, features: &Features) {
    let readings = analyzer.analyze(word);
    assert!(
        readings
            .iter()
            .any(|r| r.pos == Pos::Verb && r.lemma == lemma && r.features == *features),
        "missing {lemma} {features:?} for {word}: {readings:?}"
    );
}

#[test]
fn past_participles_take_plural_then_locative() {
    let stems = Lexicon::from_tsv("бар\tVerb\nкел\tVerb\nайт\tVerb\nіш\tVerb");
    let cases = [
        ("бар", ["барған", "барғандар", "барғанда", "барғандарда"]),
        ("кел", ["келген", "келгендер", "келгенде", "келгендерде"]),
        ("айт", ["айтқан", "айтқандар", "айтқанда", "айтқандарда"]),
        ("іш", ["ішкен", "ішкендер", "ішкенде", "ішкендерде"]),
    ];
    let endings = [
        (None, None),
        (Some(Number::Plural), None),
        (None, Some(Case::Locative)),
        (Some(Number::Plural), Some(Case::Locative)),
    ];
    for analyzer in [Analyzer::with_lexicon(stems), Analyzer::full()] {
        for (lemma, forms) in cases {
            for (word, (number, case)) in forms.into_iter().zip(endings) {
                assert_verb(&analyzer, word, lemma, &participle(number, case));
            }
        }
    }
}

#[test]
fn negative_and_passive_participles_keep_their_features() {
    let analyzer = Analyzer::new();
    let negative = Features {
        negation: true,
        ..participle(Some(Number::Plural), None)
    };
    assert_verb(&analyzer, "бармағандар", "бар", &negative);
    let passive = Features {
        voice: Some(Voice::Passive),
        negation: true,
        ..participle(None, Some(Case::Locative))
    };
    assert_verb(&analyzer, "жазылмағанда", "жаз", &passive);
}

#[test]
fn marker_agrees_with_the_verb_stem() {
    let analyzer = Analyzer::with_lexicon(Lexicon::from_tsv("жи\tVerb\nки\tVerb\nшанд\tVerb"));
    for (lemma, valid, invalid) in [
        ("жи", "жиған", "жиген"),
        ("ки", "киген", "киған"),
        ("шанд", "шандқан", "шандған"),
    ] {
        assert_verb(&analyzer, valid, lemma, &participle(None, None));
        assert!(
            !analyzer.analyze(invalid).iter().any(|r| {
                r.lemma == lemma && r.features.verb_form == Some(VerbForm::PastParticiple)
            }),
            "{invalid}"
        );
    }
}

#[test]
fn preserves_finite_and_dictionary_readings() {
    let analyzer = Analyzer::full();
    let finite = Features {
        tense: Some(Tense::PastNarrative),
        ..Features::default()
    };
    assert_verb(&analyzer, "барған", "бар", &finite);
    let first_person = Features {
        number: Some(Number::Singular),
        person: Some(Person::First),
        ..finite
    };
    assert_verb(&analyzer, "барғанмын", "бар", &first_person);
    let homograph = analyzer.analyze("қалған");
    assert!(homograph
        .iter()
        .any(|r| r.lemma == "қалған" && r.pos == Pos::Adjective));
    assert_verb(&analyzer, "қалған", "қал", &participle(None, None));
    assert!(analyzer.analyze("келгенде").iter().any(|r| {
        r.lemma == "келген" && r.pos == Pos::Noun && r.features.case == Some(Case::Locative)
    }));
}

#[test]
fn rejects_incompatible_participle_readings() {
    let analyzer = Analyzer::full();
    for (lemma, word) in [
        ("бар", "барқан"),
        ("кел", "келған"),
        ("айт", "айтған"),
        ("іш", "ішген"),
        ("бар", "барғанлар"),
        ("кел", "келгендар"),
        ("бар", "барғанде"),
        ("кел", "келгенте"),
        ("бар", "барғандарде"),
        ("бар", "барғандадар"),
        ("бар", "барғанмын"),
    ] {
        let readings = analyzer.analyze(word);
        assert!(
            !readings.iter().any(|r| {
                r.pos == Pos::Verb
                    && r.lemma == lemma
                    && r.features.verb_form == Some(VerbForm::PastParticiple)
            }),
            "unexpected participle of {lemma} for {word}: {readings:?}"
        );
    }
}
