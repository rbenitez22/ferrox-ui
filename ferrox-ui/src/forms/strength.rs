//! How strong a password is: its entropy in bits, from its length and the
//! kinds of character it uses, sorted into four levels.

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum StrengthLevel {
    /// Under 25 bits.
    VeryWeak,
    /// Under 40 bits.
    Weak,
    /// 40 bits up to the minimum asked for.
    Fair,
    /// At least the minimum asked for.
    Strong,
}

impl StrengthLevel {
    pub fn class(self) -> &'static str {
        match self {
            Self::VeryWeak => "password-strength--very-weak",
            Self::Weak     => "password-strength--weak",
            Self::Fair     => "password-strength--fair",
            Self::Strong   => "password-strength--strong",
        }
    }
}

/// A password's entropy and its level.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PasswordStrength {
    pub bits: f64,
    pub level: StrengthLevel,
}

/// The words the strength meter shows for each level (English by default).
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct StrengthLabels {
    pub very_weak: String,
    pub weak: String,
    pub fair: String,
    pub strong: String,
}

impl Default for StrengthLabels {
    fn default() -> Self {
        Self {
            very_weak: "Very weak".into(),
            weak: "Weak".into(),
            fair: "Fair".into(),
            strong: "Strong".into(),
        }
    }
}

impl StrengthLabels {
    pub fn new(very_weak: impl Into<String>, weak: impl Into<String>, fair: impl Into<String>, strong: impl Into<String>) -> Self {
        Self { very_weak: very_weak.into(), weak: weak.into(), fair: fair.into(), strong: strong.into() }
    }

    pub fn for_level(&self, level: StrengthLevel) -> &str {
        match level {
            StrengthLevel::VeryWeak => &self.very_weak,
            StrengthLevel::Weak     => &self.weak,
            StrengthLevel::Fair     => &self.fair,
            StrengthLevel::Strong   => &self.strong,
        }
    }
}

/// Entropy in bits: length × log2(the size of the character sets used).
/// Counts characters, not bytes: "é" is one symbol.
pub fn password_entropy(password: &str) -> f64 {
    let mut charset: u32 = 0;
    if password.chars().any(|c| c.is_ascii_lowercase()) { charset += 26; }
    if password.chars().any(|c| c.is_ascii_uppercase()) { charset += 26; }
    if password.chars().any(|c| c.is_ascii_digit()) { charset += 10; }
    if password.chars().any(|c| c.is_ascii_punctuation() || c.is_ascii_whitespace()) { charset += 33; }
    if !password.is_ascii() { charset += 64; }
    if charset == 0 {
        return 0.0;
    }
    password.chars().count() as f64 * f64::from(charset).log2()
}

/// The level for an entropy, given the minimum that counts as strong.
pub fn strength_level(bits: f64, min_bits: f64) -> StrengthLevel {
    if bits < 25.0 {
        StrengthLevel::VeryWeak
    } else if bits < 40.0 {
        StrengthLevel::Weak
    } else if bits < min_bits {
        StrengthLevel::Fair
    } else {
        StrengthLevel::Strong
    }
}

/// How strong `password` is, against `min_bits` for "strong".
pub fn password_strength(password: &str, min_bits: f64) -> PasswordStrength {
    let bits = password_entropy(password);
    PasswordStrength { bits, level: strength_level(bits, min_bits) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entropy_counts_characters_not_bytes() {
        // "é" is 2 bytes; it must count as one character.
        let one = password_entropy("é");
        let two = password_entropy("éé");
        assert!((two - 2.0 * one).abs() < 1e-9);
        assert!((one - 64f64.log2()).abs() < 1e-9);
    }

    #[test]
    fn entropy_grows_with_each_kind_of_character() {
        assert_eq!(password_entropy(""), 0.0);
        let lower = password_entropy("abcdefgh");
        let mixed = password_entropy("abcdEFGH");
        let digits = password_entropy("abcdEF12");
        let symbols = password_entropy("abcdEF1!");
        assert!(lower < mixed && mixed < digits && digits < symbols);
        assert!((lower - 8.0 * 26f64.log2()).abs() < 1e-9);
    }

    #[test]
    fn levels() {
        assert_eq!(strength_level(0.0, 80.0), StrengthLevel::VeryWeak);
        assert_eq!(strength_level(24.9, 80.0), StrengthLevel::VeryWeak);
        assert_eq!(strength_level(25.0, 80.0), StrengthLevel::Weak);
        assert_eq!(strength_level(40.0, 80.0), StrengthLevel::Fair);
        assert_eq!(strength_level(79.9, 80.0), StrengthLevel::Fair);
        assert_eq!(strength_level(80.0, 80.0), StrengthLevel::Strong);
    }

    #[test]
    fn labels() {
        let labels = StrengthLabels::default();
        assert_eq!(labels.for_level(StrengthLevel::Fair), "Fair");
        let ours = StrengthLabels::new("Muy débil", "Débil", "Aceptable", "Fuerte");
        assert_eq!(ours.for_level(StrengthLevel::Strong), "Fuerte");
    }
}
