use crate::domain::AssetPossessed;

pub(crate) struct CreateCoinAsset {
    pub(crate) coin_id: i64,
    pub(crate) possessed: AssetPossessed,
}
