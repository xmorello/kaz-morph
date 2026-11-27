use kaz_morph::{lexicon::Lexicon, Analyzer, Pos, Tense, VerbForm};

#[test]
fn promotes_the_auxiliary_after_an_ambiguous_converb() {
    let analyzer = Analyzer::full();
    let left = analyzer.analyze("Алып");
    assert_ne!(left[0].features.verb_form, Some(VerbForm::Converb));
    assert!(left
        .iter()
        .any(|r| r.features.verb_form == Some(VerbForm::Converb)));

    let mut expected = analyzer.analyze("қалдық");
    let index = expected
        .iter()
        .position(|r| r.lemma == "қал" && r.features.tense == Some(Tense::PastDefinite))
        .unwrap();
    assert!(index >= 2);
    let auxiliary = expected.remove(index);
    expected.insert(0, auxiliary);

    assert_eq!(
        analyzer.analyze_tokens(&["Алып", "қалдық"]),
        [left, expected]
    );
}

#[test]
fn context_stops_at_the_next_token_or_sentence_boundary() {
    let analyzer = Analyzer::full();
    let ordinary = analyzer.analyze("алды");
    assert_eq!(ordinary[0].pos, Pos::Noun);
    let phrase = analyzer.analyze_tokens(&["жазып", "алды"]);
    assert_eq!(phrase[1][0].lemma, "ал");
    assert_eq!(phrase[1][0].features.tense, Some(Tense::PastDefinite));

    for between in [",", "xyz", "негізін"] {
        let tokens = ["жазып", between, "алды"];
        assert_eq!(
            analyzer.analyze_tokens(&tokens),
            tokens.map(|t| analyzer.analyze(t))
        );
    }
    assert_eq!(
        analyzer.analyze_tokens(&["жазып"]),
        [analyzer.analyze("жазып")]
    );
    assert_eq!(analyzer.analyze_tokens(&["алды"]), [ordinary]);
    assert!(analyzer.analyze_tokens(&[]).is_empty());
}

#[test]
fn preserves_competing_participles_and_verbs_outside_the_auxiliary_list() {
    let analyzer = Analyzer::full();
    for tokens in [
        ["сақталып", "қалған"],
        ["айдалып", "келген"],
        ["аңдап", "басты"],
    ] {
        assert_eq!(
            analyzer.analyze_tokens(&tokens),
            tokens.map(|t| analyzer.analyze(t))
        );
    }
}

#[test]
fn context_uses_the_supplied_lexicon() {
    let analyzer = Analyzer::with_lexicon(Lexicon::from_tsv("жаз\tVerb\nалды\tNoun"));
    let tokens = ["жазып", "алды"];
    assert_eq!(
        analyzer.analyze_tokens(&tokens),
        tokens.map(|t| analyzer.analyze(t))
    );
}
