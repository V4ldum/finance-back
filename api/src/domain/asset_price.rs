use anyhow::Result;

#[derive(Debug)]
pub(crate) enum AssetPrice {
    Gold,
    Silver,
    SP500,
}
impl AssetPrice {
    pub(crate) fn parse(price: &str) -> Result<Self, String> {
        match price {
            "gold" => Ok(Self::Gold),
            "silver" => Ok(Self::Silver),
            "sp500" => Ok(Self::SP500),
            _ => Err("price can either be \"gold\" or \"silver\" or \"sp500\"".to_string()),
        }
    }
}

impl AsRef<str> for AssetPrice {
    fn as_ref(&self) -> &str {
        match self {
            Self::Gold => "Gold",
            Self::Silver => "Silver",
            Self::SP500 => "SP500",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::AssetPrice;
    use claims::{assert_err, assert_ok};
    use proptest::prelude::*;

    #[test]
    fn gold_is_valid() {
        assert_ok!(AssetPrice::parse("gold"));
    }

    #[test]
    fn silver_is_valid() {
        assert_ok!(AssetPrice::parse("silver"));
    }

    #[test]
    fn sp500_is_valid() {
        assert_ok!(AssetPrice::parse("sp500"));
    }

    #[test]
    fn gold_as_ref_returns_the_correct_string() {
        assert_eq!(AssetPrice::Gold.as_ref(), "Gold");
    }

    proptest! {
        #[test]
        fn invalid_composition_is_rejected(composition: String) {
            assert_err!(AssetPrice::parse(&composition));
        }
    }
}
