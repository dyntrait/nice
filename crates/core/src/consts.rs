// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025 dyntrait. All rights reserved.
//
//  @File         : consts.rs
//  @Author       : mark.m
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------
//! Core constants.

use std::env;

/// The NautilusTrader string constant.
pub static NICE_TRADER: &str = "NiceTrader";

/// The NautilusTrader version string read from the top-level `pyproject.toml` at compile time.
pub static NICE_VERSION: &str = env!("NICE_VERSION");

/// The NautilusTrader common User-Agent string including the current version at compile time.
pub static NICE_USER_AGENT: &str = env!("NICE_USER_AGENT");
