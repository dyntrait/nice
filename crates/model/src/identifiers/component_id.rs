

//! Represents a valid component ID.

use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use nice_core::correctness::{
    CorrectnessResult, CorrectnessResultExt, FAILED, check_valid_string_ascii,
};
use ustr::Ustr;

use crate::identifiers::{ActorId, ExecAlgorithmId, StrategyId};

/// Represents a valid component ID.
#[repr(C)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentId(Ustr);

impl ComponentId {
    /// Creates a new [`ComponentId`] instance with correctness checking.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is not a valid string.
    ///
    /// # Notes
    ///
    /// PyO3 requires a `Result` type for proper error handling and stacktrace printing in Python.
    pub fn new_checked<T: AsRef<str>>(value: T) -> CorrectnessResult<Self> {
        let value = value.as_ref();
        check_valid_string_ascii(value, stringify!(value))?;
        Ok(Self(Ustr::from(value)))
    }

    /// Creates a new [`ComponentId`] instance.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not a valid string.
    pub fn new<T: AsRef<str>>(value: T) -> Self {
        Self::new_checked(value).expect_display(FAILED)
    }

    /// Sets the inner identifier value.
    #[cfg_attr(not(feature = "python"), allow(dead_code))]
    pub(crate) fn set_inner(&mut self, value: &str) {
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

impl_from_identifier_for_component_id!(ActorId);
impl_from_identifier_for_component_id!(ExecAlgorithmId);
impl_from_identifier_for_component_id!(StrategyId);

impl Debug for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

impl Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::ComponentId;
    use crate::identifiers::{ActorId, ExecAlgorithmId, StrategyId, stubs::*};

    #[rstest]
    fn test_string_reprs(component_risk_engine: ComponentId) {
        assert_eq!(component_risk_engine.as_str(), "RiskEngine");
        assert_eq!(format!("{component_risk_engine}"), "RiskEngine");
    }

    #[rstest]
    fn test_from_actor_id() {
        let component_id = ComponentId::from(ActorId::from("MyActor"));
        assert_eq!(component_id, ComponentId::from("MyActor"));
    }

    #[rstest]
    fn test_from_exec_algorithm_id() {
        let component_id = ComponentId::from(ExecAlgorithmId::from("TWAP"));
        assert_eq!(component_id, ComponentId::from("TWAP"));
    }

    #[rstest]
    fn test_from_strategy_id() {
        let component_id = ComponentId::from(StrategyId::from("EMACross-001"));
        assert_eq!(component_id, ComponentId::from("EMACross-001"));
    }

    #[rstest]
    #[should_panic(expected = "Condition failed: invalid string for 'value', was empty")]
    fn test_new_with_empty_string_panics_with_display_format() {
        let _ = ComponentId::new("");
    }
}
