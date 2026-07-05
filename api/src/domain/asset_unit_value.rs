use anyhow::Result;

#[derive(Debug, Copy, Clone)]
pub(crate) struct AssetUnitValue(i64);

impl AssetUnitValue {
    pub(crate) fn parse(unit_value: i64) -> Result<Self, String> {
        if unit_value < 0 {
            return Err("unit_value must be >= 0".to_string());
        }

        Ok(Self(unit_value))
    }
}

impl AsRef<i64> for AssetUnitValue {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::AssetUnitValue;
    use claims::{assert_err, assert_ok};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn unit_value_smaller_than_0_are_rejected(unit_value in i64::MIN..0_i64) {
            assert_err!(AssetUnitValue::parse(unit_value));
        }

        #[test]
        fn valid_unit_values_are_parsed_successfully(unit_value in 0_i64..=i64::MAX) {
            assert_ok!(AssetUnitValue::parse(unit_value));
        }
    }
}
