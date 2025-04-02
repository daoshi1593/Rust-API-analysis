#!/bin/bash

# 获取用户的默认 shell
USER_SHELL=$(getent passwd $USER | cut -d: -f7)

# 如果用户 shell 不是 bash，则使用用户的 shell 重新执行脚本
if [ "$USER_SHELL" != "/bin/bash" ]; then
    exec $USER_SHELL "$0" "$@"
fi

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'

# ASCII 艺术字
LOGO="
${CYAN}██████╗ ██████╗  ██████╗ ██████╗      ██╗     ██╗
${CYAN}██╔══██╗██╔══██╗██╔════╝██╔════╝      ██║     ██║
${CYAN}██████╔╝██████╔╝██║     ██║         ██╗██║     ██║
${CYAN}██╔══██╗██╔═══╝ ██║     ██║         ╚═╝██║██   ██║
${CYAN}██║  ██║██║     ╚██████╗╚██████╗       ██║╚█████╔╝
${CYAN}╚═╝  ╚═╝╚═╝      ╚═════╝ ╚═════╝       ╚═╝ ╚════╝ ${NC}
"

# 打印带颜色的信息
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_header() {
    echo -e "${BLUE}[HEADER]${NC} $1"
}

# 显示加载动画
show_spinner() {
    local pid=$1
    local delay=0.1
    local spinstr='|/-\'
    while kill -0 $pid 2>/dev/null; do
        local temp=${spinstr#?}
        printf "\r[%c] 处理中..." "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
    done
    printf "\r   \r"
}

# 检查命令是否存在
check_command() {
    if ! command -v $1 &> /dev/null; then
        print_error "$1 未安装"
        return 1
    else
        print_success "$1 已安装"
        return 0
    fi
}

# 检查版本
check_version() {
    local cmd=$1
    local min_version=$2
    local version_cmd=$3
    
    if ! command -v $cmd &> /dev/null; then
        return 1
    fi
    
    local version=$($version_cmd)
    if [ "$(printf '%s\n' "$min_version" "$version" | sort -V | head -n1)" = "$min_version" ]; then 
        print_success "$cmd 版本 $version 满足要求"
        return 0
    else
        print_error "$cmd 版本过低，需要 $min_version 或更高版本"
        return 1
    fi
}

# 检查环境
check_environment() {
    print_header "开始检查环境..."
    
    # 检查 Rust
    check_command "rustc" || return 1
    check_version "rustc" "1.70.0" "rustc --version | cut -d' ' -f2" || return 1
    
    # 检查 Python
    check_command "python3" || return 1
    check_version "python3" "3.8.0" "python3 --version | cut -d' ' -f2" || return 1
    
    # 检查 Node.js
    check_command "node" || return 1
    check_version "node" "14.0.0" "node --version | cut -d'v' -f2" || return 1
    
    # 检查 Java
    check_command "java" || return 1
    check_version "java" "11.0.0" "java -version 2>&1 | head -n 1 | cut -d'"' -f2" || return 1
    
    # 检查 LLVM/Clang
    check_command "clang" || return 1
    check_version "clang" "14.0.0" "clang --version | head -n 1 | cut -d' ' -f3" || return 1
    
    print_success "环境检查完成"
    return 0
}

# 创建配置文件
create_config() {
    print_header "创建配置文件..."
    
    local config_dir="$HOME/.parser"
    local config_file="$config_dir/config.json"
    
    mkdir -p "$config_dir"
    
    cat > "$config_file" << EOF
{
    "ignore_dirs": ["tests", "node_modules", "target", "venv", "__pycache__"],
    "output_format": "text",
    "max_depth": 5,
    "log_level": "info",
    "output_dir": "~/.parser/output"
}
EOF
    
    print_success "配置文件创建完成"
}

# 编译 Rust 项目
compile_rust() {
    print_header "编译 Rust 项目..."
    
    cd src || exit 1
    rustc rustAPI.rs -o rustAPI
    cd ..
    
    print_success "Rust 项目编译完成"
}

# 设置 Python 环境
setup_python() {
    print_header "设置 Python 环境..."
    
    # 创建虚拟环境
    python3 -m venv venv
    source venv/bin/activate
    
    # 安装依赖
    pip install -r requirements.txt
    
    print_success "Python 环境设置完成"
}

# 设置 Node.js 环境
setup_nodejs() {
    print_header "设置 Node.js 环境..."
    
    # 安装依赖
    npm install
    
    print_success "Node.js 环境设置完成"
}

# 设置 Java 环境
setup_java() {
    print_header "设置 Java 环境..."
    
    # 编译 Java 文件
    javac src/JavaAPI.java
    
    print_success "Java 环境设置完成"
}

# 设置 C/C++ 环境
setup_cpp() {
    print_header "设置 C/C++ 环境..."
    
    # 编译 C/C++ 文件
    gcc src/cAPI.c -o src/parser
    
    print_success "C/C++ 环境设置完成"
}

# 主函数
main() {
    # 显示 logo
    echo -e "$LOGO"
    echo -e "${CYAN}API 分析工具环境配置脚本${NC}\n"
    
    # 检查环境
    if ! check_environment; then
        print_error "环境检查失败，请安装所需依赖"
        exit 1
    fi
    
    # 创建配置文件
    create_config
    
    # 编译 Rust 项目
    compile_rust
    
    # 设置 Python 环境
    setup_python
    
    # 设置 Node.js 环境
    setup_nodejs
    
    # 设置 Java 环境
    setup_java
    
    # 设置 C/C++ 环境
    setup_cpp
    
    print_success "环境配置完成！"
    print_info "使用方法："
    print_info "1. 分析 Rust 项目：./src/rustAPI <目录路径>"
    print_info "2. 分析 Python 项目：python src/pythonAPI.py <目录路径>"
    print_info "3. 分析 JavaScript 项目：node src/javascriptAPI.js <目录路径>"
    print_info "4. 分析 Java 项目：java -cp . src.JavaAPI <目录路径>"
    print_info "5. 分析 C/C++ 项目：./src/parser <目录路径>"
}

# 执行主函数
main 