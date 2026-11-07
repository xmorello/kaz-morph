/// Vowel harmony group.
///
/// Жуан (back) vowels take suffixes with а/ы.
/// Жіңішке (front) vowels take suffixes with е/і.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VowelGroup {
    /// Жуан дауыстылар: а, о, ұ, ы, у
    Back,
    /// Жіңішке дауыстылар: ә, ө, ү, е, і, и, э
    Front,
}

/// Classification of consonants by voicing.
///
/// Based on Kessikbayeva Table 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsonantClass {
    /// Үнді (sonorous): л, р, й, у, м, н, ң
    Sonorous,
    /// Ұяң (voiced): з, ж, б, в, г, д
    Voiced,
    /// Қатаң (voiceless): п, ф, қ, к, т, с, ш, ч, х, ц, щ
    Voiceless,
}

/// What the stem ends with — drives suffix allomorph selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StemEnd {
    Vowel,
    SonorousOrVoiced,
    Voiceless,
}

const BACK_VOWELS: &[char] = &['а', 'о', 'ұ', 'ы', 'у'];
const FRONT_VOWELS: &[char] = &['ә', 'ө', 'ү', 'е', 'і', 'и', 'э'];

const SONOROUS: &[char] = &['л', 'р', 'й', 'у', 'м', 'н', 'ң'];
const VOICED: &[char] = &['з', 'ж', 'б', 'в', 'г', 'д'];
const VOICELESS: &[char] = &['п', 'ф', 'қ', 'к', 'т', 'с', 'ш', 'ч', 'х', 'ц', 'щ'];

/// Determine the vowel harmony group from the last vowel in a stem.
pub fn vowel_group(stem: &str) -> Option<VowelGroup> {
    stem.chars().rev().find_map(classify_vowel)
}

/// Classify a single character as Back or Front vowel.
pub fn classify_vowel(c: char) -> Option<VowelGroup> {
    if BACK_VOWELS.contains(&c) {
        Some(VowelGroup::Back)
    } else if FRONT_VOWELS.contains(&c) {
        Some(VowelGroup::Front)
    } else {
        None
    }
}

/// Classify a consonant by voicing.
pub fn classify_consonant(c: char) -> Option<ConsonantClass> {
    if SONOROUS.contains(&c) {
        Some(ConsonantClass::Sonorous)
    } else if VOICED.contains(&c) {
        Some(ConsonantClass::Voiced)
    } else if VOICELESS.contains(&c) {
        Some(ConsonantClass::Voiceless)
    } else {
        None
    }
}

/// Classify how a stem ends — vowel, sonorous/voiced, or voiceless.
pub fn stem_end(stem: &str) -> StemEnd {
    let Some(last) = stem.chars().last() else {
        return StemEnd::Vowel;
    };

    if classify_vowel(last).is_some() {
        StemEnd::Vowel
    } else {
        match classify_consonant(last) {
            Some(ConsonantClass::Voiceless) => StemEnd::Voiceless,
            Some(_) | None => StemEnd::SonorousOrVoiced,
        }
    }
}

/// Consonant voicing mutation: voiceless → voiced.
///   п → б, к → г, қ → ғ
pub fn devoiced_to_voiced(c: char) -> Option<char> {
    match c {
        'п' => Some('б'),
        'к' => Some('г'),
        'қ' => Some('ғ'),
        _ => None,
    }
}

/// Reverse mutation: recover original stem consonant.
pub fn voiced_to_devoiced(c: char) -> Option<char> {
    match c {
        'б' => Some('п'),
        'г' => Some('к'),
        'ғ' => Some('қ'),
        _ => None,
    }
}

/// Check if a character is a vowel (either group).
pub fn is_vowel(c: char) -> bool {
    classify_vowel(c).is_some()
}

/// Check if a character is a consonant.
pub fn is_consonant(c: char) -> bool {
    classify_consonant(c).is_some()
}

