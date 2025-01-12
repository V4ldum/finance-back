use chrono::NaiveDate;

pub struct Price {
    pub name: String,
    pub value: f64,
    pub date: NaiveDate,
}
