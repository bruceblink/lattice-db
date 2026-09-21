# Lattice DB

Lattice DB 是一个面向弱网边缘环境、从零实现的通用分布式数据库项目。

本项目采用 GNU Affero General Public License v3.0 或更高版本（SPDX：`AGPL-3.0-or-later`）。完整条款见 [LICENSE](LICENSE) 和 [GNU 官方许可文本](https://www.gnu.org/licenses/agpl-3.0.html)。

当前阶段只冻结需求和一致性边界，不引入 SQLite、Redis、MySQL、PostgreSQL 或 MongoDB 作为运行时数据库。它们只作为能力和行为的参考。

## 文档

- [需求与一致性说明 v0.1](docs/requirements-and-consistency-v0.1.md)
- [后续开发计划 v0.1](docs/development-plan-v0.1.md)
- [个人可用版每日开发计划 v0.1](docs/daily-development-plan-v0.1.md)
- [阶段 0 验收说明 v0.1](docs/stage-0/acceptance-v0.1.md)

## Workspace 结构

```text
crates/
├── lattice-format/     版本化协议和文件格式
├── lattice-core/       存储引擎边界
├── lattice-sync/       同步状态机和游标边界
├── lattice-transport/  传输适配边界
└── lattice-cli/        可运行验证入口
```

当前骨架的阶段 0 验证入口：

```powershell
cargo run -p lattice-cli -- validate-stage-0
```

成功输出 `stage_0=pass`。该入口验证版本、容量、标识符、序列/连续游标、文件布局、帧头布局、同步状态转换和传输边界，不代表数据库运行时已经实现。

提交前验证入口：

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
powershell -NoProfile -File scripts/check-coverage.ps1
powershell -NoProfile -File scripts/check-source-size.ps1
```

## 当前范围

- 统一数据库内核运行在边缘节点和同步中心。
- 首期面向写多读少、长时间离线、低带宽和高延迟链路。
- 首期支持单分区本地事务、追加变更日志、幂等同步和冲突记录。
- 首期不支持跨分区事务、跨节点强一致事务或完整 SQL 兼容。

当前仓库处于开发前冻结阶段，已经包含 workspace 构建入口和阶段 0 规范验证器，但尚未包含数据库运行时或集成测试服务。实现工作应先完成开发计划中的阶段 0，再进入日志和恢复切片。
