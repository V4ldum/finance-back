use crate::domain::{AssetComposition, AssetName, AssetPossessed, AssetPurity, AssetUnitWeight};

pub(crate) struct UpdateRawAsset {
    pub(crate) name: Option<AssetName>,
    pub(crate) possessed: Option<AssetPossessed>,
    pub(crate) unit_weight: Option<AssetUnitWeight>,
    pub(crate) composition: Option<AssetComposition>,
    pub(crate) purity: Option<AssetPurity>,
}
