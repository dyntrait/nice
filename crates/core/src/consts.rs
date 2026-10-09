

//! Core constants.

/// The niceTrader string constant.
pub static NICE_TRADER: &str = "niceTrader";

/// The `nice-core` crate version string embedded at compile time.
pub static NICE_VERSION_CORE: &str = env!("CARGO_PKG_VERSION");

/// The niceTrader version string selected for the compiled application.
pub static NICE_VERSION: &str = NICE_VERSION_CORE;

/// The niceTrader common User-Agent string including the current version at compile time.
pub static NICE_USER_AGENT: &str = concat!("niceTrader/", env!("CARGO_PKG_VERSION"));

/// Prefix for log messages outside the main logging subsystem.
pub static NICE_PREFIX: &str = "[nice]";

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_nice_versions_rust() {
        assert_eq!(NICE_VERSION_CORE, env!("CARGO_PKG_VERSION"));
        assert_eq!(NICE_VERSION, env!("CARGO_PKG_VERSION"));
        assert_eq!(
            NICE_USER_AGENT,
            concat!("niceTrader/", env!("CARGO_PKG_VERSION")),
        );
    }


}
