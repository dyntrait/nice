// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025  dyntrait  All rights reserved.
//  All Rights Reserved
//
//  @File         : actor_id.rs
//  @Author       : dyntrait
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

//! Represents a valid component ID.

use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
};

use nice_core::correctness::{FAILED, check_valid_string_ascii};
use ustr::Ustr;

/// Represents a valid component ID.
#[repr(C)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActorId(Ustr);

impl ActorId {
    /// Creates a new [`ActorId`] instance with correctness checking.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is not a valid string.
    ///
    /// # Notes
    ///
    /// PyO3 requires a `Result` type for proper error handling and stacktrace printing in Python.
    pub fn new_checked<T: AsRef<str>>(value: T) -> anyhow::Result<Self> {
        let value = value.as_ref();
        check_valid_string_ascii(value, stringify!(value))?;
        Ok(Self(Ustr::from(value)))
    }

    /// Creates a new [`ActorId`] instance.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not a valid string.
    pub fn new<T: AsRef<str>>(value: T) -> Self {
        Self::new_checked(value).expect(FAILED)
    }

    /// Sets the inner identifier value.
    pub fn set_inner(&mut self, value: &str) {
        self.0 = Ustr::from(value);
    }

    /// Returns the inner identifier value.
    #[must_use]
    pub fn inner(&self) -> Ustr {
        self.0
    }

    /// Returns the inner identifier value as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Debug for ActorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for ActorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::ActorId;

    #[rstest]
    fn test_string_reprs() {
        let actor_id = ActorId::from("MyActor");
        assert_eq!(actor_id.as_str(), "MyActor");
        assert_eq!(format!("{actor_id}"), "MyActor");
    }
}
