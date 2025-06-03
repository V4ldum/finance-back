use sqlx::FromRow;

// TODO remove derive. Check Search Coin function for details.
#[derive(FromRow)]
pub struct Coin {
    pub id: i64,
    pub numista_id: String,
    pub name: String,
    pub weight: f64,
    pub size: f64,
    pub thickness: Option<f64>,
    pub min_year: String,
    pub max_year: Option<String>,
    pub composition: String,
    pub purity: i64,
    pub obverse: Option<i64>,
    pub reverse: Option<i64>,
    pub edge: Option<i64>,
}
