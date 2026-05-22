use std::hash::Hash;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceNode(PathBuf);
