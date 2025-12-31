// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025  dyntrait  All rights reserved.
//  All Rights Reserved
//
//  @File         : tracing.rs
//  @Author       : dyntrait
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------

use tracing_subscriber::Layer;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use strum::{AsRefStr, Display, EnumString};
use tracing::{Event, Subscriber};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{
        self,
        format::Writer,
        FmtContext,
        FormatEvent,
        FormatFields,
        MakeWriter,
    },
    EnvFilter,
    Registry,
    layer::SubscriberExt,
};


#[allow(unused)]
static DEFAULT_LOGGING_CONFIG: &str = r#"{
        "mode": "both",
        "encoding": "plain",
        "stdout_level": "info",
        "fileout_level": "debug",
        "is_colored": true,
        "file_writer": {
            "directory": "./logs",
            "file_name": "app.log",
            "rotation": "daily",
            "max_size_mb": 100,
            "max_backup_count": 7,
            "keep_days": 7
        }
    }"#;

/// 内部状态标志，防止跨 crate 冲突
static TRACING_LOGGER_IN_USE: AtomicBool = AtomicBool::new(false);

// --- 1. 配置定义 ---

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LogMode {
    Console,
    File,
    Both,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LogEncoding {
    Json,
    Plain,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum RotationRule {
    Daily,
    Hourly,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRotateConfig {
    #[serde(default = "default_rotation")]
    pub rotation: RotationRule,

    /// 单个文件最大大小（单位：MB）。
    /// 目前仅作为保留字段，方便未来扩展 Size-based 轮转。
    #[serde(default)]
    pub max_size_mb: u64,

    #[serde(default)]
    pub max_backup_count: u32,

    #[serde(default)]
    pub keep_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWriterConfig {
    pub directory: String,
    pub file_name: String,
    #[serde(flatten)]
    pub rotate: FileRotateConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_mode")]
    pub mode: LogMode,
    #[serde(default = "default_encoding")]
    pub encoding: LogEncoding,
    #[serde(default = "default_time_format")]
    pub time_format: String,
    pub stdout_level: String,
    pub file_level: String,
    #[serde(default = "default_true")]
    pub is_colored: bool,
    #[serde(default)]
    pub print_config: bool,
    pub file_writer: Option<FileWriterConfig>,
}
impl LogConfig {
    pub fn from_json(json_str: &str) -> anyhow::Result<Self> {
        let config: Self = serde_json::from_str(json_str)?;
        Ok(config)
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            mode: LogMode::Console,
            encoding: LogEncoding::Plain,
            time_format: "%Y-%m-%d %H:%M:%S%.3f %Z".to_string(),
            stdout_level: "INFO".to_string(),
            file_level: "DEBUG".to_string(),
            is_colored: true,
            print_config: true,
            file_writer: None,
        }
    }
}

// 默认值助手
fn default_rotation() -> RotationRule {
    RotationRule::Daily
}
fn default_mode() -> LogMode {
    LogMode::Both
}
fn default_encoding() -> LogEncoding {
    LogEncoding::Json
}
fn default_time_format() -> String {
    "%Y-%m-%dT%H:%M:%S%.3f %Z".to_string()
}
fn default_true() -> bool {
    true
}

// --- 2. 初始化核心 ---

/// 必须持有该 Guard，否则异步日志线程会提前退出
pub struct LogGuard {
    _worker_guards: Vec<WorkerGuard>,
}

impl std::fmt::Debug for LogGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogGuard")
            .field("worker_guards_count", &self._worker_guards.len())
            .finish_non_exhaustive()
    }
}

// 1. 定义带参数的结构体
#[derive(Debug, Clone)]
struct PlainShortFileLineFormatter {
    time_format: String,
}

impl PlainShortFileLineFormatter {
    // 提供一个构造函数
    pub fn new(fmt: String) -> Self {
        Self { time_format: fmt }
    }
}

impl<S, N> FormatEvent<S, N> for PlainShortFileLineFormatter
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let metadata = event.metadata();

        // 2. 【核心修改】使用保存在 self 中的时间格式
        // 这里使用 .format(&self.time_format) 动态解析
        let now = chrono::Local::now();
        write!(writer, "{} ", now.format(&self.time_format))?;

        // --- 以下逻辑保持不变 ---
        let level = metadata.level();
        write!(writer, "{:>5} ", level)?;

        if let (Some(file), Some(line)) = (metadata.file(), metadata.line()) {
            let short_file = file.rfind('/').map(|i| &file[i + 1..]).unwrap_or(file);
            write!(writer, "[{}:{}] ", short_file, line)?;
        } else {
            write!(writer, "[{}] ", metadata.target())?;
        }

        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

/// --- 自定义格式器：JSON 模式，输出 {"file_line": "short_file:line", "message": "..."} ---


#[derive(Debug, Clone)]
struct JsonShortFileLineFormatter {
    time_format: String,
}

impl JsonShortFileLineFormatter {
    pub fn new(fmt: String) -> Self {
        Self { time_format: fmt }
    }
}

impl<S, N> FormatEvent<S, N> for JsonShortFileLineFormatter
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let metadata = event.metadata();

        // 1. 获取自定义格式的时间戳
        let now = chrono::Local::now();
        let formatted_time = now.format(&self.time_format);

        // 2. 路径处理优化：使用 rfind 避免迭代器开销
        let filename = metadata.file()
            .and_then(|f| f.rfind('/').map(|i| &f[i + 1..]).or(Some(f)))
            .unwrap_or("unknown");
        let line = metadata.line().unwrap_or(0);

        // 3. 开始构建 JSON (直接写入 writer，零中间 String 分配)
        // 包含时间戳、日志级别、文件位置
        write!(
            writer,
            "{{\"time\":\"{}\",\"level\":\"{}\",\"file_line\":\"{}:{}\",\"message\":\"",
            formatted_time,
            metadata.level(),
            filename,
            line
        )?;

        // 4. 核心优化：直接流式写入消息字段
        // 注意：如果 event 里的字段包含引号，这里可能需要处理转义。
        // 对于量化系统，如果追求极致性能且消息受控，这样直接写入是最快的。
        ctx.format_fields(writer.by_ref(), event)?;

        // 5. 闭合 JSON 结构
        writeln!(writer, "\"}}")
    }
}

