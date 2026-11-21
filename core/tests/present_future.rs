use kaz_morph::{lexicon::Lexicon, Analyzer, Features, Number, Person, Pos, Tense, Voice};

fn present(person: Person, number: Option<Number>) -> Features {
    Features {
        number,
        person: Some(person),
        tense: Some(Tense::PresentDefinite),
        ..Features::default()
    }
}

fn assert_reading(analyzer: &Analyzer, word: &str, lemma: &str, expected: &Features) {
    let readings = analyzer.analyze(word);
    assert!(
        readings
            .iter()
            .any(|r| r.lemma == lemma && r.pos == Pos::Verb && r.features == *expected),
        "missing {lemma} {expected:?} for {word}, got {readings:?}"
    );
}

#[test]
fn positive_and_negative_personal_endings() {
    use Number::{Plural, Singular};
    use Person::{First, Second, SecondFormal, Third};

    let cases = [
        ("барамын", "білмеймін", First, Some(Singular)),
        ("барасың", "білмейсің", Second, Some(Singular)),
        ("барасыз", "білмейсіз", SecondFormal, Some(Singular)),
        ("барамыз", "білмейміз", First, Some(Plural)),
        ("барасыңдар", "білмейсіңдер", Second, Some(Plural)),
        ("барасыздар", "білмейсіздер", SecondFormal, Some(Plural)),
        ("барады", "білмейді", Third, None),
    ];
    for analyzer in [Analyzer::new(), Analyzer::full()] {
        for (positive, negative, person, number) in cases {
            let mut expected = present(person, number);
            assert_reading(&analyzer, positive, "бар", &expected);
            expected.negation = true;
            assert_reading(&analyzer, negative, "біл", &expected);
        }
    }
}

#[test]
fn stem_spelling_and_negative_allomorphs() {
    let analyzer = Analyzer::full();
    let cases = [
        ("оқы", "оқимын", "оқымаймын"),
        ("бекі", "бекимін", "бекімеймін"),
        ("же", "жеймін", "жемеймін"),
        ("сөйле", "сөйлеймін", "сөйлемеймін"),
        ("қой", "қоямын", "қоймаймын"),
        ("сүй", "сүйемін", "сүймеймін"),
        ("жап", "жабамын", "жаппаймын"),
        ("іш", "ішемін", "ішпеймін"),
        ("жүз", "жүземін", "жүзбеймін"),
        ("сен", "сенемін", "сенбеймін"),
        ("жұм", "жұмамын", "жұмбаймын"),
        ("ки", "киемін", "кимеймін"),
        ("қи", "қиямын", "қимаймын"),
        ("жу", "жуамын", "жумаймын"),
        ("қырқ", "қырқамын", "қырқпаймын"),
    ];
    for (lemma, positive, negative) in cases {
        let mut expected = present(Person::First, Some(Number::Singular));
        assert_reading(&analyzer, positive, lemma, &expected);
        expected.negation = true;
        assert_reading(&analyzer, negative, lemma, &expected);
    }
    assert_reading(&analyzer, "оқиды", "оқы", &present(Person::Third, None));
    assert_reading(&analyzer, "жейді", "же", &present(Person::Third, None));
}

#[test]
fn passive_and_negation_compose_with_agreement() {
    let analyzer = Analyzer::with_lexicon(Lexicon::from_tsv("жап\tVerb"));
    let expected = Features {
        voice: Some(Voice::Passive),
        negation: true,
        ..present(Person::SecondFormal, Some(Number::Plural))
    };
    assert_reading(&analyzer, "жабылмайсыздар", "жап", &expected);
}

#[test]
fn forbids_mismatched_finite_readings() {
    let analyzer = Analyzer::full();
    for (lemma, word) in [
        ("бар", "бара"),
        ("кел", "келе"),
        ("біл", "білмей"),
        ("бар", "бараң"),
        ("бар", "барамін"),
        ("бар", "барасыздер"),
        ("бар", "барадымын"),
        ("кел", "келедіңіз"),
        ("оқы", "оқыйды"),
        ("қой", "қойамын"),
        ("сүй", "сүемін"),
        ("жап", "жапамын"),
        ("бар", "барбаймын"),
        ("сен", "сенмеймін"),
        ("айт", "айтмаймын"),
    ] {
        let readings = analyzer.analyze(word);
        assert!(
            !readings.iter().any(|r| {
                r.pos == Pos::Verb
                    && r.lemma == lemma
                    && r.features.tense == Some(Tense::PresentDefinite)
            }),
            "unexpected present-future reading of {word} as {lemma}: {readings:?}"
        );
    }
}
