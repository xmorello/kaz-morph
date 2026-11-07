# kaz-morph

Rule-based morphological analyzer for Kazakh: written in Rust, with bindings for Python and WASM.

[![Crates.io](https://img.shields.io/crates/v/kaz-morph)](https://crates.io/crates/kaz-morph)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)


## The problem

Kazakh is agglutinative. Grammar is encoded as chains of suffixes on a single root:

```
жүректерімізде
  жүрек  +  тер  +  іміз  +  де
  heart     PL      our      in
  -> "in our hearts"
```

For a computer, `жүрек` and `жүректерімізде` are unrelated strings, and a spellchecker can't tell whether a five-suffix chain follows vowel harmony or breaks it. On top of that, an NLP pipeline has no idea that `барды`, `бармады`, and `барғандар` are all forms of `бару`.

I guess without structural understanding of words, every downstream tool for Kazakh is either broken or faking it.

Existing tools either need a full FST toolchain (Apertium) or are unmaintained research code (KazNLP). kaz-morph is a library you actually install and use.

## Quick Start

```toml
[dependencies]
kaz-morph = "0.1.0"
```

```rust
use kaz_morph::Analyzer;

let analyzer = Analyzer::full();
let results = analyzer.analyze("мұғалімдерімізге");
// will return --> lemma: "мұғалім", pos: Noun, number: Plural, possession: P1Pl, case: Dative
```

## Features

- **Nouns**: 7 cases, plural, 6 possessive forms, consonant mutations (п→б, к→г, қ→ғ)
- **Verbs**: 7 tenses, negation, 4 voices (passive, causative, reflexive, collective), person/number
- **Adjectives**: bare and substantivized with full noun morphology
- **Adverbs**: lexicon lookup
- **Pronouns**: irregular declension table (мен→маған, сен→саған, ол→оған)
- **Phonology**: vowel harmony and consonant harmony validation
- **Lexicon**: 22,610 stems from Apertium-kaz

## Performance

| Benchmark | Time |
|---|---|
| Single word | ~1 µs |
| Full chain (мектептерімізде) | ~1.6 µs |
| Batch 10 words | ~14 µs |
| 10,000 words | ~16 ms |

## Python

```python
from kaz_morph import Analyzer

a = Analyzer()
results = a.analyze("жаным")
print(results[0].lemma)       # жан
print(results[0].possession)  # P1Sg
```

## WASM (Browser)

```javascript
import init, { Analyzer } from 'kaz-morph-wasm';

await init();
const analyzer = new Analyzer();
const results = analyzer.analyze("бармадым");
// --> lemma: "бар", tense: "PastDefinite", negation: true, person: "First"
```

## CLI

```bash
$ cargo run -p kaz-morph-cli -- мектептерімізде жаным барды

мектептерімізде
  1. lemma: мектеп  pos: Noun
     number: Plural  possession: P1Pl  case: Locative

жаным
  1. lemma: жан  pos: Noun
     possession: P1Sg

барды
  1. lemma: бар  pos: Verb
     tense: PastDefinite
```

## Architecture

```
kaz-morph/
├── core/              # Rust library
│   └── src/
│       ├── analyzer/  # Noun, verb, adjective analysis
│       ├── phonology  # Vowel/consonant harmony
│       ├── lexicon    # 22k word lexicon
│       ├── irregular  # Pronoun declension tables
│       └── types      # POS, Case, Tense, etc.
├── cli/               # Command-line tool
├── bindings/
│   ├── python/        # PyO3 bindings
│   └── wasm/          # wasm-bindgen bindings
└── data/              # Lexicon data
```

## Academic References

Based on research from:

- Kessikbayeva & Cicekli (2014) — Rule-based morphological analyzer
- Washington et al. (2014) — Finite-state transducers for Kipchak languages
- Yiner & Kurt (2021) — Two-level Kazakh morphology
- Makhambetov et al. (2015) — Data-driven analysis and disambiguation

## License

MIT
