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
    echo
    
    # 检查操作系统
    if [ "$(uname)" != "Linux" ]; then
        print_error "当前系统不是 Linux"
        exit 1
    fi
    
    # 检查 Rust
    check_version "rustc" "1.70.0" "rustc --version"
    if [ $? -ne 0 ]; then
        print_warning "请安装 Rust 1.70.0 或更高版本"
        print_info "访问 https://rustup.rs/ 安装 Rust"
    fi
    
    # 检查 Python
    check_version "python3" "3.8.0" "python3 --version"
    if [ $? -ne 0 ]; then
        print_warning "请安装 Python 3.8.0 或更高版本"
    fi
    
    # 检查 Node.js
    check_version "node" "14.0.0" "node --version"
    if [ $? -ne 0 ]; then
        print_warning "请安装 Node.js 14.0.0 或更高版本"
    fi
    
    # 检查 Java
    check_version "java" "11" "java -version 2>&1 | head -n 1"
    if [ $? -ne 0 ]; then
        print_warning "请安装 JDK 11 或更高版本"
    fi
    
    # 检查 GCC/G++
    check_version "gcc" "9.0.0" "gcc --version | head -n 1"
    if [ $? -ne 0 ]; then
        print_warning "请安装 GCC 9.0.0 或更高版本"
    fi
    
    check_version "g++" "9.0.0" "g++ --version | head -n 1"
    if [ $? -ne 0 ]; then
        print_warning "请安装 G++ 9.0.0 或更高版本"
    fi
    
    # 检查 libclang
    if ! pkg-config --exists libclang; then
        print_warning "未找到 libclang，C/C++ 解析器可能无法正常工作"
        print_info "请安装 libclang-dev 包"
    fi
    
    echo
    read -p "按回车键继续..."
}

# 配置环境
setup_environment() {
    print_header "开始配置环境..."
    echo
    
    # 创建必要的目录
    mkdir -p ~/.parser
    mkdir -p ~/.parser/logs
    mkdir -p ~/.parser/output
    
    # 创建默认配置文件
    cat > ~/.parser/config.json << EOF
{
    "ignore_dirs": ["tests", "node_modules", "target", "venv", "__pycache__"],
    "output_format": "text",
    "max_depth": 5,
    "log_level": "info",
    "output_dir": "~/.parser/output",
    "supported_languages": ["rust", "python", "javascript", "java", "c", "cpp"],
    "analysis_options": {
        "include_comments": true,
        "include_docs": true,
        "include_tests": false
    }
}
EOF
    
    print_success "环境配置完成"
    echo
    read -p "按回车键继续..."
}

# 显示语言选择菜单
show_language_menu() {
    clear
    echo -e "$LOGO"
    echo -e "${BOLD}=== 选择要分析的语言 ===${NC}"
    echo -e "${DIM}1${NC} Rust"
    echo -e "${DIM}2${NC} Python"
    echo -e "${DIM}3${NC} JavaScript"
    echo -e "${DIM}4${NC} Java"
    echo -e "${DIM}5${NC} C"
    echo -e "${DIM}6${NC} C++"
    echo -e "${DIM}7${NC} 返回主菜单"
    echo -e "${BOLD}======================${NC}"
    
    read -p "请选择 [1-7]: " lang_choice
    
    case $lang_choice in
        1) lang="rust" ;;
        2) lang="python" ;;
        3) lang="javascript" ;;
        4) lang="java" ;;
        5) lang="c" ;;
        6) lang="cpp" ;;
        7) return ;;
        *) 
            print_error "无效的选择"
            sleep 2
            return
            ;;
    esac
    
    show_project_path_menu "$lang"
}

