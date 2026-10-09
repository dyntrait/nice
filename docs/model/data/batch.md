# DataBatch / BatchView：同质行情批量视图

源码：[`crates/model/src/data/batch.rs`](../../../crates/model/src/data/batch.rs)

这个文件解决的问题：行情经常是**一整批同一种类型**（一串 `QuoteTick`、一串 `TradeTick`），如果先打成 `Vec<Data>`，会把每种结构塞进同一个大枚举，浪费空间、克隆也贵。`BatchView` / `DataBatch` 把「共享一块连续内存」和「按类型分变体」拆开。

对照 [`crates/model/src/data/mod.rs`](../../../crates/model/src/data/mod.rs)：

| 类型 | 所有权 | 用途 |
| --- | --- | --- |
| `Data` | 拥有一条异构数据 | 单条分发，不适合大批量 |
| `DataRef<'a>` | 借一条异构数据 | 不拷贝地读一条 |
| `BatchView<T>` | 共享 `Vec<T>` 的一段 | 同质切片，克隆不拷元素 |
| `DataBatch` | 按族保存 `BatchView<具体类型>` | 同质批量 + 用 `DataRef` 逐条取出 |

## `BatchView<T>`：共享 `Vec<T>` 上的区间

```text
Arc<Vec<T>>  ──►  [ 0 | 1 | 2 | 3 | 4 | 5 ]
                       ^range.start     ^range.end
```

结构体只有两块：

- `data: Arc<Vec<T>>`：整段 backing，引用计数共享
- `range: Range<usize>`：当前视图覆盖的半开区间 `[start, end)`

故意用 `Arc<Vec<T>>` 而不是 `Arc<[T]>`（clippy 的 `rc_buffer` 被关掉）：调用方还能通过 `arc()` 把整段 `Vec` 拿回去。

### 构造

- `new(arc, range)`：指定区间；`start > end` 或 `end > len` 会 panic
- `full(arc)`：覆盖整段 `0..len`
- `From<Vec<T>>` / `From<Arc<Vec<T>>>`：包成全视图

### 便宜的 clone / slice

`Clone` 只 `Arc::clone` + 拷贝 `Range`，**不要求 `T: Clone`**。测试里用 `struct NonClone(i32)` 专门证明这一点。

`slice(start, end)` 的起止是**相对当前视图**：

```text
view  range = 1..4     元素 [20, 30, 40]
slice(1, 3)            落到 backing 的 2..4，元素 [30, 40]
```

切完后源视图可以 drop，切片仍握着同一块 `Arc`。

`Deref` / `AsRef<[T]>` 让视图当切片用：`view.len()`、`view[0]`、`view.get(i)` 都走 `&data[range]`。

### `make_mut`：写时复制

```rust
pub fn make_mut(&mut self) -> &mut [T]
where
    T: Clone
```

走 `Arc::make_mut`：

- 只有自己握着这块 backing：原地改，不拷整段 `Vec`
- 还有别的视图共享：先克隆整段 `Vec`，再改**当前 range**

测试 `test_batch_view_make_mut_clones_shared_backing_within_range` 说明副作用：

- 源视图 `1..4` 仍是 `[3, 1, 2]`
- 可变视图排序后是 `[1, 2, 3]`
- backing 下标 0 的 `9` 跟着被拷走，变成 `[9, 1, 2, 3]`

也就是：**COW 拷的是整段 allocation，不是只拷当前切片。**

## `DataBatch`：按族保存的同质批量

每个变体都是 `BatchView<具体类型>`：

- 盘口：`BookDelta` / `BookDeltas` / `BookDepth10`
- 成交与报价：`Quote` / `Trade` / `Bar`
- 衍生价格：`MarkPrice` / `IndexPrice` / `FundingRate` / `OptionGreeks`
- 状态：`InstrumentStatus` / `InstrumentClose`
- `defi` feature：`Defi(BatchView<DefiData>)`

**没有 `Custom`。** 注释写明：一份 `CustomData` 集合里可能混多种逻辑 `DataType`，没法当成单一静态族。

### 对外 API

- `len` / `is_empty`：转发给内部 `BatchView`
- `get(index) -> Option<DataRef<'_>>`：按变体把 `&T` 包成对应的 `DataRef`，供异构分发（读 `instrument_id`、`ts_init` 等），元素本身不拷
- `slice(start, end) -> Self`：保留同一个变体，只缩 range

`impl_data_batch_from_vec!` 给每种具体类型生成 `From<Vec<T>>`，所以 `DataBatch::from(vec![quote])` 会进 `Quote` 变体。

## 使用时要注意

1. 切片和 clone 很便宜，但**每个视图都钉住整段 `Vec`**。切出一小段后如果源批次很大、长期活着，内存不会按切片缩小。
2. `new` / `slice` 用 assert，非法区间是 panic，不是 `Result`。
3. `get` 越界返回 `None`，和切片 panic 策略不同。
4. 一批里只能是一种静态类型；异构流仍用 `Data` / `DataRef`。
