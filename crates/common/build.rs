// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025 dyntrait. All rights reserved.
//
//  @File         : build.rs
//  @Author       : mark.m
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------
use vergen::{
    BuildBuilder, CargoBuilder, Emitter, RustcBuilder, SysinfoBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 定义想要捕获的信息
    let build = BuildBuilder::all_build()?;
    let cargo = CargoBuilder::all_cargo()?;
    let rustc = RustcBuilder::all_rustc()?;
    let si = SysinfoBuilder::all_sysinfo()?;

    // 发射指令给 Cargo (这会自动生成环境变量)
    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?
        .emit()?;

    Ok(())
}