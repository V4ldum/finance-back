use crate::domain::{AssetComposition, AssetName, AssetPossessed, AssetPurity, AssetUnitWeight};

pub(crate) struct CreateRawAsset {
    pub(crate) name: AssetName,
    pub(crate) possessed: AssetPossessed,
    pub(crate) unit_weight: AssetUnitWeight,
    pub(crate) composition: AssetComposition,
    pub(crate) purity: AssetPurity,
}
