pub(crate) struct CoinImage {
    #[expect(dead_code)]
    pub(crate) id: i64,
    pub(crate) image_url: Option<String>,
    pub(crate) thumbnail_url: Option<String>,
    pub(crate) lettering: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) copyright: Option<String>,
}
