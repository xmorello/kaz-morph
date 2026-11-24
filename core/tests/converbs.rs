use kaz_morph::{
    lexicon::Lexicon, Analyzer, Features, Number, Person, Pos, Tense, VerbForm, Voice,
};

fn verbs(lemmas: &[&str]) -> Analyzer {
    let mut lex = Lexicon::new();
    for lemma in lemmas {
        lex.insert(lemma, Pos::Verb);
    }
    Analyzer::with_lexicon(lex)
}

fn converb() -> Features {
    Features {
        verb_form: Some(VerbForm::Converb),
        ..Features::default()
    }
}

fn assert_reading(a: &Analyzer, word: &str, lemma: &str, pos: Pos, features: &Features) {
    let readings = a.analyze(word);
    assert!(
        readings
            .iter()
            .any(|r| r.lemma == lemma && r.pos == pos && r.features == *features),
        "missing {lemma}/{pos:?}/{features:?} for {word}: {readings:?}"
    );
}

fn assert_converb(a: &Analyzer, word: &str, lemma: &str) {
    assert_reading(a, word, lemma, Pos::Verb, &converb());
}

#[test]
fn regular_converbs_follow_the_stem() {
    let cases = [
        ("барып", "бар"),
        ("келіп", "кел"),
        ("айтып", "айт"),
        ("істеп", "істе"),
        ("оқып", "оқы"),
        ("жеп", "же"),
        ("деп", "де"),
        ("киіп", "ки"),
        ("жиып", "жи"),
        ("жуып", "жу"),
        ("жымиып", "жыми"),
        ("шығып", "шық"),
        ("төгіп", "төк"),
    ];
    for a in [verbs(&cases.map(|(_, lemma)| lemma)), Analyzer::full()] {
        for (word, lemma) in cases {
            assert_converb(&a, word, lemma);
        }
    }
    assert_converb(&Analyzer::new(), "деп", "де");
}

#[test]
fn stem_alternations_depend_on_the_lexeme() {
    let cases = [
        ("жауып", "жап"),
        ("тауып", "тап"),
        ("теуіп", "теп"),
        ("сеуіп", "сеп"),
        ("кеуіп", "кеп"),
        ("қауып", "қап"),
        ("шауып", "шап"),
        ("өбіп", "өп"),
        ("обып", "оп"),
        ("серпіп", "серп"),
        ("қорқып", "қорық"),
        ("үркіп", "үрік"),
        ("таусып", "тауыс"),
        ("қайтіп", "қайт"),
        ("қайтып", "қайт"),
    ];
    let a = verbs(&cases.map(|(_, lemma)| lemma));
    for (word, lemma) in cases {
        assert_converb(&a, word, lemma);
    }
}

#[test]
fn attested_contractions_require_a_lexical_verb() {
    let full = Analyzer::full();
    let empty = verbs(&[]);
    for (word, lemma) in [
        ("ап", "ал"),
        ("боп", "бол"),
        ("кеп", "кел"),
        ("қап", "қал"),
        ("сап", "сал"),
        ("қып", "қыл"),
        ("ғып", "қыл"),
    ] {
        assert_converb(&full, word, lemma);
        assert!(
            !empty
                .analyze(word)
                .iter()
                .any(|r| r.features.verb_form == Some(VerbForm::Converb)),
            "{word}"
        );
    }
}

#[test]
fn qap_preserves_four_distinct_readings() {
    for a in [Analyzer::new(), Analyzer::full()] {
        let readings = a.analyze("Қап");
        assert_eq!(readings.len(), 4, "{readings:?}");
        for (lemma, pos, features) in [
            ("қап", Pos::Interjection, Features::default()),
            ("қап", Pos::Noun, Features::default()),
            ("қап", Pos::Verb, Features::default()),
            ("қал", Pos::Verb, converb()),
        ] {
            assert!(
                readings
                    .iter()
                    .any(|r| r.lemma == lemma && r.pos == pos && r.features == features),
                "missing {lemma}/{pos:?} in {readings:?}"
            );
        }
    }
}

#[test]
fn competing_stems_and_lexicalized_adverbs_survive() {
    let a = Analyzer::full();
    for (word, lemma) in [
        ("жауып", "жау"),
        ("жауып", "жап"),
        ("танып", "тан"),
        ("танып", "таны"),
        ("сөйтіп", "сөйт"),
    ] {
        assert_converb(&a, word, lemma);
    }
    assert_reading(&a, "сөйтіп", "сөйтіп", Pos::Adverb, &Features::default());
}

#[test]
fn voice_and_finite_readings_remain_distinct() {
    let a = Analyzer::new();
    let passive = Features {
        voice: Some(Voice::Passive),
        ..converb()
    };
    assert_reading(&a, "жазылып", "жаз", Pos::Verb, &passive);
    assert_reading(&a, "делініп", "де", Pos::Verb, &passive);
    assert!(!a
        .analyze("деліп")
        .iter()
        .any(|r| r.lemma == "де" && r.features == passive));
    assert!(!a
        .analyze("барып")
        .iter()
        .any(|r| r.lemma == "бар" && r.features.tense == Some(Tense::PastTransitional)));
    let finite = Features {
        number: Some(Number::Singular),
        person: Some(Person::Second),
        tense: Some(Tense::PastTransitional),
        ..Features::default()
    };
    assert_reading(&a, "барыпсың", "бар", Pos::Verb, &finite);
    assert!(!a
        .analyze("барыпсың")
        .iter()
        .any(|r| r.features.verb_form == Some(VerbForm::Converb)));
}

#[test]
fn rejects_unlicensed_converb_readings() {
    let cases = [
        ("айтп", "айт"),
        ("барп", "бар"),
        ("сөйлеіп", "сөйле"),
        ("оқыып", "оқы"),
        ("келып", "кел"),
        ("тапып", "тап"),
        ("табып", "тап"),
        ("жабып", "жап"),
        ("сепіп", "сеп"),
        ("тебіп", "теп"),
        ("күп", "күл"),
        ("біп", "біл"),
        ("өп", "өл"),
        ("бармап", "бар"),
    ];
    let a = verbs(&cases.map(|(_, lemma)| lemma));
    for (word, lemma) in cases {
        let readings = a.analyze(word);
        assert!(
            !readings.iter().any(|r| r.pos == Pos::Verb
                && r.lemma == lemma
                && r.features.verb_form == Some(VerbForm::Converb)),
            "unexpected {lemma} for {word}: {readings:?}"
        );
    }
}