/// 辅助函数：根据 encoding 包装 Formatter 并返回 Boxed Layer
fn build_formatted_layer<S, W>(
    writer: W,
    config: &LogConfig
) -> Box<dyn tracing_subscriber::Layer<S> + Send + Sync>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    W: for<'writer> MakeWriter<'writer> + Send + Sync + 'static,
{
    let time_fmt = config.time_format.clone();
    let fmt_layer = fmt::layer()
        .with_writer(writer)
        .with_ansi(config.is_colored);

    match config.encoding {
        LogEncoding::Json => fmt_layer
            .event_format(JsonShortFileLineFormatter::new(time_fmt))
            .boxed(),
        LogEncoding::Plain => fmt_layer
            .event_format(PlainShortFileLineFormatter::new(time_fmt))
            .boxed(),
    }
}

pub fn init_trace_logging(config: &LogConfig) -> Result<LogGuard> {
    // 1. 原子检查单例
    if TRACING_LOGGER_IN_USE
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return Err(anyhow::anyhow!("Logger is already initialized"));
    }

    let mut guards = Vec::new();

    // --- A. 控制台 Layer ---
    let console_layer = if matches!(config.mode, LogMode::Console | LogMode::Both) {
        let (nb_stdout, guard) = tracing_appender::non_blocking(std::io::stdout());
        guards.push(guard);

        Some(build_formatted_layer(nb_stdout, config)
            .with_filter(EnvFilter::from(config.stdout_level.to_lowercase())))
    } else {
        None
    };

    // --- B & C. 文件 Layer ---
    let (file_layer, error_file_layer) = if matches!(config.mode, LogMode::File | LogMode::Both) {
        if let Some(fw_config) = &config.file_writer {
            // 闭包：根据文件名创建 Appender
            let mut create_appender = |name: String| {
                let appender = match fw_config.rotate.rotation {
                    RotationRule::Daily => tracing_appender::rolling::daily(&fw_config.directory, name),
                    RotationRule::Hourly => tracing_appender::rolling::hourly(&fw_config.directory, name),
                    RotationRule::Never => tracing_appender::rolling::never(&fw_config.directory, name),
                };
                let (nb, guard) = tracing_appender::non_blocking(appender);
                guards.push(guard);
                nb
            };

            // 创建主日志层
            let main_nb = create_appender(fw_config.file_name.clone());
            let f_layer = build_formatted_layer(main_nb, config)
                .with_filter(EnvFilter::from(config.file_level.to_lowercase()));

            // 创建错误日志层
            let err_nb = create_appender(format!("error.{}", fw_config.file_name));
            let e_layer = build_formatted_layer(err_nb, config)
                .with_filter(EnvFilter::from("error"));

            (Some(f_layer), Some(e_layer))
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    // --- D. 注册 ---
    let subscriber = Registry::default()
        .with(console_layer)
        .with(file_layer)
        .with(error_file_layer);

    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set global subscriber")?;

    Ok(LogGuard { _worker_guards: guards })
}

// --- 3. 单元测试模块 ---
/// 根目录下执行:cargo test --lib tracing::tests
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::fs;
    use tempfile::tempdir;
    use tracing::{debug, error, info};

    #[fixture]
    fn base_config() -> (tempfile::TempDir, LogConfig) {
        let dir = tempdir().expect("Failed to create temp dir");
        let config = LogConfig {
            mode: LogMode::Both,
            encoding: LogEncoding::Plain,
            time_format: "%Y-%m-%d %H:%M:%S%.3f %Z".to_string(),
            stdout_level: "DEBUG".to_string(),
            file_level: "DEBUG".to_string(),
            is_colored: true,
            print_config: true,
            file_writer: Some(FileWriterConfig {
                directory: dir.path().to_str().unwrap().to_string(),
                file_name: "test_app.log".to_string(),
                rotate: FileRotateConfig {
                    rotation: RotationRule::Never,
                    max_size_mb: 10,
                    max_backup_count: 1,
                    keep_days: 1,
                },
            }),
        };

        (dir, config)
    }

    #[rstest]
    #[case(100)] // 测试发送 100 条日志
    fn test_log_dispatching(base_config: (tempfile::TempDir, LogConfig), #[case] count: usize) {
        let (dir, config) = base_config;
        println!(
            "Console filter: {}",
            config.stdout_level.clone().to_string()
        );

        // 必须持有 guard 才能保证日志刷入磁盘
        let guard = init_trace_logging(&config);
        assert!(guard.is_ok(), "First initialization should succeed");

        let second_init = init_trace_logging(&config);
        assert!(second_init.is_err(), "Second initialization should fail");
        let guard = guard.expect("initialization should succeed");

        // 发送混合级别的日志
        for i in 0..count {
            info!("This is info log number {}", i);
            error!("This is error log number {}", i);
            debug!("This is debug log number {}", i); // 应该进入全量文件，不进控制台（根据级别设置）
        }

        // 手动释放 guard 会触发 WorkerGuard 的 Drop，从而 Flush 日志线程
        drop(guard);

        // 检查文件是否存在
        let main_log_path = dir.path().join("test_app.log");
        let error_log_path = dir.path().join("error.test_app.log");

        assert!(main_log_path.exists(), "Main log file should be created");
        assert!(error_log_path.exists(), "Error log file should be created");

        // 读取内容验证分流
        let main_content = fs::read_to_string(main_log_path).expect("Read main log failed");
        let error_content = fs::read_to_string(error_log_path).expect("Read error log failed");

        // 验证分流逻辑
        assert!(
            main_content.contains("INFO"),
            "Main log should contain INFO level"
        );
        assert!(
            main_content.contains("DEBUG"),
            "Main log should contain DEBUG level"
        );

        assert!(
            error_content.contains("ERROR"),
            "Error log should contain ERROR level"
        );
        assert!(
            !error_content.contains("INFO"),
            "Error log should NOT contain INFO level"
        );
    }
}
