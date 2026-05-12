pub(crate) struct CashAsset {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) possessed: i64,
    pub(crate) unit_value: i64,
    #[expect(dead_code)]
    pub(crate) id_user: i64,
}
