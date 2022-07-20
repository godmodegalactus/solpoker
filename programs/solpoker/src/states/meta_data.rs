#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
#[repr(C)]
/// Stores meta information about the `Account` on chain
pub struct MetaData {
    pub data_type: DataType,
    pub version: u8,
    pub is_initialized: bool,
    pub extra_info: [u8; 5],
}