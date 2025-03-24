#!/usr/bin/env python
# -*- coding: utf-8 -*-

import os
import ast
import sys
from pathlib import Path
from typing import List, Optional


class FunctionVisitor(ast.NodeVisitor):
    def __init__(self):
        self.functions = []
        self.current_class = None

    def visit_FunctionDef(self, node):
        if self.current_class:
            # 如果在类内部，记录为方法，格式：类名.方法名
            self.functions.append(f"{self.current_class}.{node.name}")
        else:
            # 否则记录为函数
            self.functions.append(node.name)
        # 递归访问函数体
        self.generic_visit(node)

    def visit_AsyncFunctionDef(self, node):
        # 处理异步函数定义，与普通函数处理方式相同
        self.visit_FunctionDef(node)

    def visit_ClassDef(self, node):
        old_class = self.current_class
        self.current_class = node.name
        # 递归访问类体
        self.generic_visit(node)
        self.current_class = old_class

    def visit_Lambda(self, node):
        # 匿名函数无法命名，所以不记录
        self.generic_visit(node)


def process_file(file_path: Path) -> Optional[List[str]]:
    """处理单个Python文件，提取所有函数和方法名"""
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            content = file.read()
        tree = ast.parse(content, filename=file_path)
        visitor = FunctionVisitor()
        visitor.visit(tree)
        return visitor.functions
    except Exception as e:
        print(f"处理文件 {file_path} 时出错: {e}")
        return None


def walk_directory(directory: Path) -> None:
    """遍历目录，处理所有Python文件"""
    log_path = directory / "python_fns_log"
    
    with open(log_path, 'w', encoding='utf-8') as log_file:
        for root, _, files in os.walk(directory):
            for file in files:
                if file.endswith('.py'):
                    file_path = Path(root) / file
                    functions = process_file(file_path)
                    
                    if functions and len(functions) > 0:
                        # 写入文件路径
                        log_file.write(f"文件: {file_path}\n")
                        # 写入函数名称
                        for func in functions:
                            log_file.write(f"  - {func}\n")
    
    print(f"Python函数列表已写入到 {log_path}")


def main():
    """主函数"""
    # 如果存在toRead.txt文件，则从文件读取目录路径
    directory = Path(sys.argv[1])
    print(f"使用命令行参数目录: {directory}")
    # 检查路径是否存在
    if not directory.exists():
        print(f"错误：目录 '{directory}' 不存在")
        return
    if not directory.is_dir():
        print(f"错误：路径 '{directory}' 不是一个目录")
        return
    
    walk_directory(directory)


if __name__ == "__main__":
    main()