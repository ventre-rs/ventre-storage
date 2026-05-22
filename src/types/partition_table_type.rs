use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartitionTableType {
    Gpt,
    Dos,
    Other(String),
}
