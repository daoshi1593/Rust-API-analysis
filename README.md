# 代码解析工具集

这个仓库提供了一套用于多种编程语言的代码解析工具集，可以帮助开发者分析和理解不同语言项目的结构和函数组织。(只支持linux环境)

## 功能特点

- 支持多种编程语言:Rust、Python、JavaScript、Java、C和C++
- 统一的命令行接口，易于使用
- 分析源代码结构，提取函数、方法和类信息
- 自动生成代码结构报告
- 模块化设计，易于扩展支持新的语言

## 快速开始

### 安装

```bash
# 克隆仓库
git clone https://github.com/daoshi1593/Rust-API-analysis.git
cd Rust-API-analysis/src

# 编译控制器
rustc main.rs -o parser
```

### 使用方法

```bash
./parser <语言> <目录路径>
```

例如:

```bash
# 分析Python项目
./parser python /path/to/python/project

# 分析Rust项目
./parser rust /path/to/rust/project

# 分析JavaScript项目
./parser javascript /path/to/js/project
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
│    ├─ main.rs
│    ├─ pythonAPI.py
│    └─ rustAPI.rs
```

## 支持的语言

### Python

Python解析器可以提取Python文件中的函数、类和方法定义。

```bash
./parser python /path/to/python/project
```

### Rust

Rust解析器使用syn库分析Rust项目，提取函数、方法、trait实现等。

```bash
./parser rust /path/to/rust/project
```

### JavaScript/TypeScript

JavaScript解析器支持分析现代JS/TS项目，包括函数、类、方法和箭头函数等。

```bash
./parser javascript /path/to/js/project
```

### Java

Java解析器提取Java项目中的类、方法和接口定义。

```bash
./parser java /path/to/java/project
```

### C/C++

C和C++解析器分析C/C++项目中的函数、结构体和类定义。

```bash
./parser c /path/to/c/project
./parser cpp /path/to/cpp/project
```

## 输出格式

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

## 扩展支持新语言

要添加对新语言的支持:

1. 在`parsers/`目录下创建新的解析器文件（例如:`goAPI.go`）
2. 在`controller.rs`中添加对应的处理函数和匹配规则
3. 实现解析器，确保它能接收目录路径作为命令行参数
4. 生成与其他解析器兼容的输出格式

## 依赖项

- Rust（用于控制器和Rust解析器）
- Python 3.x（用于Python解析器）
- Node.js（用于JavaScript解析器）
- JDK（用于Java解析器）
- GCC/G++（用于C/C++解析器）


# 脚本工具集

## 自动注释脚本

这是一个基于大型语言模型的自动代码注释工具，能够为Python、Rust等多种语言的代码文件添加清晰的注释，提升代码可读性。

### 获取API_KEY

要使用本脚本，您需要获取火山引擎ARK平台的API密钥:

1. 登录火山引擎ARK平台控制台 (https://console.volcengine.com/ark/)
2. 导航至"密钥管理"页面
3. 创建新的API密钥或使用现有密钥
4. 复制生成的API_KEY字符串，该密钥将用于授权API调用

### 设置API_KEY和MODEL_NAME
(MODEL_NAME在火山平台是接入点)
在运行脚本前，需要将获取的API_KEY,MODEL_NAME设置为环境变量:

**Linux:**
```bash
export ARK_API_KEY="your_api_key_here"
export MODEL_NAME="model_name"
```

### 运行脚本

脚本使用方法如下:

```bash
python script_name.py [目录路径] [选项]
```

**基本用法:**
```bash
python script_name.py /path/to/your/code
```

**指定文件类型:**
```bash
python script_name.py /path/to/your/code --extensions py js java
```

**使用特定模型:**
```bash
python script_name.py /path/to/your/code --model ep-custom-model-id
```

**参数说明:**
- `directory`:必选参数，指定要处理的代码目录
- `--extensions`:可选参数，指定要处理的文件扩展名，默认为 `.py` 和 `.rs`
- `--model`:可选参数，指定使用的ARK模型ID
- `--ignore-dirs`:要忽略的目录
脚本会递归扫描指定目录下所有符合扩展名条件的文件，通过ARK API生成适当的代码注释，并将注释后的代码写回原文件。

注意:此操作可能会消耗大量的token,请谨慎使用
并且请注意备份和版本控制，此操作由于llm回复不确定性可能会出现无法返回正确的代码的情况
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
