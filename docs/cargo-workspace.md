# Workspace 成员：能不能用 `crates/*`

**能。** 不必每加一个 crate 就在 `members` 里手写一行。Cargo 的 glob 只扩一层，且会匹配到没有 `Cargo.toml` 的目录；那一层必须 `exclude`。

## 结论写法

```toml
[workspace]
resolver = "2"
members = [
    #"bin/raptor",
    "crates",
    "crates/*",
    "crates/adapters/*",
    "crates/persistence/macros",
    "examples/tutorials",
]
exclude = ["crates/adapters"]
```

- 在 `crates/` 下新建带 `Cargo.toml` 的目录（例如 `crates/foo`）→ 自动进 workspace。
- 在 `crates/adapters/` 下新建适配器 crate → 自动进 workspace。
- 只有再深一层、又不在现有 glob 里的包，才需要再加一条路径。

## 为什么不能只写 `crates/*`

`crates/*` 会匹配 `crates/` 下**所有一级子目录**，不管里面有没有 `Cargo.toml`。

当前 `crates/` 里只有 `adapters/` 没有自己的 `Cargo.toml`（它只是分组目录）。只写 `crates/*` 时 Cargo 会去读 `crates/adapters/Cargo.toml`，失败后整个 workspace 加载失败。RustRover / rust-analyzer 就会报：

> The file does not belong to a known Cargo project

对 `crates/backtest/Cargo.toml` 也一样：不是 backtest 自己坏了，是根 workspace 没加载成功。

`exclude = ["crates/adapters"]` 把这个分组目录踢掉，glob 才能用。

## 各条路径各自管什么

`crates`（没有 `*`）是 `crates/Cargo.toml` 这个包本身（`nice-trader`）。`crates/*` 匹配不到这个文件。

`crates/*` 只扩一层，覆盖：

- `crates/analysis`
- `crates/backtest`
- `crates/cli`
- …以及其它带 `Cargo.toml` 的一级 crate

匹配不到：

- `crates/adapters/binance` 这类二级包 → 用 `crates/adapters/*`
- `crates/persistence/macros` 这类三级包 → 必须写死路径（或再加对应 glob）

`crates/adapters/*` 同样只扩一层，覆盖 `architect_ax`、`binance` 等。不会把 `crates/adapters/lighter/fuzz/pornin` 收进来；那个是 fuzz 子工程，本来也不该进根 workspace。

## 和手写清单的差别

手写 `crates/backtest`、`crates/adapters/binance`……：新 crate 必须改 `Cargo.toml`，漏写就进不了 workspace。

改成 glob + `exclude`：一级 crate 和 adapters 下的 crate 自动收录。代价是必须永远 `exclude` 那个没有 manifest 的 `crates/adapters` 分组目录。

不要用 `crates/**`：Cargo workspace 成员不支持递归 `**`。

## 曾经会让整个 workspace 挂掉的另一件事

`examples/tutorials` 一度还在 `workspace = true` 继承 `nautilus-common` 等旧名，根 `[workspace.dependencies]` 里是 `nice-*`。成员解析失败时，IDE 同样会认为任何 crate（包括 backtest）都不属于已知 Cargo project。

当前仓库源码、`Cargo.toml`、Rust/Python 里已经没有 `nautilus-*` 依赖名。`examples/tutorials/Cargo.toml` 包名是 `nice-tutorials`，依赖是 `nice-common` / `nice-core` / `nice-databento` 等。那条报错如果还在，不是旧名残留。
