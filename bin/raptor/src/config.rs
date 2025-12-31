// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025  dyntrait  All rights reserved.
//  All Rights Reserved
//
//  @File         : config.rs
//  @Author       : dyntrait
//  @Description  : 
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

use figment::{Figment, providers::{Format, Toml, Env}};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub log: nice_common::tracing::LogConfig,
}
pub static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    AppConfig::load().expect("miss config")
});

impl AppConfig {
    fn load() -> Result<Self, figment::Error> {
        Figment::new()
            // 基础：从配置文件读取 [log]
            .merge(Toml::file("./etc/config.toml"))
            // 覆盖：从环境变量读取 APP_LOG_...
            .merge(Env::prefixed("RAPTOR_").split("_"))
            .extract()
    }
}