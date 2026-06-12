use std::fmt::Formatter;
use std::ops::Deref;

use serde::de::MapAccess;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub(crate) struct CoinQuery {
    pub(crate) title: String,
    pub(crate) weight: f64,
    pub(crate) size: f64,
    pub(crate) thickness: Option<f64>,
    #[serde(deserialize_with = "deserialize_date_to_string")]
    pub(crate) min_year: String,
    #[serde(deserialize_with = "deserialize_date_to_string")]
    pub(crate) max_year: String,
    pub(crate) composition: CoinQueryComposition,
    pub(crate) obverse: Option<CoinQuerySide>,
    pub(crate) reverse: Option<CoinQuerySide>,
    pub(crate) edge: Option<CoinQuerySide>,
    pub(crate) watermark: Option<CoinQuerySide>,
}

#[derive(Debug)]
pub(crate) struct CoinQueryComposition {
    //pub text: String,
    pub(crate) composition: String,
    pub(crate) purity: i32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct CoinQuerySide {
    pub(crate) description: Option<String>,
    pub(crate) lettering: Option<String>,
    pub(crate) picture: Option<String>,
    pub(crate) thumbnail: Option<String>,
    pub(crate) picture_copyright: Option<String>,
}

fn deserialize_date_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let date = u32::deserialize(deserializer)?;

    Ok(date.to_string())
}

impl<'de> Deserialize<'de> for CoinQueryComposition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CoinQueryComposition;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a JSON map containing a key called \"text\" with data following the format \"precious_metal purity‰\"")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut text: Option<String> = None;

                // Search for the correct key : "text"
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "text" => {
                            text = Some(map.next_value()?);
                        }
                        _ => {
                            map.next_value::<()>()?;
                        }
                    }
                }

                let Some(text) = text else {
                    return Err(serde::de::Error::missing_field("text"));
                };

                // Split around whitespaces. First part should be the metal (Gold or Silver), second part should
                // be the purity (formatted as follows: XXX,X‰).
                // There can be additional part after than, we don't care about them
                let parts: Vec<_> = text.split_whitespace().collect();
                if parts.len() < 2 {
                    return Err(serde::de::Error::invalid_length(parts.len(), &"2+ parts expected"));
                }

                // Get metal & purity
                let composition = <&str>::deref(parts.first().expect("We should have parts")).to_owned();
                let mut purity = <&str>::deref(
                    parts
                        .iter()
                        .find(|&&e| e.ends_with('‰'))
                        .ok_or_else(|| serde::de::Error::custom("the char ‰ was expected"))?,
                )
                .to_owned();
                purity.pop().expect("purity should not be empty"); // Remove the ‰ sign to parse it to int

                // Parse it first to float in case there is a comma, multiply it by 10 then hard cast it to i32
                let purity: f32 = purity
                    .replace(',', ".")
                    .parse()
                    .map_err(|_| serde::de::Error::custom("purity should be a unsigned int"))?;
                // Purity bounded by 0..10000 so the cast can't overflow
                #[allow(clippy::cast_possible_truncation)]
                let purity = (purity * 10.0).round() as i32;

                Ok(CoinQueryComposition {
                    //text,
                    composition: match composition.as_str() {
                        "Or" => "GOLD",
                        "Argent" => "SILVER",
                        _ => return Err(serde::de::Error::custom("Unexpected composition")),
                    }
                    .into(),
                    purity,
                })
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{deserialize_date_to_string, CoinQueryComposition};
    use claims::assert_err;

    #[test]
    fn coin_query_composition_parses_gold() {
        let composition = serde_json::from_str::<CoinQueryComposition>(r#"{"text": "Or 999‰"}"#).unwrap();
        assert_eq!(composition.composition, "GOLD");
        assert_eq!(composition.purity, 9990);
    }

    #[test]
    fn coin_query_composition_parses_silver() {
        let composition = serde_json::from_str::<CoinQueryComposition>(r#"{"text": "Argent 925‰"}"#).unwrap();
        assert_eq!(composition.composition, "SILVER");
        assert_eq!(composition.purity, 9250);
    }

    #[test]
    fn coin_query_composition_parses_purity_with_comma_as_decimal() {
        let composition = serde_json::from_str::<CoinQueryComposition>(r#"{"text": "Or 999,9‰"}"#).unwrap();
        assert_eq!(composition.purity, 9999);
    }

    #[test]
    fn coin_query_composition_ignores_extra_parts_after_purity() {
        let composition =
            serde_json::from_str::<CoinQueryComposition>(r#"{"text": "Argent 800‰ some extra text"}"#).unwrap();
        assert_eq!(composition.composition, "SILVER");
        assert_eq!(composition.purity, 8000);
    }

    #[test]
    fn coin_query_composition_locates_purity_by_permille_suffix() {
        let composition = serde_json::from_str::<CoinQueryComposition>(r#"{"text": "Or coin 916‰"}"#).unwrap();
        assert_eq!(composition.composition, "GOLD");
        assert_eq!(composition.purity, 9160);
    }

    #[test]
    fn coin_query_composition_errors_when_parsing_missing_text_field() {
        let result = serde_json::from_str::<CoinQueryComposition>(r#"{"other": "Or 999‰"}"#);
        assert_err!(result);
    }

    #[test]
    fn coin_query_composition_errors_when_parsing_text_with_a_single_part() {
        let result = serde_json::from_str::<CoinQueryComposition>(r#"{"text": "Or"}"#);
        assert_err!(result);
    }

    #[test]
    fn coin_query_composition_errors_when_parsing_empty_text() {
        let result = serde_json::from_str::<CoinQueryComposition>(r#"{"text": ""}"#);
        assert_err!(result);
    }

    #[test]
    fn coin_query_composition_errors_when_parsing_text_without_a_permille_sign() {
        let result = serde_json::from_str::<CoinQueryComposition>(r#"{"text": "Or 999"}"#);
        assert_err!(result);
    }

    #[test]
    fn coin_query_composition_errors_when_parsing_unknown_composition() {
        let result = serde_json::from_str::<CoinQueryComposition>(r#"{"text": "Platine 999‰"}"#);
        assert_err!(result);
    }

    #[test]
    fn coin_query_composition_errors_when_parsing_non_numeric_purity() {
        let result = serde_json::from_str::<CoinQueryComposition>(r#"{"text": "Or abc‰"}"#);
        assert_err!(result);
    }

    #[test]
    fn deserialize_date_to_string_converts_date_to_number() {
        let mut de = serde_json::Deserializer::from_str("1852");
        assert_eq!(deserialize_date_to_string(&mut de).unwrap(), "1852");
    }

    #[test]
    fn deserialize_date_to_string_errors_on_non_numeric_date() {
        let mut de = serde_json::Deserializer::from_str("eighteen fifty two");
        assert!(deserialize_date_to_string(&mut de).is_err());
    }

    #[test]
    fn deserialize_date_to_string_errors_on_negative_date() {
        let mut de = serde_json::Deserializer::from_str("-1852");
        assert!(deserialize_date_to_string(&mut de).is_err());
    }
}
