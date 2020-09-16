//! This module defines the traits by which the rest of Delorean
//! interacts with the storage system. The goal is to define a clear
//! interface as well as being able to test other parts of Delorean
//! using mockups that conform to these traits

use std::convert::TryFrom;

mod block;
pub mod database;
mod memdb;
pub mod partitioned_store;
pub mod predicate;
mod remote_partition;
mod s3_partition;

use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ReadPoint<T: Clone> {
    pub time: i64,
    pub value: T,
}

impl<T: Clone> From<&'_ crate::line_parser::Point<T>> for ReadPoint<T> {
    fn from(other: &'_ crate::line_parser::Point<T>) -> Self {
        let crate::line_parser::Point { time, value, .. } = other;
        Self {
            time: *time,
            value: value.clone(),
        }
    }
}

// The values for these enum variants have no real meaning, but they
// are serialized to disk. Revisit these whenever it's time to decide
// on an on-disk format.
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SeriesDataType {
    I64 = 0,
    F64 = 1,
    String = 2,
    Bool = 3,
    //    U64,
}

impl From<SeriesDataType> for u8 {
    fn from(other: SeriesDataType) -> Self {
        other as Self
    }
}

impl TryFrom<u8> for SeriesDataType {
    type Error = u8;

    fn try_from(other: u8) -> Result<Self, Self::Error> {
        use SeriesDataType::*;

        match other {
            v if v == I64 as u8 => Ok(I64),
            v if v == F64 as u8 => Ok(F64),
            v if v == String as u8 => Ok(String),
            v if v == Bool as u8 => Ok(Bool),
            _ => Err(other),
        }
    }
}