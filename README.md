# 代码解析工具集

这个仓库提供了一套用于多种编程语言的代码解析工具集，可以帮助开发者分析和理解不同语言项目的结构和函数组织。(只支持linux环境)

## 功能特点

- 支持多种编程语言:Rust、Python、JavaScript、Java、C和C++
- 统一的命令行接口，易于使用
- 分析源代码结构，提取函数、方法和类信息
- 自动生成代码结构报告
- 模块化设计，易于扩展支持新的语言
- 支持递归分析项目依赖
- 支持自定义输出格式
- 支持忽略特定目录和文件
- 交互式命令行界面
- 自动环境检测和配置

## 快速开始

### 系统要求

- Linux 操作系统
- Rust 1.70.0 或更高版本
- Python 3.8 或更高版本（用于Python解析器）
- Node.js 14.0.0 或更高版本（用于JavaScript解析器）
- JDK 11 或更高版本（用于Java解析器）
- GCC/G++ 9.0 或更高版本（用于C/C++解析器）

### 安装

```bash
# 克隆仓库
git clone https://github.com/daoshi1593/Rust-API-analysis.git
cd Rust-API-analysis

# 编译控制器
cd src
rustc main.rs -o parser
cd ..

# 设置脚本执行权限
chmod +x scripts/setup.sh
```

### 使用方法

#### 交互式界面

推荐使用交互式界面，它提供了更友好的用户体验：

```bash
./scripts/setup.sh
```

交互式界面提供以下功能：
1. 环境检查：自动检查所需的依赖是否已安装
2. 环境配置：创建必要的配置文件和目录
3. 运行解析器：选择语言并分析项目
4. 查看帮助：显示详细的使用说明

#### 命令行方式

也可以直接使用命令行方式：

```bash
./src/parser <语言> <目录路径> [选项]
```

#### 基本用法示例:

```bash
# 分析Python项目
./parser python /path/to/python/project

# 分析Rust项目
./parser rust /path/to/rust/project

# 分析JavaScript项目
./parser javascript /path/to/js/project
```

#### 高级选项:

```bash
# 忽略特定目录
./parser python /path/to/project --ignore-dirs "tests,venv"

# 自定义输出文件
./parser rust /path/to/project --output analysis_report.txt

# 递归分析依赖
./parser python /path/to/project --recursive

# 指定文件扩展名
./parser python /path/to/project --extensions "py,pyx"
```

## 项目结构

```
Rust-API-analysis
├─ .gitignore
├─ Cargo.lock
├─ Cargo.toml
├─ LICENSE
├─ README.md
├─ git_s.sh
├─ src
│    ├─ main.rs          # 主控制器
│    ├─ pythonAPI.py     # Python解析器
│    └─ rustAPI.rs       # Rust解析器
└─ scripts
    └─ auto_comment.py   # 自动注释脚本
```

## 支持的语言

### Python

Python解析器可以提取Python文件中的函数、类和方法定义。

特性：
- 支持异步函数分析
- 支持装饰器识别
- 支持类型注解解析
- 支持嵌套类和方法

```bash
./parser python /path/to/python/project
```

### Rust

Rust解析器使用syn库分析Rust项目，提取函数、方法、trait实现等。

特性：
- 支持宏展开分析
- 支持泛型函数解析
- 支持trait实现识别
- 支持模块结构分析

```bash
./parser rust /path/to/rust/project
```

### JavaScript/TypeScript

JavaScript解析器支持分析现代JS/TS项目，包括函数、类、方法和箭头函数等。

特性：
- 支持ES6+语法
- 支持TypeScript类型
- 支持装饰器解析
- 支持模块导入分析

```bash
./parser javascript /path/to/js/project
```

### Java

Java解析器提取Java项目中的类、方法和接口定义。

特性：
- 支持注解解析
- 支持泛型类型
- 支持内部类分析
- 支持接口实现识别

```bash
./parser java /path/to/java/project
```

### C/C++

C和C++解析器分析C/C++项目中的函数、结构体和类定义。

