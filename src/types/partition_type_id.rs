use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartitionTypeId {
    MbrCode(u8),
    GptUuid(String),
    Other(String),
}