# 显示项目路径选择菜单
show_project_path_menu() {
    local lang=$1
    clear
    echo -e "$LOGO"
    echo -e "${BOLD}=== 选择项目路径 ===${NC}"
    echo -e "${DIM}1${NC} 当前目录"
    echo -e "${DIM}2${NC} 指定目录"
    echo -e "${DIM}3${NC} 返回语言选择"
    echo -e "${BOLD}==================${NC}"
    
    read -p "请选择 [1-3]: " path_choice
    
    case $path_choice in
        1)
            project_path="."
            ;;
        2)
            read -p "请输入项目路径: " project_path
            project_path="${project_path/#\~/$HOME}"
            ;;
        3)
            return
            ;;
        *)
            print_error "无效的选择"
            sleep 2
            return
            ;;
    esac
    
    if [ ! -d "$project_path" ]; then
        print_error "目录不存在: $project_path"
        sleep 2
        return
    fi
    
    show_output_menu "$lang" "$project_path"
}

# 显示输出选项菜单
show_output_menu() {
    local lang=$1
    local project_path=$2
    clear
    echo -e "$LOGO"
    echo -e "${BOLD}=== 选择输出选项 ===${NC}"
    echo -e "${DIM}1${NC} 默认输出 (fns_log)"
    echo -e "${DIM}2${NC} 自定义输出路径"
    echo -e "${DIM}3${NC} 返回项目路径选择"
    echo -e "${BOLD}==================${NC}"
    
    read -p "请选择 [1-3]: " output_choice
    
    case $output_choice in
        1)
            output_path="$project_path/fns_log"
            ;;
        2)
            read -p "请输入输出文件路径: " output_path
            output_path="${output_path/#\~/$HOME}"
            ;;
        3)
            show_project_path_menu "$lang"
            return
            ;;
        *)
            print_error "无效的选择"
            sleep 2
            return
            ;;
    esac
    
    run_analysis "$lang" "$project_path" "$output_path"
}

# 运行分析
run_analysis() {
    local lang=$1
    local project_path=$2
    local output_path=$3
    
    clear
    echo -e "$LOGO"
    print_header "开始分析 $lang 项目: $project_path"
    echo
    
    # 确保项目路径是绝对路径
    project_path=$(realpath "$project_path")
    
    case $lang in
        "rust")
            if [ ! -f "./src/rustAPI" ]; then
                print_error "Rust API 可执行文件不存在"
                return 1
            fi
            ./src/rustAPI "$project_path" "$output_path" &
            show_spinner $!
            ;;
        *)
            if [ ! -f "./parser" ]; then
                print_error "解析器可执行文件不存在"
                return 1
            fi
            ./parser analyze $lang "$project_path" "$output_path" &
            show_spinner $!
            ;;
    esac
    
    if [ $? -eq 0 ]; then
        print_success "分析完成！结果已保存到: $output_path"
    else
        print_error "分析过程中出现错误"
    fi
    
    echo
    read -p "按回车键继续..."
}

# 显示主菜单
show_menu() {
    while true; do
        clear
        echo -e "$LOGO"
        echo -e "${BOLD}=== 主菜单 ===${NC}"
        echo -e "${DIM}1${NC} 检查环境"
        echo -e "${DIM}2${NC} 配置环境"
        echo -e "${DIM}3${NC} 运行分析"
        echo -e "${DIM}4${NC} 查看帮助"
        echo -e "${DIM}5${NC} 退出"
        echo -e "${BOLD}=============${NC}"
        
        read -p "请选择 [1-5]: " choice
        
        case $choice in
            1)
                check_environment
                ;;
            2)
                setup_environment
                ;;
            3)
                show_language_menu
                ;;
            4)
                show_help
                ;;
            5)
                print_success "感谢使用！"
                exit 0
                ;;
            *)
                print_error "无效的选择"
                sleep 2
                ;;
        esac
    done
}

