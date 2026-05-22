use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceNumber {
    pub major: u32,
    pub minor: u32,
}
