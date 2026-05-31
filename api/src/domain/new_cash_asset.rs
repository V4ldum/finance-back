use crate::domain::{AssetName, AssetPossessed, AssetUnitValue};

pub(crate) struct NewCashAsset {
    pub(crate) name: AssetName,
    pub(crate) possessed: AssetPossessed,
    pub(crate) unit_value: AssetUnitValue,
}
