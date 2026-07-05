use anyhow::Result;

#[derive(Debug)]
pub(crate) enum AssetComposition {
    Gold,
    Silver,
}

impl AssetComposition {
    pub(crate) fn parse(composition: &str) -> Result<Self, String> {
        match composition {
            "GOLD" => Ok(Self::Gold),
            "SILVER" => Ok(Self::Silver),
            _ => Err("composition can either be \"GOLD\" or \"SILVER\"".to_string()),
        }
    }
}

impl AsRef<str> for AssetComposition {
    fn as_ref(&self) -> &str {
        match self {
            Self::Gold => "GOLD",
            Self::Silver => "SILVER",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::AssetComposition;
    use claims::{assert_err, assert_ok};
    use proptest::prelude::*;

    #[test]
    fn gold_is_valid() {
        assert_ok!(AssetComposition::parse("GOLD"));
    }

    #[test]
    fn silver_is_valid() {
        assert_ok!(AssetComposition::parse("SILVER"));
    }

    #[test]
    fn gold_lowercase_is_rejected() {
        assert_err!(AssetComposition::parse("gold"));
    }

    #[test]
    fn silver_lowercase_is_rejected() {
        assert_err!(AssetComposition::parse("silver"));
    }

    #[test]
    fn parse_then_as_ref_round_trips_is_valid() {
        let gold = AssetComposition::parse("GOLD").unwrap();
        let silver = AssetComposition::parse("SILVER").unwrap();
        assert_eq!(gold.as_ref(), "GOLD");
        assert_eq!(silver.as_ref(), "SILVER");
    }

    proptest! {
        #[test]
        fn invalid_composition_is_rejected(composition: String) {
            assert_err!(AssetComposition::parse(&composition));
        }
    }
}
