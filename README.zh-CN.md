# sortlab

> 一个可扩展的 Rust 排序算法实验库，提供统一的 API。

[English documentation](README.md)

## 目标

- **可插拔策略**：所有算法实现同一个 [`Sorter`] trait。
- **迭代器适配器**：任何迭代器都可以直接 `.collect_sorted()` 得到有序结果。
- **教学导向**：实现清晰易读，优先于极限性能。
- **逐步成长**：先搭好可编译的骨架，再逐个替换为真实算法。

## 快速开始

```bash
cargo add sortlab
```

```rust
use sortlab::prelude::*;

fn main() {
    let mut data = vec![5, 2, 8, 1, 9, 3];
    Bubble.sort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 5, 8, 9]);

    let sorted = vec![3, 1, 2].into_iter().collect_sorted();
    assert_eq!(sorted, vec![1, 2, 3]);
}
```

开启 `parallel` feature 可使用多线程归并排序：

```toml
[dependencies]
sortlab = { version = "0.1", features = ["parallel"] }
```

```rust
use sortlab::prelude::*;

let mut data: Vec<i64> = /* 大切片 */;
ParallelMerge.sort(&mut data); // 跨 std 线程切分
```

## 目录结构

```
├── Cargo.toml
├── README.md
├── README.zh-CN.md   # 中文文档
├── src
│   ├── lib.rs          # crate 根与重导出
│   ├── prelude.rs      # 一行导入所有常用项
│   ├── traits.rs       # Sorter trait 与 IterAdaptor
│   ├── algorithms
│   │   ├── mod.rs
│   │   ├── bubble.rs   # 冒泡排序
│   │   ├── quick.rs    # 快速排序
│   │   ├── merge.rs    # 归并排序
│   │   └── shell.rs    # 希尔排序
│   └── utils
│       ├── mod.rs
│       └── helpers.rs  # is_sorted 等工具函数
```

## 算法一览

| 算法 | 平均复杂度 | 最坏复杂度 | 空间 | 稳定性 |
|------|-----------|-----------|------|--------|
| 冒泡排序 | O(n²) | O(n²) | O(1) | 稳定 |
| 希尔排序 | 取决于间隔序列 | O(n²) | O(1) | 不稳定 |
| 归并排序 | O(n log n) | O(n log n) | O(n) | 稳定 |
| 快速排序 | O(n log n) | O(n²) | O(log n) | 不稳定 |

## 路线图

- [x] `Sorter` trait 与 `IterAdaptor` blanket 实现
- [x] 真实算法实现：冒泡、快排（Lomuto + 三数取中）、归并（稳定、O(n) 辅助空间）、希尔（3x+1 间隔）
- [x] 对照 `slice::sort` 的随机压力测试
- [x] `parallel` feature：基于 `std::thread::scope` 的 `ParallelMerge`，零第三方依赖
- [ ] 基准测试

## 许可证

MIT。
