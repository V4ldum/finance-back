use anyhow::Result;

#[derive(Debug, Copy, Clone)]
pub(crate) struct AssetUnitWeight(i64);

impl AssetUnitWeight {
    pub(crate) fn parse(unit_weight: i64) -> Result<Self, String> {
        if unit_weight < 0 {
            return Err("unit_weight must be >= 0".to_string());
        }

        Ok(Self(unit_weight))
    }
}

impl AsRef<i64> for AssetUnitWeight {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::AssetUnitWeight;
    use claims::{assert_err, assert_ok};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn unit_weight_smaller_than_0_are_rejected(unit_weight in i64::MIN..0_i64) {
            assert_err!(AssetUnitWeight::parse(unit_weight));
        }

        #[test]
        fn valid_unit_weights_are_parsed_successfully(unit_weight in 0_i64..=i64::MAX) {
            assert_ok!(AssetUnitWeight::parse(unit_weight));
        }
    }
}