# 显示帮助
show_help() {
    clear
    echo -e "$LOGO"
    echo -e "${BOLD}=== 帮助信息 ===${NC}"
    echo -e "${DIM}1. 环境要求：${NC}"
    echo -e "   - Linux 操作系统"
    echo -e "   - Rust 1.70.0+"
    echo -e "   - Python 3.8+"
    echo -e "   - Node.js 14.0.0+"
    echo -e "   - JDK 11+"
    echo -e "   - GCC/G++ 9.0+"
    echo
    echo -e "${DIM}2. 使用方法：${NC}"
    echo -e "   - 首次使用请先运行环境检查"
    echo -e "   - 配置环境以创建必要的目录和配置文件"
    echo -e "   - 选择要分析的语言和项目路径"
    echo
    echo -e "${DIM}3. 配置文件位置：${NC}"
    echo -e "   - 主配置文件：~/.parser/config.json"
    echo -e "   - 日志目录：~/.parser/logs"
    echo -e "   - 输出目录：~/.parser/output"
    echo
    echo -e "${DIM}4. 常见问题：${NC}"
    echo -e "   - 如果遇到权限问题，请确保脚本有执行权限"
    echo -e "   - 如果遇到依赖问题，请检查环境要求"
    echo -e "   - 如果遇到性能问题，请调整配置文件中的参数"
    echo
    read -p "按回车键继续..."
}

# 主程序
main() {
    # 检查脚本是否在正确的目录
    if [ ! -f "src/main.rs" ]; then
        print_error "请在项目根目录运行此脚本"
        exit 1
    fi
    
    # 检查必要的文件是否存在
    local required_files=(
        "src/rustAPI.rs"
        "src/pythonAPI.py"
        "src/javascriptAPI.js"
        "src/JavaAPI.java"
        "src/cAPI.c"
        "src/cppAPI.cpp"
    )
    
    for file in "${required_files[@]}"; do
        if [ ! -f "$file" ]; then
            print_error "缺少必要的文件: $file"
            exit 1
        fi
    done
    
    # 编译 Rust 项目
    print_header "正在编译 Rust 项目..."
    if ! cargo build --release; then
        print_error "Rust 项目编译失败"
        exit 1
    fi
    
    # 复制编译好的可执行文件
    cp target/release/code-parser ./parser
    chmod +x ./parser
    
    # 编译 C 和 C++ API
    print_header "正在编译 C/C++ API..."
    if ! pkg-config --exists libclang; then
        print_warning "跳过 C/C++ API 编译，因为未找到 libclang"
    else
        if ! gcc -o src/cAPI src/cAPI.c $(pkg-config --cflags --libs libclang); then
            print_error "C API 编译失败"
            exit 1
        fi
        
        if ! g++ -o src/cppAPI src/cppAPI.cpp $(pkg-config --cflags --libs libclang); then
            print_error "C++ API 编译失败"
            exit 1
        fi
    fi
    
    # 确保 Python 和 JavaScript 脚本有执行权限
    chmod +x src/pythonAPI.py
    chmod +x src/javascriptAPI.js
    
    # 编译 Java API
    print_header "正在编译 Java API..."
    if ! mvn compile; then
        print_error "Java API 编译失败"
        exit 1
    fi
    
    # 编译 Rust API
    print_header "正在编译 Rust API..."
    # 创建临时 Cargo.toml
    cat > src/Cargo.toml << EOF
[package]
name = "rust-api"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "rustAPI"
path = "rustAPI.rs"

[dependencies]
syn = { version = "2.0", features = ["full", "visit", "parsing", "extra-traits"] }
walkdir = "2.3"
anyhow = "1.0"
EOF
    
    # 编译 Rust API
    if ! (cd src && cargo build --release); then
        print_error "Rust API 编译失败"
        exit 1
    fi
    
    # 复制编译好的可执行文件
    cp src/target/release/rustAPI ./src/rustAPI
    chmod +x ./src/rustAPI
    
    # 删除临时 Cargo.toml
    rm src/Cargo.toml
    
    # 显示主菜单
    show_menu
}

# 运行主程序
main 