// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025  dyntrait  All rights reserved.
//  All Rights Reserved
//
//  @File         : lib.rs
//  @Author       : dyntrait
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------
pub mod config;
pub use config::CONFIG;
pub fn init() {
    println!("hello world");
}