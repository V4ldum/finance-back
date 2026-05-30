use crate::domain::{AssetName, AssetPossessed, AssetUnitValue};

pub struct NewCashAsset {
    pub name: AssetName,
    pub possessed: AssetPossessed,
    pub unit_value: AssetUnitValue,
}
