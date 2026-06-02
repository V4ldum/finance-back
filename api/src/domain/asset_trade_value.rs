use anyhow::{Result, anyhow};

#[derive(Debug)]
pub(crate) enum AssetTradeValue {
    Gold,
    Silver,
    SP500,
}
impl AssetTradeValue {
    pub(crate) fn parse(trade_value: String) -> Result<Self> {
        match trade_value.as_str() {
            "gold" => Ok(Self::Gold),
            "silver" => Ok(Self::Silver),
            "sp500" => Ok(Self::SP500),
            _ => Err(anyhow!("trade_value can either be \"gold\" or \"silver\" or \"sp500\"")),
        }
    }
}

impl AsRef<str> for AssetTradeValue {
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
    use crate::domain::AssetTradeValue;
    use claims::{assert_err, assert_ok};
    use proptest::prelude::*;

    #[test]
    fn gold_is_valid() {
        assert_ok!(AssetTradeValue::parse("gold".to_string()));
    }

    #[test]
    fn silver_is_valid() {
        assert_ok!(AssetTradeValue::parse("silver".to_string()));
    }

    #[test]
    fn sp500_is_valid() {
        assert_ok!(AssetTradeValue::parse("sp500".to_string()));
    }

    #[test]
    fn gold_as_ref_returns_the_correct_string() {
        assert_eq!(AssetTradeValue::Gold.as_ref(), "Gold");
    }

    proptest! {
        #[test]
        fn invalid_composition_is_rejected(composition: String) {
            assert_err!(AssetTradeValue::parse(composition));
        }
    }
}
