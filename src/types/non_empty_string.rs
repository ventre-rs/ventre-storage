use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyString(String);
