// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025 dyntrait. All rights reserved.
//
//  @File         : trader_id.rs
//  @Author       : dyntrait
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

//! Represents a valid trader ID.

use std::fmt::{Debug, Display, Formatter};

use nice_core::correctness::{FAILED, check_string_contains, check_valid_string_ascii};
use ustr::Ustr;

/// Represents a valid trader ID.
#[repr(C)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TraderId(Ustr);

impl TraderId {
    /// Creates a new [`TraderId`] instance.
    ///
    /// Must be correctly formatted with two valid strings either side of a hyphen.
    /// It is expected a trader ID is the abbreviated name of the trader
    /// with an order ID tag number separated by a hyphen.
    ///
    /// Example: "TESTER-001".
    ///
    /// The reason for the numerical component of the ID is so that order and position IDs
    /// do not collide with those from another node instance.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is not a valid string, or does not contain a hyphen '-' separator.
    ///
    /// # Notes
    ///
    /// PyO3 requires a `Result` type for proper error handling and stacktrace printing in Python.
    pub fn new_checked<T: AsRef<str>>(value: T) -> anyhow::Result<Self> {
        let value = value.as_ref();
        check_valid_string_ascii(value, stringify!(value))?;
        check_string_contains(value, "-", stringify!(value))?;
        Ok(Self(Ustr::from(value)))
    }

    /// Creates a new [`TraderId`] instance.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not a valid string, or does not contain a hyphen '-' separator.
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

    /// Returns the numerical tag portion of the trader ID.
    ///
    /// # Panics
    ///
    /// Panics if the internal ID string does not contain a '-' separator.
    #[must_use]
    pub fn get_tag(&self) -> &str {
        // SAFETY: Unwrap safe as value previously validated
        self.0.split('-').next_back().unwrap()
    }
}

impl Debug for TraderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for TraderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::identifiers::{stubs::*, trader_id::TraderId};

    #[rstest]
    fn test_string_reprs(trader_id: TraderId) {
        assert_eq!(trader_id.as_str(), "TRADER-001");
        assert_eq!(format!("{trader_id}"), "TRADER-001");
    }

    #[rstest]
    fn test_get_tag(trader_id: TraderId) {
        assert_eq!(trader_id.get_tag(), "001");
    }
}


