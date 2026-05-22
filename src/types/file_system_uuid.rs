use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileSystemUuid(String);
