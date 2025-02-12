use crate::{
    constants::{EMAIL_OFFSET, EMAIL_SIZE, ID_OFFSET, ID_SIZE, USERNAME_OFFSET, USERNAME_SIZE},
    engine::Row,
};
use std::ptr;

/// A helper type for serialization.
pub struct Serialize;

/// A helper type for deserialization.
pub struct Deserialize;

impl Serialize {
    /// Copies the fields of `source` into the memory pointed to by `destination`
    /// using the predefined offsets and sizes.
    pub unsafe fn serialize_row(source: &Row, destination: *mut u8) {
        ptr::copy_nonoverlapping(
            &source.id as *const i32 as *const u8,
            destination.add(ID_OFFSET),
            ID_SIZE,
        );
        ptr::copy_nonoverlapping(
            source.username.as_ptr(),
            destination.add(USERNAME_OFFSET),
            USERNAME_SIZE,
        );
        ptr::copy_nonoverlapping(
            source.email.as_ptr(),
            destination.add(EMAIL_OFFSET),
            EMAIL_SIZE,
        );
    }
}

impl Deserialize {
    /// Copies data from the memory pointed to by `source` into the fields of `destination`
    /// using the predefined offsets and sizes.
    pub unsafe fn deserialize_row(source: *const u8, destination: &mut Row) {
        ptr::copy_nonoverlapping(
            source.add(ID_OFFSET),
            &mut destination.id as *mut i32 as *mut u8,
            ID_SIZE,
        );
        ptr::copy_nonoverlapping(
            source.add(USERNAME_OFFSET),
            destination.username.as_mut_ptr(),
            USERNAME_SIZE,
        );
        ptr::copy_nonoverlapping(
            source.add(EMAIL_OFFSET),
            destination.email.as_mut_ptr(),
            EMAIL_SIZE,
        );
    }
}
