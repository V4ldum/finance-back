use anyhow::{Result, anyhow};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub(crate) struct AssetName(String);

impl AssetName {
    pub(crate) fn parse(name: String) -> Result<Self> {
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = name.chars().any(|c| forbidden_characters.contains(&c));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            return Err(anyhow!("Invalid asset name: '{}'", name));
        }

        Ok(Self(name))
    }
}

impl AsRef<str> for AssetName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::AssetName;
    use claims::{assert_err, assert_ok};
    use proptest::prelude::*;

    #[test]
    fn a_256_graphemes_long_name_is_valid() {
        let name = "ë".repeat(256);
        assert_ok!(AssetName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(AssetName::parse(name));
    }

    #[test]
    fn a_whitespace_only_name_is_rejected() {
        let name = " ".to_string();
        assert_err!(AssetName::parse(name));
    }

    #[test]
    fn an_empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(AssetName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for c in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = c.to_string();
            assert_err!(AssetName::parse(name));
        }
    }

    fn valid_name_strategy() -> impl Strategy<Value = String> {
        proptest::collection::vec(
            any::<char>().prop_filter("must be allowed char", |c| {
                !(['/', '(', ')', '"', '<', '>', '\\', '{', '}'].contains(c) || c.is_control())
            }),
            1..=256,
        )
        .prop_map(|chars| chars.into_iter().collect::<String>())
        .prop_filter("not whitespace-only", |s| !s.trim().is_empty())
    }

    proptest! {
        #[test]
        fn valid_names_are_parsed_successfully(name in valid_name_strategy()) {
            assert_ok!(AssetName::parse(name));
        }
    }
}