特性：
- 支持宏定义解析
- 支持模板分析
- 支持命名空间
- 支持多文件项目

```bash
./parser c /path/to/c/project
./parser cpp /path/to/cpp/project
```

## 输出格式

### 标准输出格式

所有解析器生成的报告将保存在项目目录下，包含源代码中所有函数和方法的列表。例如:

```
文件: /path/to/project/src/main.py
  - function1
  - function2
  - Class1.method1
  - Class1.method2

文件: /path/to/project/src/module.py
  - helper_function
  - AnotherClass.do_something
```

### JSON输出格式

使用 `--format json` 选项可以获取JSON格式的输出：

```json
{
  "project": {
    "files": [
      {
        "path": "/path/to/project/src/main.py",
        "functions": ["function1", "function2"],
        "classes": [
          {
            "name": "Class1",
            "methods": ["method1", "method2"]
          }
        ]
      }
    ]
  }
}
```

## 配置说明

### 环境变量

可以通过设置以下环境变量来自定义解析器行为：

```bash
# 设置输出目录
export PARSER_OUTPUT_DIR="/path/to/output"

# 设置日志级别
export PARSER_LOG_LEVEL="debug"

# 设置最大递归深度
export PARSER_MAX_DEPTH=5
```

### 配置文件

配置文件位于 `~/.parser/config.json`，可以通过交互式界面自动创建，也可以手动编辑：

```json
{
    "ignore_dirs": ["tests", "node_modules", "target", "venv", "__pycache__"],
    "output_format": "text",
    "max_depth": 5,
    "log_level": "info",
    "output_dir": "~/.parser/output"
}
```

## 常见问题解答

### Q: 如何处理大型项目？

A: 对于大型项目，建议使用以下选项：
- 使用 `--ignore-dirs` 忽略不必要的目录
- 设置 `--max-depth` 限制递归深度
- 使用 `--output` 指定输出文件

### Q: 解析器支持哪些编码格式？

A: 默认支持 UTF-8 编码，可以通过 `--encoding` 选项指定其他编码。

### Q: 如何处理项目依赖？

A: 使用 `--recursive` 选项可以递归分析项目依赖，但请注意这可能会显著增加处理时间。

### Q: 如何自定义输出格式？

A: 可以通过 `--format` 选项选择输出格式，支持 "text"、"json" 和 "html" 格式。

## 扩展支持新语言

要添加对新语言的支持:

1. 在`parsers/`目录下创建新的解析器文件（例如:`goAPI.go`）
2. 在`controller.rs`中添加对应的处理函数和匹配规则
3. 实现解析器，确保它能接收目录路径作为命令行参数
4. 生成与其他解析器兼容的输出格式

## 依赖项

- Rust 1.70.0+（用于控制器和Rust解析器）
- Python 3.8+（用于Python解析器）
- Node.js 14.0.0+（用于JavaScript解析器）
- JDK 11+（用于Java解析器）
- GCC/G++ 9.0+（用于C/C++解析器）

## 性能优化建议

1. 对于大型项目，建议使用 `--ignore-dirs` 排除不必要的目录
2. 使用 `--max-depth` 限制递归深度
3. 考虑使用 `--parallel` 选项启用并行处理
4. 对于频繁使用的项目，可以缓存解析结果

## 贡献指南

欢迎贡献！如果您想改进现有解析器或添加新语言支持，请遵循以下步骤:

1. Fork该仓库
2. 创建您的功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个Pull Request

## 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。

## 联系方式

如有任何问题或建议，请通过 [issues](https://github.com/daoshi1593/Rust-API-analysis/issues) 页面联系我们。

## 更新日志

### v1.0.0 (2024-03-24)
- 初始版本发布
- 支持多种编程语言解析
- 提供基本的命令行接口
- 支持自动注释功能

### v1.1.0 (2024-03-25)
- 添加交互式命令行界面
- 添加自动环境检测功能
- 添加环境配置向导
- 改进用户交互体验
- 添加更多配置选项
