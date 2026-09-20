# 阶段 0 验收说明 v0.1

## 术语表与命名约定

| 规范名称 | English / Acronym | 在本说明中的职责边界 | 不代表什么 |
| --- | --- | --- | --- |
| 阶段 0 规范 | Stage 0 Specification | 冻结文件格式、容量、状态转换和错误引用的机器可读记录 | 不代表存储引擎已经完成 |
| 故障测试矩阵 | Fault Test Matrix | 记录故障注入点、预期结果和恢复动作 | 不代表已经执行所有运行时故障 |
| 稳定规范摘要 | Stable Specification Summary | 对规范文件和故障矩阵字节计算的稳定哈希和计数 | 不代表密码学签名 |

## 验收入口

从 workspace 根目录运行：

```powershell
cargo run -p lattice-cli -- validate-stage-0
```

通过时至少输出：

```text
stage_0=pass
spec_version=0.1
fault_cases=16
transition_rules=5
spec_hash=<16 hexadecimal characters>
```

验证器读取并检查：

- `frozen-spec-v0.1.toml` 的版本、编码、容量、帧类型、错误码、事件结果和持久化规则。
- `fault-matrix-v0.1.toml` 的 16 个唯一案例、7 类故障、状态结果和错误码引用。
- `local_committed -> hub_received -> hub_applied` 的五条允许状态转换。
- 当前 workspace 的存储、同步和传输边界验证。

失败时返回非零退出码，并输出具体字段、案例 ID 或引用错误。阶段 0 只能在以下命令全部通过后提交：

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
powershell -NoProfile -File scripts/check-coverage.ps1
powershell -NoProfile -File scripts/check-source-size.ps1
cargo run -p lattice-cli -- validate-stage-0
```

覆盖率清单把 `lattice-format/src/baseline.rs`、`lattice-format/src/stage_zero.rs`、`lattice-core/src/lib.rs`、`lattice-sync/src/lib.rs` 和 `lattice-transport/src/lib.rs` 作为阶段 0 关键模块，函数、行和区域阈值均为 100%。`lattice-cli` 仅负责启动装配和输出路由，`lattice-format/src/lib.rs` 仅负责模块声明与 API 重导出；两者不计入关键模块覆盖率，但仍由 workspace 测试和 CLI 验收覆盖。

本阶段没有数据库网络服务，因此不执行 Docker 集成测试；Docker 集成从阶段 3C 开始。阶段 0 的运行证据只证明规范和边界验证器可运行，不证明日志、事务或同步运行时已经完成。
