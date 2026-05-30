use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct AssetPossessed(i64);

impl AssetPossessed {
    pub fn parse(possessed: i64) -> Result<Self> {
        if possessed < 1 {
            return Err(anyhow!("possessed must be >= 1"));
        }

        Ok(Self(possessed))
    }
}

impl AsRef<i64> for AssetPossessed {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use crate::domain::AssetPossessed;
    use claims::{assert_err, assert_ok};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn possessed_smaller_than_1_are_rejected(possessed in i64::MIN..=0i64) {
            assert_err!(AssetPossessed::parse(possessed));
        }

        #[test]
        fn valid_possessed_are_parsed_successfully(possessed in 1i64..=i64::MAX) {
            assert_ok!(AssetPossessed::parse(possessed));
        }

    }
}
