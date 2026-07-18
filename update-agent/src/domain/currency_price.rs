use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EURUSDExchangeRate {
    pub chart: EURUSDExchangeRateChart,
}

#[derive(Debug, Deserialize)]
pub struct EURUSDExchangeRateChart {
    pub result: Vec<EURUSDExchangeRateResult>,
}

#[derive(Debug, Deserialize)]
pub struct EURUSDExchangeRateResult {
    pub indicators: EURUSDExchangeRateResultIndicator,
}

#[derive(Debug, Deserialize)]
pub struct EURUSDExchangeRateResultIndicator {
    pub quote: Vec<EURUSDExchangeRateQuote>,
}

#[derive(Debug, Deserialize)]
pub struct EURUSDExchangeRateQuote {
    pub close: Vec<Option<f64>>,
}

impl EURUSDExchangeRate {
    pub fn exchange_rate(&self) -> Option<f64> {
        self.chart
            .result
            .first()?
            .indicators
            .quote
            .iter()
            .find_map(|quote| quote.close.iter().copied().rfind(|item| item.is_some())?)
    }
}

#[cfg(test)]
mod tests {
    use approx::relative_eq;
    use claims::{assert_none, assert_some};
    use fake::Fake;

    use super::{
        EURUSDExchangeRate, EURUSDExchangeRateChart, EURUSDExchangeRateQuote, EURUSDExchangeRateResult,
        EURUSDExchangeRateResultIndicator,
    };

    fn rate_to_find() -> f64 {
        // Magic number to find in the exchange rate
        2.0
    }

    fn rate() -> f64 {
        (0.5..1.5).fake()
    }

    #[test]
    fn exchange_rate_returns_some_when_the_input_is_valid() {
        let rate_to_find = rate_to_find();
        let test_cases = vec![
            (
                EURUSDExchangeRate {
                    chart: EURUSDExchangeRateChart {
                        result: vec![EURUSDExchangeRateResult {
                            indicators: EURUSDExchangeRateResultIndicator {
                                quote: vec![EURUSDExchangeRateQuote {
                                    close: vec![Some(rate_to_find)],
                                }],
                            },
                        }],
                    },
                },
                "close had one rate",
            ),
            (
                EURUSDExchangeRate {
                    chart: EURUSDExchangeRateChart {
                        result: vec![EURUSDExchangeRateResult {
                            indicators: EURUSDExchangeRateResultIndicator {
                                quote: vec![EURUSDExchangeRateQuote {
                                    close: vec![None, Some(rate()), None, Some(rate_to_find)],
                                }],
                            },
                        }],
                    },
                },
                "close had multiple rates and Nones",
            ),
            (
                EURUSDExchangeRate {
                    chart: EURUSDExchangeRateChart {
                        result: vec![EURUSDExchangeRateResult {
                            indicators: EURUSDExchangeRateResultIndicator {
                                quote: vec![
                                    EURUSDExchangeRateQuote {
                                        close: vec![None, Some(rate()), None, Some(rate_to_find)],
                                    },
                                    EURUSDExchangeRateQuote {
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
                EURUSDExchangeRate {
                    chart: EURUSDExchangeRateChart {
                        result: vec![EURUSDExchangeRateResult {
                            indicators: EURUSDExchangeRateResultIndicator {
                                quote: vec![
                                    EURUSDExchangeRateQuote {
                                        close: vec![None, None, None],
                                    },
                                    EURUSDExchangeRateQuote {
                                        close: vec![Some(rate()), None, Some(rate_to_find), None],
                                    },
                                    EURUSDExchangeRateQuote {
                                        close: vec![None, Some(rate())],
                                    },
                                    EURUSDExchangeRateQuote {
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
            let exchange_rate = quote.exchange_rate();

            let exchange_rate = assert_some!(exchange_rate, "exchange rate was not Some when {error_message}");
            assert!(
                relative_eq!(exchange_rate, rate_to_find),
                "Correct exchange rate was not found when {error_message}"
            );
        }
    }

    #[test]
    fn exchange_rate_returns_none_when_the_input_is_invalid() {
        let test_cases = vec![
            (
                EURUSDExchangeRate {
                    chart: EURUSDExchangeRateChart { result: Vec::new() },
                },
                "result was empty",
            ),
            (
                EURUSDExchangeRate {
                    chart: EURUSDExchangeRateChart {
                        result: vec![EURUSDExchangeRateResult {
                            indicators: EURUSDExchangeRateResultIndicator { quote: Vec::new() },
                        }],
                    },
                },
                "quote was empty",
            ),
            (
                EURUSDExchangeRate {
                    chart: EURUSDExchangeRateChart {
                        result: vec![EURUSDExchangeRateResult {
                            indicators: EURUSDExchangeRateResultIndicator {
                                quote: vec![EURUSDExchangeRateQuote { close: Vec::new() }],
                            },
                        }],
                    },
                },
                "close was empty with one item",
            ),
            (
                EURUSDExchangeRate {
                    chart: EURUSDExchangeRateChart {
                        result: vec![EURUSDExchangeRateResult {
                            indicators: EURUSDExchangeRateResultIndicator {
                                quote: vec![
                                    EURUSDExchangeRateQuote { close: Vec::new() },
                                    EURUSDExchangeRateQuote { close: Vec::new() },
                                ],
                            },
                        }],
                    },
                },
                "close was empty with multiple items",
            ),
            (
                EURUSDExchangeRate {
                    chart: EURUSDExchangeRateChart {
                        result: vec![EURUSDExchangeRateResult {
                            indicators: EURUSDExchangeRateResultIndicator {
                                quote: vec![EURUSDExchangeRateQuote {
                                    close: vec![None, None],
                                }],
                            },
                        }],
                    },
                },
                "close was filled with None with one item",
            ),
            (
                EURUSDExchangeRate {
                    chart: EURUSDExchangeRateChart {
                        result: vec![EURUSDExchangeRateResult {
                            indicators: EURUSDExchangeRateResultIndicator {
                                quote: vec![
                                    EURUSDExchangeRateQuote {
                                        close: vec![None, None],
                                    },
                                    EURUSDExchangeRateQuote {
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
            assert_none!(
                invalid_rate.exchange_rate(),
                "Exchange rate did not return None when {error_message}"
            );
        }
    }
}
