use crate::*;
use crate::states::enums::DataType;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
#[repr(C)]
/// Stores meta information about the `Account` on chain
pub struct MetaData {
    pub data_type: DataType,
    pub version: u8,
    pub is_initialized: bool,
}

impl Default for MetaData {
    fn default() -> Self {
        MetaData { data_type: DataType::Unknown, version: 0, is_initialized: false }
    }
}