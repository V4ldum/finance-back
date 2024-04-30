use std::fmt::Formatter;
use std::ops::Deref;

use serde::{Deserialize, Deserializer};
use serde::de::MapAccess;

#[derive(Deserialize, Debug)]
pub struct CoinQuery {
    pub title: String,
    pub weight: f64,
    pub size: f64,
    pub thickness: Option<f64>,
    #[serde(deserialize_with = "deserialize_date_to_string")]
    pub min_year: String,
    #[serde(deserialize_with = "deserialize_date_to_string")]
    pub max_year: String,
    pub composition: CoinQueryComposition,
    pub obverse: Option<CoinQuerySide>,
    pub reverse: Option<CoinQuerySide>,
    pub edge: Option<CoinQuerySide>,
    pub watermark: Option<CoinQuerySide>,
}

#[derive(Debug)]
pub struct CoinQueryComposition {
    //pub text: String,
    pub composition: String,
    pub purity: i32,
}

#[derive(Deserialize, Debug)]
pub struct CoinQuerySide {
    pub description: Option<String>,
    pub lettering: Option<String>,
    pub picture: Option<String>,
    pub thumbnail: Option<String>,
    pub picture_copyright: Option<String>,
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
                            map.next_value()?;
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
                    return Err(serde::de::Error::invalid_length(
                        parts.len(),
                        &"2+ parts expected",
                    ));
                }

                // Get metal & purity
                let composition =
                    <&str>::deref(parts.first().expect("We should have parts")).to_owned();
                let mut purity = <&str>::deref(
                    parts
                        .iter()
                        .find(|&&e| e.ends_with('‰'))
                        .expect("the char ‰ was expected"),
                )
                .to_owned();
                purity.pop().expect("purity should not be empty"); // Remove the ‰ sign to parse it to int

                // Parse it first to float in case there is a comma, multiply it by 10 then hard cast it to i32
                let purity: f32 = purity.parse().expect("purity should be a unsigned int");
                let purity = (purity * 10.0) as i32;

                Ok(CoinQueryComposition {
                    //text,
                    composition: match composition.as_str() {
                        "Or" => "GOLD",
                        "Argent" => "SILVER",
                        _ => panic!("Unexpected composition"),
                    }
                    .into(),
                    purity,
                })
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}
