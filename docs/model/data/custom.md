# CustomData：移除 `python` feature 绑定

源码：[`crates/model/src/data/custom.rs`](../../../crates/model/src/data/custom.rs)

## 背景

`nice-model` 的 `Cargo.toml` 没有声明 `python` feature（当前 features 只有 `arrow` / `test-support` / `high-precision` / `defi`）。因此 `custom.rs` 中所有 `#[cfg(feature = "python")]` 代码从未编译，属于死代码。

## 已删除内容

从 `custom.rs` 移除：

| 符号 | 作用 |
| --- | --- |
| `PythonCustomDataWrapper` | 把 Python 对象包装成 `Arc<dyn CustomDataTrait>` |
| `intern_type_name_static` | 类型名 intern，供 Python wrapper 的 `'static` 返回值 |
| `register_python_data_class` / `get_python_data_class` | Python 自定义数据类型注册表 |
| `reconstruct_python_custom_data` | 从 JSON 重建 Python 自定义数据 |
| `clone_pyclass_to_pyobject` | `#[pyclass]` 转 `Py<PyAny>` |
| `CustomDataTrait::to_pyobject` | trait 上的 PyO3 转换钩子 |
| `CustomDataTrait::to_json_py` | Python 侧 JSON 序列化（默认转发给 `to_json`） |
| `#[pyclass]` / `gen_stub_pyclass` | `CustomData` 的 PyO3 绑定属性 |

同步清理 [`crates/model/src/data/mod.rs`](../../../crates/model/src/data/mod.rs) 中对这些符号的 `pub use`，避免留下悬空再导出。

## 保留的 Rust 路径

`CustomData` / `CustomDataTrait` 仍是纯 Rust 自定义数据入口：

- `register_custom_data_json` / `ensure_custom_data_json_registered`：JSON 反序列化注册
- `CustomData::from_arc` / `CustomData::new`：构造
- `CustomData::from_json_bytes` + envelope `{ type, data_type, payload }`：JSON 往返

## 验证

```bash
cargo test -p nice-model --lib data::custom --offline
```

结果：`test_custom_data_wrapper`、`test_custom_data_json_roundtrip` 均通过。

## 关于「IDE 工具不可用」

RustRover 确实打开了本仓库和 `custom.rs`。上次搜引用没用 `mcp__idea__*`，不是因为文件没打开，而是 **`mcp__idea__*` 连的是 IntelliJ，不是 RustRover**。

完整原因见 [`docs/ide-mcp.md`](../../ide-mcp.md)。
