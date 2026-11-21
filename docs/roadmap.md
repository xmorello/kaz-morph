# Roadmap

kaz-morph will analyze Kazakh words in its Rust core using explicit rules for which suffixes may follow each form. The Python and JavaScript packages will expose the same readings. We will adapt grammatical mechanisms from Apertium one subsystem at a time and compare the results offline with a pinned build of [apertium-kaz](https://github.com/apertium/apertium-kaz/tree/95c6dd0d8536ee69a7058634b03a3e82100b6b6e).

## First PR — short personal endings

The definite past and the conditional take the same short set of personal endings. Model it once and route the two constructions through it. For example, `бардым` and `барсам` both mark first person with `-м`. The polite singular ending occurs in both `бардыңыз` and `барсаңыз`. The reading must still distinguish the definite past from the conditional. See the upstream [agreement class](https://github.com/apertium/apertium-kaz/blob/95c6dd0d8536ee69a7058634b03a3e82100b6b6e/apertium-kaz.kaz.lexc#L778-L792), [past marker](https://github.com/apertium/apertium-kaz/blob/95c6dd0d8536ee69a7058634b03a3e82100b6b6e/apertium-kaz.kaz.lexc#L902-L906) and [conditional markers](https://github.com/apertium/apertium-kaz/blob/95c6dd0d8536ee69a7058634b03a3e82100b6b6e/apertium-kaz.kaz.lexc#L969-L972).

Check the paradigm on front- and back-vowel stems, positive and negative forms, and polite singular and plural endings. Check the specific forbidden readings for `бардымын` and `барсамын`, and retain the long ending in `барғанмын`. Run the cases with a controlled verb lexicon and with the full dictionary.

On the fixed corpus sample below, this change raises verb lemma candidate recall from 814 to 839 of 1,549 tokens (54.2%). It adds a matching lemma on 25 tokens and retains all 814 previous matches.

## Present–future agreement

The finite reading combines -а/-е/-й with a personal ending, including third-person -ды/-ді. For example, `оқиды` recovers `оқы`, and `білмейсіздер` recovers `біл` with negation and polite plural agreement. The current scope is full written forms in isolation. Compare the [Apertium aorist endings](https://github.com/apertium/apertium-kaz/blob/95c6dd0d8536ee69a7058634b03a3e82100b6b6e/apertium-kaz.kaz.lexc#L792-L807) and the [orthographic analysis of сүйемін](https://emle.kz/kz/rule?id=46).

On the same 1,549-token sample, verb lemma candidate recall increases from 839 to 948 (61.2%). All 839 previous matches are retained.

## Next steps

1. Add participles, converbs and forms in `-у` with their permitted nominal suffixes. Record independently reviewed positive cases and forbidden readings for each construction.
2. Preserve the stem variants, continuation classes and phonological annotations needed for the next rules. For reused entries or tables, record the source file, revision, material used, changes and applicable distribution terms. The pinned upstream distribution includes a [GPLv3 license](https://github.com/apertium/apertium-kaz/blob/95c6dd0d8536ee69a7058634b03a3e82100b6b6e/COPYING).
3. Stabilize the common output schema and default dictionary. Test the installed Python wheel and npm tarball against the same examples. Ship a preview for the documented coverage when the data and packages are ready.
4. Expand derivation and ranking against held-out data. Add versioned transliteration schemes with explicit treatment of ambiguous conversions.

## Measuring progress

Track whether the correct verb lemma is present, whether the required features are present, whether a forbidden reading is returned, and whether the first result is correct. Report the data, denominator, dictionary and revision with each metric.

For reference, on 1,549 single-word Cyrillic verb tokens from the [UD Kazakh KTB test split](https://github.com/UniversalDependencies/UD_Kazakh-KTB/blob/c850e5334a50befaf35a0907df766c4de89f68a1/kk_ktb-ud-test.conllu), Rust revision `8aed94a` found the annotated lemma among its verb candidates 814 times (52.6%). Apertium revision `95c6dd0d` found it 1,534 times (99.0%). This comparison scored lemma candidates only. The [corpus documentation](https://universaldependencies.org/treebanks/kk_ktb/index.html) describes its connection to Apertium. Use an independent Kazakh sample when judging improvements to linguistic accuracy.
