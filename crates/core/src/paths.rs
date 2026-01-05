// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025 dyntrait. All rights reserved.
//
//  @File         : paths.rs
//  @Author       : dyntrait
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

//! Utility functions for resolving project and workspace directory paths.

use std::path::PathBuf;

/// Returns the workspace root directory path.
///
/// # Panics
///
/// Panics if the `CARGO_MANIFEST_DIR` environment variable is not set or its parent directory cannot be determined.
#[must_use]
pub fn get_workspace_root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Failed to get project root")
        .to_path_buf()
}

/// Returns the project root directory path.
///
/// # Panics
///
/// Panics if the workspace root path cannot be determined or its parent directory is missing.
#[must_use]
pub fn get_project_root_path() -> PathBuf {
    get_workspace_root_path()
        .parent()
        .expect("Failed to get workspace root")
        .to_path_buf()
}

/// Returns the tests root directory path.
#[must_use]
pub fn get_tests_root_path() -> PathBuf {
    get_project_root_path().join("tests")
}

/// Returns the test data directory path.
#[must_use]
pub fn get_test_data_path() -> PathBuf {
    if let Ok(test_data_root_path) = std::env::var("TEST_DATA_ROOT_PATH") {
        get_project_root_path()
            .join(test_data_root_path)
            .join("test_data")
    } else {
        get_project_root_path().join("tests").join("test_data")
    }
}
