use kaz_morph::{lexicon::Lexicon, Analyzer, Case, Features, Pos, VerbForm, Voice};

fn verbal_noun(case: Option<Case>) -> Features {
    Features {
        case,
        verb_form: Some(VerbForm::VerbalNoun),
        ..Features::default()
    }
}

fn has_reading(a: &Analyzer, word: &str, lemma: &str, pos: Pos, features: &Features) -> bool {
    a.analyze(word)
        .iter()
        .any(|r| r.lemma == lemma && r.pos == pos && &r.features == features)
}

#[test]
fn suffix_and_dative_follow_the_verb_stem() {
    let cases = [
        ("бар", "бару", "баруға"),
        ("кел", "келу", "келуге"),
        ("де", "деу", "деуге"),
        ("оқы", "оқу", "оқуға"),
        ("бекі", "беку", "бекуге"),
        ("байы", "баю", "баюға"),
        ("қой", "қою", "қоюға"),
        ("сый", "сыю", "сыюға"),
        ("сүй", "сүю", "сүюге"),
        ("ки", "кию", "киюге"),
        ("қи", "қию", "қиюға"),
        ("жап", "жабу", "жабуға"),
        ("жау", "жауу", "жаууға"),
        ("қырқ", "қырқу", "қырқуға"),
        ("қорық", "қорқу", "қорқуға"),
        ("үрік", "үрку", "үркуге"),
        ("тауыс", "таусу", "таусуға"),
    ];
    let mut lexicon = Lexicon::new();
    for (lemma, _, _) in cases {
        lexicon.insert(lemma, Pos::Verb);
    }
    for analyzer in [Analyzer::with_lexicon(lexicon), Analyzer::full()] {
        for (lemma, bare, dative) in cases {
            for (word, case) in [(bare, None), (dative, Some(Case::Dative))] {
                assert!(
                    has_reading(&analyzer, word, lemma, Pos::Verb, &verbal_noun(case)),
                    "missing verbal noun of {lemma} for {word}"
                );
            }
        }
    }
}

#[test]
fn homographs_keep_their_lexemes() {
    let full = Analyzer::full();
    for (word, case) in [("бару", None), ("баруға", Some(Case::Dative))] {
        let noun = Features {
            case,
            ..Features::default()
        };
        assert!(has_reading(&full, word, "бару", Pos::Noun, &noun), "{word}");
        assert!(
            has_reading(&full, word, "бар", Pos::Verb, &verbal_noun(case)),
            "{word}"
        );
    }
    let only_tan = Analyzer::with_lexicon(Lexicon::from_tsv("тан\tVerb"));
    for (word, case) in [("тану", None), ("тануға", Some(Case::Dative))] {
        let features = verbal_noun(case);
        for lemma in ["тан", "таны"] {
            assert!(
                has_reading(&full, word, lemma, Pos::Verb, &features),
                "{word} from {lemma}"
            );
        }
        assert!(
            has_reading(&only_tan, word, "тан", Pos::Verb, &features),
            "{word}"
        );
        assert!(
            !has_reading(&only_tan, word, "таны", Pos::Verb, &features),
            "{word}"
        );
    }
    for lemma in ["жу", "жуы"] {
        assert!(
            has_reading(&full, "жуу", lemma, Pos::Verb, &verbal_noun(None)),
            "{lemma}"
        );
    }
}

#[test]
fn vowel_drop_requires_the_restored_lexeme() {
    let a = Analyzer::with_lexicon(Lexicon::from_tsv("тау\tVerb"));
    for (word, case) in [("таусу", None), ("таусуға", Some(Case::Dative))] {
        let plain = verbal_noun(case);
        let collective = Features {
            voice: Some(Voice::Collective),
            ..verbal_noun(case)
        };
        assert!(!has_reading(&a, word, "тауыс", Pos::Verb, &plain), "{word}");
        assert!(
            !has_reading(&a, word, "тау", Pos::Verb, &collective),
            "{word}"
        );
    }
}

#[test]
fn negation_and_voice_precede_nominal_inflection() {
    let a = Analyzer::new();
    for (lemma, bare, dative, voice, negation) in [
        ("бар", "бармау", "бармауға", None, true),
        ("тан", "танбау", "танбауға", None, true),
        ("таны", "танымау", "танымауға", None, true),
        ("жаз", "жазылу", "жазылуға", Some(Voice::Passive), false),
        ("жаз", "жазылмау", "жазылмауға", Some(Voice::Passive), true),
        ("кел", "келісу", "келісуге", Some(Voice::Collective), false),
        ("жап", "жабылу", "жабылуға", Some(Voice::Passive), false),
    ] {
        for (word, case) in [(bare, None), (dative, Some(Case::Dative))] {
            let expected = Features {
                voice,
                negation,
                ..verbal_noun(case)
            };
            assert!(has_reading(&a, word, lemma, Pos::Verb, &expected), "{word}");
        }
    }
}

#[test]
fn excludes_crossed_stems_and_wrong_allomorphs() {
    let a = Analyzer::full();
    for (lemma, word) in [
        ("жап", "жауу"),
        ("жап", "жаууға"),
        ("жау", "жабу"),
        ("жау", "жабуға"),
        ("тан", "танымау"),
        ("таны", "танбау"),
        ("жап", "жапу"),
        ("жап", "жапылу"),
        ("кел", "келсу"),
        ("оқы", "оқыу"),
        ("сүй", "сүйу"),
        ("ки", "киу"),
        ("бар", "баруге"),
        ("кел", "келуға"),
        ("кел", "келуке"),
        ("сүй", "сүюға"),
        ("сый", "сыюге"),
    ] {
        let results = a.analyze(word);
        assert!(
            !results.iter().any(|r| r.lemma == lemma
                && r.pos == Pos::Verb
                && r.features.verb_form == Some(VerbForm::VerbalNoun)),
            "unexpected verbal noun of {lemma} for {word}: {results:?}"
        );
    }
}
