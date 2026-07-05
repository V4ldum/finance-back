use chrono::NaiveDate;

pub(crate) struct PriceDb {
    pub(crate) name: String,
    pub(crate) value: f64,
    pub(crate) date: NaiveDate,
}
