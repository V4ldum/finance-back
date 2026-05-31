use crate::domain::{AssetName, AssetPossessed, AssetUnitValue};

pub(crate) struct UpdateCashAsset {
    pub(crate) name: Option<AssetName>,
    pub(crate) possessed: Option<AssetPossessed>,
    pub(crate) unit_value: Option<AssetUnitValue>,
}
