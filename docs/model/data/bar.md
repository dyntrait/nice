# jiff 0.2 用法（对照 `bar.rs`）

源码：[`crates/model/src/data/bar.rs`](../../../crates/model/src/data/bar.rs)

本仓库 workspace 钉的是 **jiff 0.2.35**（`default-features = false`，打开 `std` / `serde` / `perf-inline` / `tz-fat` / `tzdb-bundle-always`）。上游最新是 **0.2.36**（2026-09-12）：修了 `Offset` 的 checked 减法，并加了可选 `arbitrary`。1.0 还没发，API 仍以 0.2 为准。

官方入口：[docs.rs/jiff](https://docs.rs/jiff/)、[GitHub](https://github.com/BurntSushi/jiff)、[CHANGELOG](https://github.com/BurntSushi/jiff/blob/master/CHANGELOG.md)。

`bar.rs` 只引进了 K 线对齐真正用到的四个类型：

```rust
use jiff::{SignedDuration, Timestamp, civil::Date, tz::Offset};
```

月线算术走 [`crates/core/src/datetime.rs`](../../../crates/core/src/datetime.rs) 的 `add_n_months` / `subtract_n_months`，内部用 `Span` + `TimeZone::UTC`。

## 先分清五类时间

jiff 学 [Temporal](https://tc39.es/proposal-temporal/docs/index.html)，故意把「瞬间」和「墙上时钟」拆开：

| 类型 | 是什么 | `bar.rs` 里怎么用 |
| --- | --- | --- |
| `Timestamp` | Unix 纪元起的纳秒瞬间，本身没有时区 | `now`、K 线起点 |
| `civil::Date` / `DateTime` | 日历日期/墙上时间，没有时区 | 取周一、取当天 00:00、取 1 月 1 日 |
| `tz::Offset` | 固定偏移（UTC 就是 `+00`） | `Offset::UTC.to_datetime` / `to_timestamp` |
| `tz::TimeZone` / `Zoned` | IANA 时区 + DST | K 线对齐全程 UTC，不用 `Zoned`；加月在 `datetime.rs` 里用 |
| `SignedDuration` vs `Span` | 物理时长 vs 日历跨度 | ms/s/min/hour/day 用前者；星期回退、加月用后者 |

选错类型是 jiff 里最常见的坑：`Timestamp + 1.month()` 在 0.2 里要先变成 `Zoned`，因为「一个月」不是固定纳秒。

## `Timestamp`：瞬间

```rust
use jiff::Timestamp;

let t: Timestamp = "2022-07-20T20:34:56.123Z".parse()?; // bar.rs 测试 helper
let t = Timestamp::new(1_658_349_296, 123_000_000)?;    // 秒 + 纳秒
let t = Timestamp::from_nanosecond(i128::from(nanos))?;
let t = Timestamp::now();
let t = Timestamp::UNIX_EPOCH;

t.as_nanosecond();          // i128
t.subsec_nanosecond();
t.duration_until(later);    // -> SignedDuration
```

`UnixNanos::to_datetime_utc()` 就是 `Timestamp::from_nanosecond`。打印带偏移用 `display_with_offset(Offset::UTC)`，不是默认 `Display`（默认按 UTC 的 RFC 3339，但精度控制要自己 format）。

0.2.34 起 `strftime` 不再 panic，非法格式会原样穿过。

## `civil` + `Offset`：墙上时钟（`bar.rs` 主路径）

K 线边界按 **UTC 日历日** 对齐，所以用固定 `Offset::UTC`，不走 IANA：

```rust
let now_civil = Offset::UTC.to_datetime(now);          // Timestamp -> civil DateTime
let midnight = now_civil.date().at(0, 0, 0, 0);        // Date -> DateTime（时分秒纳秒）
let day_start = Offset::UTC.to_timestamp(midnight)?;   // 再变回 Timestamp
```

周线：先看今天是周几，再用 **日历** `Span` 回到周一 00:00：

```rust
let days_from_monday = i64::from(now_civil.weekday().to_monday_zero_offset());
let week_start_date = now_civil
    .date()
    .checked_sub(jiff::Span::new().days(days_from_monday))?;
let start = Offset::UTC.to_timestamp(week_start_date.at(0, 0, 0, 0))?;
```

年线/月线起点用 `Date::new(year, 1, 1)?.at(0, 0, 0, 0)`。`year` 在 jiff 里是 `i16`，超出范围会失败——`test_get_time_bar_start_year_step_exceeds_jiff_range_panics` 就是在测这个。

等价的新写法（本文件没采用，但 0.2 文档推荐）：

```rust
use jiff::civil::date;
date(2022, 7, 20).at(20, 34, 56, 123_000_000);
```

## `SignedDuration`：物理间隔

像带符号的 `std::time::Duration`，单位只有纳秒。适合 ms/s/min/hour，以及把「N 天」换成 24N 小时（K 线比较长度用的 proxy）：

```rust
SignedDuration::from_millis(step)
SignedDuration::from_secs(step)
SignedDuration::from_mins(step)
SignedDuration::from_hours(step)
SignedDuration::try_from_hours(days * 24)?   // 天/周的物理长度
SignedDuration::from_nanos_i128(n)
SignedDuration::ZERO
```

`find_closest_smaller_time` 的核心是：

1. 当天 UTC 0 点 + `origin_offset` 得到 `base_time`
2. `base_time.duration_until(now)` 得到有符号物理差
3. `div_euclid(period_ns)` 向下取整到上一根边界
4. `base_time + SignedDuration::from_nanos_i128(...)`

`Timestamp` 加减 `SignedDuration` 是瞬间平移，**不管 DST**。对 UTC K 线这是对的。

0.2.35 修过时长解析符号：`"-PT0.5S"` / `"-0.5s"` 以前会丢负号。本仓库已钉在含该修复的版本。

## `Span`：日历跨度

`Span` 可以混 years/months/days/hours，长度取决于相对日期。官方惯用 [`ToSpan`](https://docs.rs/jiff/latest/jiff/trait.ToSpan.html)：

```rust
use jiff::ToSpan;
1.month().hours(2)
Span::new().days(3)
Span::new().try_months(n)?
```

`datetime.rs` 加月：

```rust
datetime.to_zoned(TimeZone::UTC).checked_add(Span::new().try_months(months)?)?.timestamp()
```

这里必须 `to_zoned`：`Timestamp` 自己不知道「一个月」落在哪一天。

不能把带年月的 `Span` 直接 `try_into` 成 `SignedDuration`（没有相对日期，jiff 会拒绝）。

## `Zoned`：带时区的完整时间

官方入门例子（[README](https://github.com/BurntSushi/jiff)）：

```rust
use jiff::{Timestamp, ToSpan};

let time: Timestamp = "2024-07-11T01:14:00Z".parse()?;
let zoned = time.in_tz("America/New_York")?.checked_add(1.month().hours(2))?;
assert_eq!(zoned.to_string(), "2024-08-10T23:14:00-04:00[America/New_York]");
```

`Zoned::now()` 用系统时区。本仓库为了确定性，用 `tzdb-bundle-always` 打进二进制，通过 `get_timezone(name)` 读捆绑 IANA 库，不用主机 `/usr/share/zoneinfo`。

K 线对齐全程 UTC，所以 `bar.rs` 用 `Offset::UTC` 就够；只有加月/加年这种日历运算才升到 `TimeZone::UTC`。

## 和本仓库时间类型的换算

```text
UnixNanos  --from_nanosecond-->  Timestamp  --Offset::UTC.to_datetime-->  civil::DateTime
     ^                              |                                         |
     +---- as_nanosecond -----------+                    date() / at() / weekday()
```

- `UnixNanos` / `DurationNanos` 是交易域的 `u64` 纳秒，不允许负
- jiff 的 `Timestamp` / `SignedDuration` 是 `i128`/`i64` 有符号
- 越界用 `u64::try_from`；`Timestamp::MAX` 转 `UnixNanos` 会失败（`try_datetime_to_unix_nanos`）

解析 RFC 3339 时，`UnixNanos` 故意比 jiff 更窄：不要 RFC 9557 的 `[America/New_York]` 注解，只接受 `T`/`t`/空格分隔。

## 0.2 里值得记住的最新习惯

1. **瞬间用 `Timestamp`，墙上时钟用 `civil`，带 DST 用 `Zoned`。** 不要把 chrono 的 `DateTime<Utc>` 思维套过来。
2. **物理间隔 `SignedDuration`，日历间隔 `Span`。** `bar.rs` 的 ms–day 走前者，周回退和月线走后者。
3. **用 `checked_add` / `checked_sub`，不要默认 overflow panic。** 日历运算会撞 jiff 年份范围。
4. **固定偏移用 `Offset`，IANA 用 `TimeZone`。** UTC 对齐选 `Offset::UTC` 更轻。
5. **整数构造优先 `try_*`：** `try_from_mins` / `try_from_hours` / `try_months`。`from_mins` 在过大时会 panic。
6. 打印：`Zoned` 的 `Display` 是 RFC 9557（带 `[Zone]`）；只要 RFC 3339 就用 `zoned.timestamp()` 或 `display_with_offset`。

## 版本

| 位置 | 版本 |
| --- | --- |
| 本仓库 `Cargo.toml` | 0.2.35 |
| crates.io 最新（2026-09-12） | 0.2.36 |

0.2.36 对 `bar.rs` 无必须升级点（`arbitrary` 和 `Offset` 减法修复）。升级只需改 workspace 版本号。
