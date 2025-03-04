# Rust 函数提取工具

一个用于遍历指定目录下的 Rust 源代码文件，并提取所有函数/方法名称的命令行工具。

## 功能特性

- [x] 递归扫描目录结构
- [x] 支持普通函数提取
- [x] 支持 impl 块方法提取
- [x] 支持 trait 方法提取
- [x] 生成带层级结构的日志文件
- [x] 跨平台支持（Windows/Linux/macOS）

## 快速开始

### 前置要求

- Rust 1.60+
- Cargo

### 安装 & 使用

1. 准备配置文件：
   ```bash
   # 创建配置文件
   echo "/path/to/your/rust/project" > src/toRead.txt
   ```

2. 添加依赖项：
   ```toml
	[package]
	name = "extract_functions"
	version = "0.1.0"
	edition = "2021"
	[dependencies]	
	syn = { version = "2.0", features = ["full", "visit", "parsing"] }
	walkdir = "2.3"
	anyhow = "1.0"
   ```

3. 运行程序：
   ```bash
   cargo run --release
   ```

## 文件结构

```
.
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── toRead.txt       # 目标目录配置文件
└── fns_log              # 自动生成的输出文件
```

## 输出示例

```text
文件: src/module/example.rs
  - initialize
  - calculate_stats
  - validate_input

文件: src/utils/helpers.rs
  - format_string
  - parse_config
  - sanitize_input
```

## 配置说明

### toRead.txt 格式

纯文本文件，包含单个有效目录路径：
```text
/path/to/analyze
# 或 Windows 路径
C:\Rust\projects\demo
```

## 注意事项

1. 确保目标目录包含合法的 Rust 源文件（*.rs）
2. 输出文件会覆盖同名现有文件
3. 支持处理包含宏和条件编译的代码
4. 日志文件始终生成在目标目录根路径下

## 性能指标

测试环境：i7-11800H / 32GB RAM
```
| 代码规模 | 处理时间 |
| -------- | -------- |
| 10K LOC  | 0.8s     |
| 50K LOC  | 3.2s     |
| 100K LOC | 6.7s     |
```

## 开发文档

### 核心组件

1. **AST 解析器**：基于 `syn` 库的语法树分析
2. **目录遍历**：使用 `walkdir` 进行递归扫描
3. **函数访问器**：实现 `Visit` trait 的自定义遍历逻辑

### 扩展建议

- 添加忽略列表功能
- 支持输出格式化（JSON/XML）
- 实现函数调用关系分析

## 许可证

MIT License
Thinking...
> 
> 
> 好的，用户让我帮忙写“如何为本项目做贡献”的栏目内容。首先，我需要理解用户的项目是什么类型的，但目前信息有限。可能需要假设这是一个开源软件项目，常见的如GitHub上的仓库。用户可能需要一个标准的贡献指南，方便其他开发者参与。
> 
> 接下来，我应该考虑贡献指南通常包含哪些部分。通常包括开发环境搭建、代码风格、提交流程、Issue和PR的规范、测试要求等。需要结构清晰，步骤明确，让贡献者容易跟随。
> 
> 用户可能希望内容详尽但不过于复杂，适合不同层次的贡献者。可能需要分点列出，比如环境设置、代码风格、提交流程、报告问题、功能请求、文档改进等部分。每个部分下给出具体的步骤或要求。
> 
> 同时，要注意使用友好的语气，鼓励社区参与。可能还需要包括行为准则，比如遵守贡献者公约，保持尊重等。另外，许可证信息也是重要的一部分，确保贡献者了解代码的使用条款。
> 
> 另外，用户可能没有提到代码测试，但这是贡献指南中重要的一环，需要包含如何运行测试，以及贡献新测试的要求。还有，代码审查流程和分支管理策略也可能需要涉及，比如使用特性分支，PR指向主分支等。
> 
> 可能还需要考虑不同操作系统的用户，比如开发环境设置中的命令是否适用于所有系统，或者是否需要特别说明。此外，是否需要提供联系方式或讨论渠道，如Slack、Discord等，方便贡献者沟通。
> 
> 最后，保持模板的灵活性，用户可以根据项目实际情况调整各部分内容。确保语言简洁，步骤清晰，避免使用过于专业的术语，让新手也能理解。
> 


以下是标准的开源项目贡献指南模板，可根据你的项目特点调整使用：

---
## 如何贡献

### 🛠️ 开发准备
1. **克隆仓库**
```bash
git clone https://github.com/yourusername/project.git
cd project
```

2. **安装依赖**
```bash
cargo build  # Rust 项目
```

## 📝 贡献流程

### 1. 报告问题
- 在 [Issues](issues/) 中搜索是否已有类似问题
- 使用问题模板提交新 Issue，包含：
  - 环境信息（OS/语言版本）
  - 重现步骤
  - 预期与实际行为

### 2. 开发新功能
```bash
git checkout -b feat/your-feature-name
# 开发完成后提交
git commit -m "feat: 添加XX功能"
git push origin feat/your-feature-name
```

### 3. 提交 Pull Request
- 目标分支：`main`
- PR 必须包含：
  - 功能描述
  - 测试用例
  - 文档更新
  - 关联的 Issue 编号

### ⚙️ 代码规范
- 遵循 [Rust API 指南](https://rust-lang.github.io/api-guidelines/) 
- 提交信息使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：
  ```text
  feat: 添加新功能
  fix: 修复问题
  docs: 文档更新
  style: 代码格式
  refactor: 重构代码
  ```

### 🧪 测试要求
```bash
cargo test --all      # 运行所有测试
cargo test --doc      # 文档测试
cargo clippy --all-targets -- -D warnings  # 代码规范检查
```

### 💬 沟通方式
- 讨论区：[GitHub Discussions](discussions/)
- 紧急问题：daoshi1593@email.com

## 📃 许可证
贡献即表示您同意遵守 [MIT 许可证](LICENSE) 条款