/// Validate that a suffix variant is compatible with a stem.
///
/// Checks vowel harmony and consonant harmony.
pub fn is_valid_suffix(stem: &str, suffix: &str) -> bool {
    if stem.is_empty() || suffix.is_empty() {
        return true;
    }

    let is_instrumental = matches!(
        suffix,
        "мен" | "бен" | "пен" | "менен" | "бенен" | "пенен"
    );

    // Check vowel harmony
    if !is_instrumental {
        if let Some(stem_group) = vowel_group(stem) {
            if let Some(suffix_group) = vowel_group(suffix) {
                if stem_group != suffix_group {
                    return false;
                }
            }
        }
    }

    // Instrumental has its own consonant selection rules, skip check
    if is_instrumental {
        return true;
    }

    // Check consonant harmony
    let s_end = stem_end(stem);
    let Some(suffix_first) = suffix.chars().next() else {
        return true;
    };

    match suffix_first {
        'д' => s_end != StemEnd::Voiceless,
        'т' => s_end == StemEnd::Voiceless,
        'н' => s_end == StemEnd::Vowel || s_end == StemEnd::SonorousOrVoiced,
        'л' => s_end == StemEnd::Vowel,
        'ғ' | 'г' => s_end != StemEnd::Voiceless,
        'қ' | 'к' => s_end == StemEnd::Voiceless,
        'м' => s_end == StemEnd::Vowel || s_end == StemEnd::SonorousOrVoiced,
        'б' => s_end == StemEnd::SonorousOrVoiced,
        'п' => s_end == StemEnd::Voiceless,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_vowel_harmony() {
        assert_eq!(vowel_group("қала"), Some(VowelGroup::Back));
        assert_eq!(vowel_group("балық"), Some(VowelGroup::Back));
        assert_eq!(vowel_group("тау"), Some(VowelGroup::Back));
    }

    #[test]
    fn front_vowel_harmony() {
        assert_eq!(vowel_group("мектеп"), Some(VowelGroup::Front));
        assert_eq!(vowel_group("үстел"), Some(VowelGroup::Front));
        assert_eq!(vowel_group("жүрек"), Some(VowelGroup::Front));
    }

    #[test]
    fn loan_word_last_syllable_wins() {
        assert_eq!(vowel_group("мұғалім"), Some(VowelGroup::Front));
    }

    #[test]
    fn stem_end_classification() {
        assert_eq!(stem_end("мектеп"), StemEnd::Voiceless);
        assert_eq!(stem_end("қала"), StemEnd::Vowel);
        assert_eq!(stem_end("үстел"), StemEnd::SonorousOrVoiced);
        assert_eq!(stem_end("балық"), StemEnd::Voiceless);
    }

    #[test]
    fn consonant_mutations() {
        assert_eq!(devoiced_to_voiced('п'), Some('б'));
        assert_eq!(devoiced_to_voiced('қ'), Some('ғ'));
        assert_eq!(devoiced_to_voiced('к'), Some('г'));
        assert_eq!(devoiced_to_voiced('л'), None);
    }

    #[test]
    fn reverse_mutations() {
        assert_eq!(voiced_to_devoiced('б'), Some('п'));
        assert_eq!(voiced_to_devoiced('ғ'), Some('қ'));
        assert_eq!(voiced_to_devoiced('г'), Some('к'));
        assert_eq!(voiced_to_devoiced('д'), None);
    }

    #[test]
    fn vowel_consonant_checks() {
        assert!(is_vowel('а'));
        assert!(is_vowel('ө'));
        assert!(!is_vowel('т'));

        assert!(is_consonant('т'));
        assert!(is_consonant('л'));
        assert!(!is_consonant('а'));
    }

    #[test]
    fn valid_suffix_vowel_harmony() {
        assert!(is_valid_suffix("мектеп", "те"));
        assert!(is_valid_suffix("қала", "да"));

        assert!(!is_valid_suffix("қала", "де"));
        assert!(!is_valid_suffix("қала", "те"));

        assert!(is_valid_suffix("мектеп", "тер"));
        assert!(!is_valid_suffix("мектеп", "тар"));
    }

    #[test]
    fn valid_suffix_consonant_harmony() {
        assert!(is_valid_suffix("мектеп", "те"));
        assert!(!is_valid_suffix("мектеп", "де"));

        assert!(is_valid_suffix("қала", "да"));
        assert!(!is_valid_suffix("қала", "та"));

        assert!(is_valid_suffix("көз", "де"));
        assert!(!is_valid_suffix("көз", "те"));
    }

    #[test]
    fn instrumental_skips_harmony() {
        assert!(is_valid_suffix("қала", "мен"));
        assert!(is_valid_suffix("мектеп", "пен"));
    }
}
