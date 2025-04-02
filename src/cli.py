#!/usr/bin/env python3
import os
import sys
import subprocess
from typing import List, Dict, Any

class Dashboard:
    """API 分析仪表板"""
    
    def __init__(self):
        self.term_size = os.get_terminal_size()
        self.width = self.term_size.columns
        self.height = self.term_size.lines
        self.api_data = []
        self.current_section = 0  # 当前选中的部分
        self.sections = ["Expressions", "History", "Memory", "Registers", "Source"]
        
    def clear_screen(self):
        """清空屏幕"""
        print("\033[2J\033[H", end="")
        
    def print_header(self, text: str, style: str = "1;32"):
        """打印带颜色的标题"""
        print(f"\033[{style}m{text}\033[0m")
        
    def print_divider(self):
        """打印分隔线"""
        print(f"\033[90m{'─' * self.width}\033[0m")
        
    def print_section(self, title: str, content: List[str]):
        """打印一个区域"""
        self.print_header(f" {title} ", "1;33")
        self.print_divider()
        for line in content:
            print(line)
            
    def format_register(self, name: str, value: str) -> str:
        """格式化寄存器显示"""
        return f"\033[90m{name:>4}\033[0m \033[32m{value}\033[0m"
        
    def display_registers(self):
        """显示寄存器区域"""
        registers = [
            ("rax", "0x0000000000401156"),
            ("rbx", "0x00000000004011c0"),
            ("rcx", "0x00000000004011c0"),
            ("rdx", "0x00007fffffffd268"),
            ("rsi", "0x00007fffffffd258"),
            ("rdi", "0x0000000000000001"),
        ]
        
        lines = []
        for i in range(0, len(registers), 4):
            row = registers[i:i+4]
            line = "  ".join(self.format_register(name, value) for name, value in row)
            lines.append(line)
        return lines
        
    def display_source(self):
        """显示源代码区域"""
        return [
            "\033[90m1\033[0m  #include <stdio.h>",
            "\033[90m2\033[0m  ",
            "\033[90m3\033[0m  int main() {",
            "\033[90m4\033[0m      printf(\"Hello, World!\\n\");",
            "\033[90m5\033[0m      return 0;",
            "\033[90m6\033[0m  }",
        ]
        
    def display(self):
        """显示整个界面"""
        self.clear_screen()
        
        # 显示断点信息
        self.print_header("[1] break at 0x0000000000401156 in testa.c:6 for main hit 1 time", "1;36")
        
        # 显示各个区域
        sections_content = {
            "Expressions": [""],
            "History": [""],
            "Memory": [""],
            "Registers": self.display_registers(),
            "Source": self.display_source()
        }
        
        for section in self.sections:
            self.print_section(section, sections_content[section])
            print()
            
    def run(self):
        """运行界面"""
        try:
            while True:
                self.display()
                cmd = input("\033[1;32mgdb>\033[0m ")
                if cmd in ["q", "quit"]:
                    break
                elif cmd == "refresh":
                    continue
                else:
                    print(f"Unknown command: {cmd}")
                    input("Press Enter to continue...")
                    
        except KeyboardInterrupt:
            print("\nExiting...")
            sys.exit(0)

if __name__ == "__main__":
    dashboard = Dashboard()
    dashboard.run() 