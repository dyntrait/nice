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

use anyhow::Result;
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
    Layer,
    EnvFilter,
    Registry,
    layer::SubscriberExt,
    util::SubscriberInitExt
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
    #[serde(flatten)]
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
        let metadata = event.metadata(); // 这个结构体包含了编译器在编译阶段就能确定的、关于这条日志产生位置的所有静态信息

        // 2. 【核心修改】使用保存在 self 中的时间格式
        // 这里使用 .format(&self.time_format) 动态解析
        let now = chrono::Local::now();
        write!(writer, "{} ", now.format(&self.time_format))?; // 右对齐，占 5 个字符宽度

        // --- 以下逻辑保持不变 ---
        let level = metadata.level();
        write!(writer, "{:>5} ", level)?;

        if let (Some(file), Some(line)) = (metadata.file(), metadata.line()) {
            let short_file = file.rfind('/').map(|i| &file[i + 1..]).unwrap_or(file);
            write!(writer, "[{}:{}] ", short_file, line)?;
        }else{
            write!(writer, "[{}] ", metadata.target())?;
        }

        ctx.format_fields(writer.by_ref(), event)?;//info!("message", key = value) 里的 message 和 key = value 渲染出来。它调用了默认的字段格式化器
        writeln!(writer)
    }
}

/// --- 自定义格式器：JSON 模式，输出 {"file_line": "short_file:line", "message": "..."} ---


use tracing::field::{Field, Visit};

// 1. 修改 Visitor 的定义：让它直接持有一个能写的地方
struct JsonVisitor<'a, 'b> {
    // Writer<'_> 本身就是一个包装引用，我们直接持有它的可变借用
    writer: &'a mut Writer<'b>,
    is_first: bool,
    result: std::fmt::Result, // 增加一个字段记录写入过程中的错误
}

impl<'a, 'b> Visit for JsonVisitor<'a, 'b> {
    // 1. 处理基础类型（数字、布尔不加引号）
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.write_pair(field.name(), value.to_string(), false);
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.write_pair(field.name(), value.to_string(), false);
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.write_pair(field.name(), value.to_string(), false);
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        // 关键逻辑：探测字符串是否为有效的 JSON 对象或数组
        let is_json = (value.starts_with('{') && value.ends_with('}')) ||
            (value.starts_with('[') && value.ends_with(']'));

        self.write_pair(field.name(), value.to_string(), !is_json);
    }

    fn record_error(&mut self, field: &Field,  value: &(dyn std::error::Error + 'static)) {
        self.write_pair(field.name(), value.to_string(), false);
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        let debug_str = format!("{:?}", value);
        // 探测 Debug 输出是否已经是合法的 JSON 格式
        let is_json = (debug_str.starts_with('{') && debug_str.ends_with('}')) ||
            (debug_str.starts_with('[') && debug_str.ends_with(']'));

        self.write_pair(field.name(), debug_str, !is_json);
    }
}

impl<'a, 'b> JsonVisitor<'a, 'b> {
    // 统一写入逻辑：need_quotes 控制是否包裹双引号
    fn write_pair(&mut self, name: &str, value: String, need_quotes: bool) {
        if self.result.is_err() { return; }
        let prefix = if self.is_first { "" } else { "," };

        if need_quotes {
            // 普通字符串：加引号
            self.result = write!(self.writer, "{}\"{}\":\"{}\"", prefix, name, value);
        } else {
            // 数字、布尔、或已经是 JSON 的内容：不加引号
            self.result = write!(self.writer, "{}\"{}\":{}", prefix, name, value);
        }
        self.is_first = false;
    }
}

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
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let metadata = event.metadata();

        // 1. 获取自定义格式的时间戳
        let now = chrono::Local::now();

        // 2.优化文件名提取：避免多次 slice 检查
        let file_info = metadata.file().map(|f| {
            let i = f.rfind('/').map(|idx| idx + 1).unwrap_or(0);
            &f[i..]
        }).unwrap_or("unknown");
        let line = metadata.line().unwrap_or(0);

        // 3. 开始构建 JSON (直接写入 writer，零中间 String 分配)
        // 包含时间戳、日志级别、文件位置
        write!(
            writer,
            "{{\"time\":\"{}\",\"level\":\"{}\",\"file\":\"{}:{}\"", // 注意末尾是逗号
            now.format(&self.time_format),
            metadata.level(),
            file_info,
            line
        )?;
        //ctx.format_fields(writer.by_ref(), event)?;
        // 重点：使用我们的 Visitor 遍历所有字段（包括 message）
        // 初始化 Visitor，直接把局部变量 writer 的可变引用传进去
        let mut visitor = JsonVisitor {
            writer: &mut writer, // 这里的生命周期现在是正确的
            is_first: false,
            result: Ok(()),
        };
        // 运行访问者
        event.record(&mut visitor);

        // 检查访问过程中是否有错误
        visitor.result?;

        // 5. 闭合 JSON 结构
        writeln!(writer, "}}")
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
    // 1. 原子检查单例：确保全局唯一初始化
    if TRACING_LOGGER_IN_USE
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return Err(anyhow::anyhow!("Logger is already initialized"));
    }

    let mut worker_guards = Vec::new();

    // --- A. 控制台输出层 ---
    let console_layer = if matches!(config.mode, LogMode::Console | LogMode::Both) {
        let (stdout_writer, guard) = tracing_appender::non_blocking(std::io::stdout());
        worker_guards.push(guard);

        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(&config.stdout_level));

        Some(build_formatted_layer(stdout_writer, config).with_filter(filter))
    } else {
        None
    };

    // --- B & C. 文件输出层 ---
    let (primary_file_layer, error_file_layer) = if matches!(config.mode, LogMode::File | LogMode::Both) {
        if let Some(file_config) = &config.file_writer {

            // 闭包：使用 &str 避免 String 的分配开销
            let mut create_writer = |file_suffix: &str| {
                let full_name = if file_suffix.is_empty() {
                    file_config.file_name.clone()
                } else {
                    format!("{}.{}", file_suffix, file_config.file_name)
                };

                let appender = match file_config.rotate.rotation {
                    RotationRule::Daily => tracing_appender::rolling::daily(&file_config.directory, full_name),
                    RotationRule::Hourly => tracing_appender::rolling::hourly(&file_config.directory, full_name),
                    RotationRule::Never => tracing_appender::rolling::never(&file_config.directory, full_name),
                };

                let (writer, guard) = tracing_appender::non_blocking(appender);
                worker_guards.push(guard);
                writer
            };

            // 1. 构建主日志层 (Primary Layer)
            let primary_writer = create_writer("");
            let primary_filter = EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(&config.file_level));

            let primary_layer = build_formatted_layer(primary_writer, config)
                .with_filter(primary_filter);

            // 2. 构建错误日志层 (Error-only Layer)
            let error_writer = create_writer("error");
            let error_layer = build_formatted_layer(error_writer, config)
                .with_filter(EnvFilter::from("error"));

            (Some(primary_layer), Some(error_layer))
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    // --- D. 注册全局订阅者 ---
    let subscriber = Registry::default()
        .with(console_layer)
        .with(primary_file_layer)
        .with(error_file_layer);

    subscriber.try_init()
        .map_err(|e| anyhow::anyhow!("Failed to set global subscriber: {}", e))?;

    Ok(LogGuard { _worker_guards: worker_guards })
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
