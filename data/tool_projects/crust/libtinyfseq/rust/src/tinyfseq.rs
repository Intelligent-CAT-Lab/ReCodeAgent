#[derive(Debug, PartialEq)]
pub enum TFError {
TF_OK = 0,
TF_EINVALID_MAGIC,
TF_EINVALID_COMPRESSION_TYPE,
TF_EINVALID_BUFFER_SIZE,
TF_EINVALID_VAR_SIZE,
}
impl TFError {
pub fn to_string(&self) -> &'static str {
match self {
TFError::TF_OK => "TF_OK (ok)",
TFError::TF_EINVALID_MAGIC => "TF_EINVALID_MAGIC (invalid magic file signature)",
TFError::TF_EINVALID_COMPRESSION_TYPE => "TF_EINVALID_COMPRESSION_TYPE (unknown compression identifier)",
TFError::TF_EINVALID_BUFFER_SIZE => "TF_EINVALID_BUFFER_SIZE (undersized data decoding buffer argument)",
TFError::TF_EINVALID_VAR_SIZE => "TF_EINVALID_VAR_SIZE (invalid variable size in header)",
}
}
}
#[derive(Debug)]
pub enum TFCompressionType {
TF_COMPRESSION_NONE,
TF_COMPRESSION_ZSTD,
TF_COMPRESSION_ZLIB,
}
#[derive(Debug)]
pub struct TFHeader {
pub channel_data_offset: u16,
pub minor_version: u8,
pub major_version: u8,
pub variable_data_offset: u16,
pub channel_count: u32,
pub frame_count: u32,
pub frame_step_time_millis: u8,
pub compression_type: TFCompressionType,
pub compression_block_count: u8,
pub channel_range_count: u8,
pub sequence_uid: u64,
}
#[derive(Debug)]
pub struct TFCompressionBlock {
pub first_frame_id: u32,
pub size: u32,
}
#[derive(Debug)]
pub struct TFVarHeader {
pub size: u16,
pub id: [u8; 2],
}
#[derive(Debug)]
pub struct TFChannelRange {
pub first_channel_number: u32,
pub channel_count: u32,
}
pub fn tf_var_header_read(
    bd: &[u8],
    var_header: &mut TFVarHeader,
    vd: &mut [u8],
    ep: Option<&mut &[u8]>,
) -> TFError {
    unimplemented!()
}
pub fn tf_header_read(
    bd: &[u8],
    header: &mut TFHeader,
    ep: Option<&mut &[u8]>,
) -> TFError {
    unimplemented!()
}
pub fn tf_compression_block_read(
    bd: &[u8],
    block: &mut TFCompressionBlock,
    ep: Option<&mut &[u8]>,
) -> TFError {
    unimplemented!()
}
pub fn tf_channel_range_read(
    bd: &[u8],
    channel_range: &mut TFChannelRange,
    ep: Option<&mut &[u8]>,
) -> TFError {
    unimplemented!()
}