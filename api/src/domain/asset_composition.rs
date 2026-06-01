use anyhow::{Result, anyhow};

#[derive(Debug)]
pub(crate) enum AssetComposition {
    Gold,
    Silver,
}

impl AssetComposition {
    pub(crate) fn parse(composition: String) -> Result<Self> {
        match composition.as_str() {
            "GOLD" => Ok(Self::Gold),
            "SILVER" => Ok(Self::Silver),
            _ => Err(anyhow!("composition can either be \"GOLD\" or \"SILVER\"")),
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
        assert_ok!(AssetComposition::parse("GOLD".to_string()));
    }

    #[test]
    fn silver_is_valid() {
        assert_ok!(AssetComposition::parse("SILVER".to_string()));
    }

    #[test]
    fn gold_lowercase_is_rejected() {
        assert_err!(AssetComposition::parse("gold".to_string()));
    }

    #[test]
    fn silver_lowercase_is_rejected() {
        assert_err!(AssetComposition::parse("silver".to_string()));
    }

    #[test]
    fn parse_then_as_ref_round_trips_is_valid() {
        let gold = AssetComposition::parse("GOLD".to_string()).unwrap();
        let silver = AssetComposition::parse("SILVER".to_string()).unwrap();
        assert_eq!(gold.as_ref(), "GOLD");
        assert_eq!(silver.as_ref(), "SILVER");
    }

    proptest! {
        #[test]
        fn invalid_composition_is_rejected(composition: String) {
            assert_err!(AssetComposition::parse(composition));
        }
    }
}
