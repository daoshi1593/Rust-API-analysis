#!/bin/bash

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 显示菜单
show_menu() {
    clear
    echo -e "${BLUE}=== API 分析工具 ===${NC}"
    echo -e "1. 检查环境"
    echo -e "2. 分析 Rust 项目"
    echo -e "3. 分析 Python 项目"
    echo -e "4. 分析 JavaScript 项目"
    echo -e "5. 分析 Java 项目"
    echo -e "6. 分析 C/C++ 项目"
    echo -e "0. 退出"
    echo -e "\n请选择操作 (0-6): "
}

# 检查环境
check_env() {
    echo -e "\n${BLUE}=== 环境检查 ===${NC}"
    
    # 检查 Rust
    if command -v rustc &> /dev/null; then
        echo -e "${GREEN}✓ Rust 已安装${NC}"
    else
        echo -e "${RED}✗ Rust 未安装${NC}"
    fi
    
    # 检查 Python
    if command -v python3 &> /dev/null; then
        echo -e "${GREEN}✓ Python 已安装${NC}"
    else
        echo -e "${RED}✗ Python 未安装${NC}"
    fi
    
    # 检查 Node.js
    if command -v node &> /dev/null; then
        echo -e "${GREEN}✓ Node.js 已安装${NC}"
    else
        echo -e "${RED}✗ Node.js 未安装${NC}"
    fi
    
    # 检查 Java
    if command -v java &> /dev/null; then
        echo -e "${GREEN}✓ Java 已安装${NC}"
    else
        echo -e "${RED}✗ Java 未安装${NC}"
    fi
    
    # 检查 Clang
    if command -v clang &> /dev/null; then
        echo -e "${GREEN}✓ Clang 已安装${NC}"
    else
        echo -e "${RED}✗ Clang 未安装${NC}"
    fi
    
    echo -e "\n按回车键继续..."
    read
}

# 分析项目
analyze_project() {
    local type=$1
    local cmd=$2
    
    echo -e "\n${BLUE}=== 分析 $type 项目 ===${NC}"
    echo -e "请输入项目目录路径: "
    read path
    
    if [ ! -d "$path" ]; then
        echo -e "${RED}错误: 目录不存在${NC}"
        echo -e "\n按回车键继续..."
        read
        return
    fi
    
    echo -e "\n开始分析..."
    eval "$cmd $path"
    
    echo -e "\n按回车键继续..."
    read
}

# 主循环
while true; do
    show_menu
    read choice
    
    case $choice in
        1) check_env ;;
        2) analyze_project "Rust" "./src/rustAPI" ;;
        3) analyze_project "Python" "python src/pythonAPI.py" ;;
        4) analyze_project "JavaScript" "node src/javascriptAPI.js" ;;
        5) analyze_project "Java" "java -cp . src.JavaAPI" ;;
        6) analyze_project "C/C++" "./src/parser" ;;
        0) echo -e "\n${GREEN}感谢使用！${NC}"; exit 0 ;;
        *) echo -e "\n${RED}无效的选择，请重试${NC}"; sleep 1 ;;
    esac
done 