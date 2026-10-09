# `mcp__idea__*` 为什么连到 Java 仓，而不是 RustRover 的 nice

结论分两层：

1. **当前聊天已经跟 IDE 打开的工程走。** 本会话由 RustRover ACP 拉起，工作目录是 `/home/zh/tools/project/nice`。
2. **`mcp__idea__*` 不会跟。** 它走 Claude 全局 MCP 里写死的 HTTP 地址，当前指向 **IntelliJ IDEA** 上的 Java 三仓。

## 现象

在 RustRover 打开 `/home/zh/tools/project/nice`，并打开 `crates/model/src/data/custom.rs` 时，`mcp__idea__get_all_open_file_paths` 仍返回：

- `/home/zh/project/java/spot/ryzen-fund`
- `/home/zh/project/java/spot/ryzen-biz`
- `/home/zh/project/java/spot/ryzen-admin`

把 `projectPath=/home/zh/tools/project/nice` 传进去会报 `doesn't correspond to any open project`。

## 原因

本机同时跑着两个 JetBrains IDE，各自有独立 MCP HTTP 端口：

| 进程 | MCP HTTP | Claude 里的名字 | 打开的工程 |
| --- | --- | --- | --- |
| IntelliJ IDEA (`idea` pid 12839) | `127.0.0.1:64342` | 全局 `idea` | Java 三仓 |
| RustRover (`rustrover` pid 331409) | `127.0.0.1:64622` | **未接入** | `nice` |

Claude 全局配置 `~/.claude.json` 的 `mcpServers` 是：

```json
{
  "pycharm": { "url": "http://127.0.0.1:64462/stream", "type": "http" },
  "idea":    { "url": "http://127.0.0.1:64342/stream", "type": "http" }
}
```

没有 `rustrover` 条目。`idea` 这个名字对应 **IntelliJ 的 64342**，不是「当前 IDE」。

两条集成通道是分开的：

| 通道 | 本会话实际连到谁 |
| --- | --- |
| ACP（RustRover 里的 Claude 聊天、工作目录、打开文件） | RustRover + `nice` |
| MCP `idea`（`mcp__idea__search_*` / `get_all_open_file_paths`） | IntelliJ + 三个 Java 仓 |

所以：你在 RustRover 里看 `custom.rs` 完全正确；调用 `mcp__idea__*` 时却在问 IntelliJ「你现在打开了什么工程」。

端口还会变：JetBrains MCP 从 64342 起找空闲端口，[不能在 UI 里固定](https://youtrack.jetbrains.com/articles/SUPPORT-A-3223)。IntelliJ 先启动占了 64342，RustRover 后启动就落到 64622。

## 能不能跟 IDE 打开的项目走？

**不能靠现有的 `mcp__idea__*` 自动跟。** Claude 在会话启动时把 MCP URL 绑死，运行中不会看「当前焦点 IDE」再切端口。

能跟的只有：

| 想跟什么 | 现在能不能 | 怎么做 |
| --- | --- | --- |
| 聊天、cwd、@ 文件 | 能，已经在跟 | 继续从 RustRover 开 Claude |
| `search_symbol` / `analyze_calls` 这类 IDE 工具 | 不能自动跟 | 给 RustRover MCP 单独接到 Claude，然后用新会话 |
| 按 cwd 在 IntelliJ / RustRover 之间热切换同一个 `mcp__idea__*` | 不能 | 没有这层路由；硬改 `idea` URL 会把 Java 仓弄断 |

另外：RustRover 的 `64622` 现在是 **restricted mode**，裸 HTTP 返回 `401`（`Please, provide valid authorization token`）。IntelliJ 开了 Brave Mode，所以现在的 `idea` 能直接连。

## 要把 IDE 工具接到 nice（推荐）

在 **RustRover** 里操作，不要改 IntelliJ 那条 `idea`：

1. **Settings → Tools → MCP Server**，打开 **Enable MCP Server**。
2. 点 **Auto-Configure Claude Code**（或复制 HTTP Stream 配置）。官方说明：[IntelliJ MCP Server](https://www.jetbrains.com/help/idea/mcp-server.html)。
3. 让它**新增**名为 `rustrover` 的 MCP，URL 指向当前端口（此刻是 `http://127.0.0.1:64622/stream`）。**不要**把现有 `idea` 改成 64622，否则 Java 仓的 `mcp__idea__*` 会失效。
4. 若仍是 restricted mode：把 UI 里的 authorization token 写进该 MCP 的 `headers.Authorization`；或者像 IntelliJ 一样打开 Brave Mode（无确认执行终端/运行配置，面更大）。
5. **新开一轮 Claude 对话。** 本会话已经把 `mcp__idea__*` 绑在 64342，改配置不会热切换。
6. 新会话里应出现 `mcp__rustrover__*`，再传 `projectPath=/home/zh/tools/project/nice`。

未完成第 5 步之前，本仓库继续用本地 `Read` / `rg` / `cargo`，不要用 `mcp__idea__*` 搜 nice。

## 不要做的

- 不要把全局 `idea` 的 URL 改成 RustRover 端口（Java 三仓会断）。
- 不要把 MCP token 写进仓库里的 `.mcp.json`。
- 不要假设端口永远是 64622。
