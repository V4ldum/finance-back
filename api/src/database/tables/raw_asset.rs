pub(crate) struct RawAsset {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) possessed: i64,
    pub(crate) unit_weight: i64,
    pub(crate) composition: String,
    pub(crate) purity: i64,
    #[expect(dead_code)]
    pub(crate) id_user: i64,
}
