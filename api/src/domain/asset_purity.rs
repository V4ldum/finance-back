use anyhow::{Result, anyhow};

#[derive(Debug, Copy, Clone)]
pub(crate) struct AssetPurity(i64);

impl AssetPurity {
    pub(crate) fn parse(purity: i64) -> Result<Self> {
        if !(1..=9999).contains(&purity) {
            return Err(anyhow!("purity must be between 1 and 9999"));
        }

        Ok(Self(purity))
    }
}

impl AsRef<i64> for AssetPurity {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::AssetPurity;
    use claims::{assert_err, assert_ok};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn purity_smaller_than_1_are_rejected(purity in i64::MIN..=0_i64) {
            assert_err!(AssetPurity::parse(purity));
        }

        #[test]
        fn purity_larger_than_9999_are_rejected(purity in 10000_i64..=i64::MAX) {
            assert_err!(AssetPurity::parse(purity));
        }

        #[test]
        fn valid_purity(purity in 1_i64..=9999_i64) {
            assert_ok!(AssetPurity::parse(purity));
        }

    }
}
