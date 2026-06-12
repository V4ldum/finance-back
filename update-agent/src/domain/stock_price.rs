use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SP500Price {
    pub chart: SP500PriceChart,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceChart {
    pub result: Vec<SP500PriceResult>,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceResult {
    pub indicators: SP500PriceResultIndicator,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceResultIndicator {
    pub quote: Vec<SP500PriceResultIndicatorQuote>,
}

#[derive(Deserialize, Debug)]
pub struct SP500PriceResultIndicatorQuote {
    pub close: Vec<Option<f64>>,
}

impl SP500Price {
    pub fn price(&self) -> Option<f64> {
        self.chart
            .result
            .first()?
            .indicators
            .quote
            .iter()
            .find_map(|quote| quote.close.iter().rfind(|item| item.is_some())?.to_owned())
    }
}

#[cfg(test)]
mod test {
    use approx::relative_eq;
    use claims::{assert_none, assert_some};
    use fake::Fake;

    use super::{
        SP500Price, SP500PriceChart, SP500PriceResult, SP500PriceResultIndicator, SP500PriceResultIndicatorQuote,
    };

    fn price_to_find() -> f64 {
        // Magic number to find in the exchange rate
        2000.0
    }

    fn rate() -> f64 {
        (500.0..1500.0).fake()
    }

    #[test]
    fn price_returns_some_when_the_input_is_valid() {
        let price_to_find = price_to_find();
        let test_cases = vec![
            (
                SP500Price {
                    chart: SP500PriceChart {
                        result: vec![SP500PriceResult {
                            indicators: SP500PriceResultIndicator {
                                quote: vec![SP500PriceResultIndicatorQuote {
                                    close: vec![Some(price_to_find)],
                                }],
                            },
                        }],
                    },
                },
                "close had one price",
            ),
            (
                SP500Price {
                    chart: SP500PriceChart {
                        result: vec![SP500PriceResult {
                            indicators: SP500PriceResultIndicator {
                                quote: vec![SP500PriceResultIndicatorQuote {
                                    close: vec![None, Some(rate()), None, Some(price_to_find)],
                                }],
                            },
                        }],
                    },
                },
                "close had multiple prices and Nones",
            ),
            (
                SP500Price {
                    chart: SP500PriceChart {
                        result: vec![SP500PriceResult {
                            indicators: SP500PriceResultIndicator {
                                quote: vec![
                                    SP500PriceResultIndicatorQuote {
                                        close: vec![None, Some(rate()), None, Some(price_to_find)],
                                    },
                                    SP500PriceResultIndicatorQuote {
                                        close: vec![None, Some(rate()), None],
                                    },
                                ],
                            },
                        }],
                    },
                },
                "there was multiple quotes and the first one was valid",
            ),
            (
                SP500Price {
                    chart: SP500PriceChart {
                        result: vec![SP500PriceResult {
                            indicators: SP500PriceResultIndicator {
                                quote: vec![
                                    SP500PriceResultIndicatorQuote {
                                        close: vec![None, None, None],
                                    },
                                    SP500PriceResultIndicatorQuote {
                                        close: vec![Some(rate()), None, Some(price_to_find), None],
                                    },
                                    SP500PriceResultIndicatorQuote {
                                        close: vec![None, Some(rate())],
                                    },
                                    SP500PriceResultIndicatorQuote {
                                        close: vec![Some(rate()), None, Some(rate()), None],
                                    },
                                ],
                            },
                        }],
                    },
                },
                "there was multiple quotes and the first one was invalid",
            ),
        ];

        for (quote, error_message) in test_cases {
            let price = quote.price();

            let price = assert_some!(price, "Price was not Some when {error_message}");
            assert!(
                relative_eq!(price, price_to_find),
                "Correct price was not found when {error_message}"
            );
        }
    }

    #[test]
    fn price_returns_none_when_the_input_is_invalid() {
        let test_cases = vec![
            (
                SP500Price {
                    chart: SP500PriceChart { result: Vec::new() },
                },
                "result was empty",
            ),
            (
                SP500Price {
                    chart: SP500PriceChart {
                        result: vec![SP500PriceResult {
                            indicators: SP500PriceResultIndicator { quote: Vec::new() },
                        }],
                    },
                },
                "quote was empty",
            ),
            (
                SP500Price {
                    chart: SP500PriceChart {
                        result: vec![SP500PriceResult {
                            indicators: SP500PriceResultIndicator {
                                quote: vec![SP500PriceResultIndicatorQuote { close: Vec::new() }],
                            },
                        }],
                    },
                },
                "close was empty with one item",
            ),
            (
                SP500Price {
                    chart: SP500PriceChart {
                        result: vec![SP500PriceResult {
                            indicators: SP500PriceResultIndicator {
                                quote: vec![
                                    SP500PriceResultIndicatorQuote { close: Vec::new() },
                                    SP500PriceResultIndicatorQuote { close: Vec::new() },
                                ],
                            },
                        }],
                    },
                },
                "close was empty with multiple items",
            ),
            (
                SP500Price {
                    chart: SP500PriceChart {
                        result: vec![SP500PriceResult {
                            indicators: SP500PriceResultIndicator {
                                quote: vec![SP500PriceResultIndicatorQuote {
                                    close: vec![None, None],
                                }],
                            },
                        }],
                    },
                },
                "close was filled with None with one item",
            ),
            (
                SP500Price {
                    chart: SP500PriceChart {
                        result: vec![SP500PriceResult {
                            indicators: SP500PriceResultIndicator {
                                quote: vec![
                                    SP500PriceResultIndicatorQuote {
                                        close: vec![None, None],
                                    },
                                    SP500PriceResultIndicatorQuote {
                                        close: vec![None, None],
                                    },
                                ],
                            },
                        }],
                    },
                },
                "close was filled with None with multiple items",
            ),
        ];

        for (invalid_rate, error_message) in test_cases {
            assert_none!(invalid_rate.price(), "Price did not return None when {error_message}");
        }
    }
}
