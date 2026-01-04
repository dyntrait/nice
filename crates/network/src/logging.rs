// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2026  dyntrait  All rights reserved.
//  All Rights Reserved
//
//  @File         : logging.rs
//  @Author       : dyntrait
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------


/// Logs that a task has started using `tracing::debug!`.
pub fn log_task_started(task_name: &str) {
    tracing::debug!("Started task '{task_name}'");
}

/// Logs that a task has stopped using `tracing::debug!`.
pub fn log_task_stopped(task_name: &str) {
    tracing::debug!("Stopped task '{task_name}'");
}

/// Logs that a task was aborted using `tracing::debug!`.
pub fn log_task_aborted(task_name: &str) {
    tracing::debug!("Aborted task '{task_name}'");
}
