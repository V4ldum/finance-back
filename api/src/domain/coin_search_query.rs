use anyhow::{Result, anyhow};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub(crate) struct CoinSearchQuery(String);

impl CoinSearchQuery {
    pub(crate) fn parse(query: String) -> Result<Self> {
        let is_empty_or_whitespace = query.trim().is_empty();
        let is_too_long = query.graphemes(true).count() > 256;

        if is_empty_or_whitespace || is_too_long {
            return Err(anyhow!("Invalid search query: '{}'", query));
        }

        Ok(Self(query))
    }
}

impl AsRef<str> for CoinSearchQuery {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::CoinSearchQuery;
    use claims::{assert_err, assert_ok};
    use proptest::prelude::*;

    #[test]
    fn a_256_graphemes_long_query_is_valid() {
        let query = "ë".repeat(256);
        assert_ok!(CoinSearchQuery::parse(query));
    }

    #[test]
    fn a_query_longer_than_256_graphemes_is_rejected() {
        let query = "a".repeat(257);
        assert_err!(CoinSearchQuery::parse(query));
    }

    #[test]
    fn a_whitespace_only_query_is_rejected() {
        let query = " ".to_string();
        assert_err!(CoinSearchQuery::parse(query));
    }

    #[test]
    fn an_empty_string_is_rejected() {
        let query = "".to_string();
        assert_err!(CoinSearchQuery::parse(query));
    }

    fn valid_query_strategy() -> impl Strategy<Value = String> {
        proptest::collection::vec(any::<char>(), 1..=256)
            .prop_map(|chars| chars.into_iter().collect::<String>())
            .prop_filter("not whitespace-only", |s| !s.trim().is_empty())
    }

    proptest! {
        #[test]
        fn valid_queries_are_parsed_successfully(query in valid_query_strategy()) {
            assert_ok!(CoinSearchQuery::parse(query));
        }
    }
}
