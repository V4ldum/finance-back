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
    pub obverse_id: Option<i64>,
    pub reverse_id: Option<i64>,
    pub edge_id: Option<i64>,
}
