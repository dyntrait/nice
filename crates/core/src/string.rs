// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025 dyntrait. All rights reserved.
//
//  @File         : string.rs
//  @Author       : dyntrait
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

//! String manipulation functionality.

/// Masks an API key by showing only the first and last 4 characters.
///
/// For keys 8 characters or shorter, returns asterisks only.
///
/// # Examples
///
/// ```
/// use nautilus_core::string::mask_api_key;
///
/// assert_eq!(mask_api_key("abcdefghijklmnop"), "abcd...mnop");
/// assert_eq!(mask_api_key("short"), "*****");
/// ```
#[must_use]
pub fn mask_api_key(key: &str) -> String {
    // Work with Unicode scalars to avoid panicking on multibyte characters.
    let chars: Vec<char> = key.chars().collect();
    let len = chars.len();

    if len <= 8 {
        return "*".repeat(len);
    }

    let first: String = chars[..4].iter().collect();
    let last: String = chars[len - 4..].iter().collect();

    format!("{first}...{last}")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("", "")]
    #[case("a", "*")]
    #[case("abc", "***")]
    #[case("abcdefgh", "********")]
    #[case("abcdefghi", "abcd...fghi")]
    #[case("abcdefghijklmnop", "abcd...mnop")]
    #[case("VeryLongAPIKey123456789", "Very...6789")]
    fn test_mask_api_key(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(mask_api_key(input), expected);
    }
}
