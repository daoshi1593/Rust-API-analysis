# Rust API 分析工具

一个用于分析多种编程语言 API 的工具，支持 Rust、Python、JavaScript、Java、C 和 C++。

## 功能特性

- 支持多种编程语言的 API 分析
  - Rust: 使用 syn 2.0 进行语法分析
  - Python: 使用 ast 模块解析
  - JavaScript: 使用 esprima 解析
  - Java: 使用 JavaParser 解析
  - C/C++: 使用 libclang 解析
- 自动提取函数声明和定义
- 支持宏定义分析
- 支持继承关系分析
- 生成详细的 API 文档
- 跨平台支持
- 统一的命令行接口
- 支持递归分析项目依赖
- 支持自定义输出格式
- 支持忽略特定目录和文件
- 交互式命令行界面
- 自动环境检测和配置

## 环境要求

- Rust 1.70+
- Python 3.8+
- Node.js 14+
- Java 11+
- LLVM/Clang 14+

## 安装

1. 克隆仓库：
```bash
git clone https://github.com/daoshi1593/Rust-API-analysis.git
cd Rust-API-analysis
```

2. 运行安装脚本：
```bash
./scripts/setup.sh
```

## 使用方法

### 交互式界面

推荐使用交互式界面，它提供了更友好的用户体验：

```bash
./scripts/setup.sh
```

交互式界面提供以下功能：
1. 环境检查：自动检查所需的依赖是否已安装
2. 环境配置：创建必要的配置文件和目录
3. 运行解析器：选择语言并分析项目
4. 查看帮助：显示详细的使用说明

### 命令行方式

也可以直接使用命令行方式：

```bash
# 分析 Rust 项目
./src/rustAPI <目录路径>

# 分析 Python 项目
python src/pythonAPI.py <目录路径>

# 分析 JavaScript 项目
node src/javascriptAPI.js <目录路径>

# 分析 Java 项目
javac src/JavaAPI.java
java -cp . src.JavaAPI <目录路径>

# 分析 C/C++ 项目
./src/parser <目录路径>
```

#### 基本用法示例:

```bash
# 分析 Rust 项目
./src/rustAPI /path/to/rust/project

# 分析 Python 项目
python src/pythonAPI.py /path/to/python/project

# 分析 JavaScript 项目
node src/javascriptAPI.js /path/to/js/project
```

#### 高级选项:

```bash
# 指定输出文件
./src/rustAPI /path/to/project > output.txt

# 递归分析依赖
python src/pythonAPI.py /path/to/project --recursive

# 忽略特定目录
node src/javascriptAPI.js /path/to/project --ignore "node_modules,tests"
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
- 使用重定向将输出保存到文件

### Q: 解析器支持哪些编码格式？

A: 默认支持 UTF-8 编码，可以通过 `--encoding` 选项指定其他编码。

### Q: 如何处理项目依赖？

A: 使用 `--recursive` 选项可以递归分析项目依赖，但请注意这可能会显著增加处理时间。

### Q: 如何自定义输出格式？

A: 可以通过 `--format` 选项选择输出格式，支持 "text"、"json" 和 "html" 格式。

## 项目结构

```
.
├── src/                    # 源代码目录
│   ├── rustAPI.rs         # Rust API 分析器
│   ├── pythonAPI.py       # Python API 分析器
│   ├── javascriptAPI.js   # JavaScript API 分析器
│   ├── JavaAPI.java       # Java API 分析器
│   ├── cAPI.c            # C API 分析器
│   └── cppAPI.cpp        # C++ API 分析器
├── tests/                 # 测试用例
├── scripts/              # 脚本文件
│   └── setup.sh         # 环境配置脚本
└── README.md            # 项目文档
```

## 开发

### 添加新的语言支持

1. 在 `src` 目录下创建新的分析器文件
2. 实现必要的解析逻辑
3. 更新 `setup.sh` 脚本以支持新语言
4. 添加相应的测试用例

### 运行测试

```bash
cargo test
```

## 性能优化建议

1. 对于大型项目，建议使用 `--ignore-dirs` 排除不必要的目录
2. 使用 `--max-depth` 限制递归深度
3. 考虑使用 `--parallel` 选项启用并行处理
4. 对于频繁使用的项目，可以缓存解析结果

## 贡献

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

### v1.2.0 (2024-03-26)
- 升级 syn 到 2.0 版本
- 优化 Rust API 分析器
- 添加多语言支持
- 完善项目文档
- 添加测试用例
