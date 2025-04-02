#!/usr/bin/env python3
import os
import sys
import subprocess
import json
from typing import List, Dict, Any
from pathlib import Path

class Dashboard:
    """API 分析仪表板"""
    
    def __init__(self):
        self.term_size = os.get_terminal_size()
        self.width = self.term_size.columns
        self.height = self.term_size.lines
        self.current_file = None
        self.current_language = None
        self.analysis_results = {}
        self.config = self.load_config()
        
    def load_config(self) -> Dict:
        """加载配置文件"""
        config_path = os.path.expanduser("~/.parser/config.json")
        if os.path.exists(config_path):
            with open(config_path, 'r') as f:
                return json.load(f)
        return {
            "ignore_dirs": ["tests", "node_modules", "target", "venv", "__pycache__"],
            "output_format": "text",
            "max_depth": 5,
            "log_level": "info",
            "output_dir": os.path.expanduser("~/.parser/output")
        }
        
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
            
    def format_info(self, name: str, info: str) -> str:
        """格式化信息显示"""
        return f"\033[90m{name:>20}\033[0m \033[32m{info}\033[0m"
        
    def check_environment(self) -> Dict[str, bool]:
        """检查环境"""
        env_status = {}
        tools = {
            "Rust": "rustc --version",
            "Python": "python3 --version",
            "Node.js": "node --version",
            "Java": "java -version",
            "Clang": "clang --version"
        }
        
        for name, cmd in tools.items():
            try:
                subprocess.run(cmd.split(), capture_output=True, check=True)
                env_status[name] = True
            except:
                env_status[name] = False
                
        return env_status
        
    def run_analyzer(self, language: str, directory: str):
        """运行相应的分析器"""
        analyzers = {
            "rust": ["./src/rustAPI", directory],
            "python": ["python3", "src/pythonAPI.py", directory],
            "javascript": ["node", "src/javascriptAPI.js", directory],
            "java": ["java", "-cp", ".", "src.JavaAPI", directory],
            "c": ["./src/parser", directory],
            "cpp": ["./src/parser", directory]
        }
        
        if language not in analyzers:
            print(f"\033[31m错误：不支持的语言 {language}\033[0m")
            return
            
        try:
            cmd = analyzers[language]
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode == 0:
                # 读取分析结果文件
                log_file = None
                if language == "rust":
                    log_file = os.path.join(directory, "fns_log")
                elif language == "python":
                    log_file = os.path.join(directory, "python_fns_log")
                
                if log_file and os.path.exists(log_file):
                    with open(log_file, 'r') as f:
                        content = f.read()
                        self.analysis_results = self.parse_log_content(content)
                else:
                    # 尝试直接解析输出
                    try:
                        self.analysis_results = json.loads(result.stdout)
                    except json.JSONDecodeError:
                        # 如果不是 JSON 格式，则解析文本输出
                        self.analysis_results = self.parse_log_content(result.stdout)
                
                self.current_language = language
                print(f"\033[32m分析完成！\033[0m")
            else:
                print(f"\033[31m分析失败：{result.stderr}\033[0m")
        except Exception as e:
            print(f"\033[31m执行出错：{str(e)}\033[0m")
            
    def parse_log_content(self, content: str) -> Dict:
        """解析日志内容为统一格式"""
        result = {"files": []}
        current_file = None
        
        for line in content.splitlines():
            line = line.strip()
            if not line:
                continue
                
            if line.startswith("文件:"):
                if current_file:
                    result["files"].append(current_file)
                current_file = {
                    "path": line[4:].strip(),
                    "functions": [],
                    "classes": []
                }
            elif line.startswith("  - "):
                if current_file:
                    func_name = line[4:].strip()
                    if "." in func_name:
                        # 这是一个类方法
                        class_name, method_name = func_name.split(".")
                        # 查找或创建类
                        class_obj = None
                        for cls in current_file["classes"]:
                            if cls["name"] == class_name:
                                class_obj = cls
                                break
                        if not class_obj:
                            class_obj = {"name": class_name, "methods": []}
                            current_file["classes"].append(class_obj)
                        class_obj["methods"].append({"name": method_name})
                    else:
                        # 这是一个普通函数
                        current_file["functions"].append({"name": func_name})
                        
        if current_file:
            result["files"].append(current_file)
            
        return result
        
    def display_environment(self):
        """显示环境状态"""
        env_status = self.check_environment()
        return [
            f"\033[{'32' if env_status['Rust'] else '31'}m{'✓' if env_status['Rust'] else '✗'}\033[0m Rust",
            f"\033[{'32' if env_status['Python'] else '31'}m{'✓' if env_status['Python'] else '✗'}\033[0m Python",
            f"\033[{'32' if env_status['Node.js'] else '31'}m{'✓' if env_status['Node.js'] else '✗'}\033[0m Node.js",
            f"\033[{'32' if env_status['Java'] else '31'}m{'✓' if env_status['Java'] else '✗'}\033[0m Java",
            f"\033[{'32' if env_status['Clang'] else '31'}m{'✓' if env_status['Clang'] else '✗'}\033[0m Clang"
        ]
        
    def display_analysis(self):
        """显示分析结果"""
        if not self.analysis_results:
            return ["尚未进行分析"]
            
        result = []
        for file_info in self.analysis_results.get("files", []):
            result.append(f"\033[36m文件：{file_info['path']}\033[0m")
            for func in file_info.get("functions", []):
                result.append(f"  \033[32m- {func['name']}\033[0m")
            for cls in file_info.get("classes", []):
                result.append(f"  \033[33m+ {cls['name']}\033[0m")
                for method in cls.get("methods", []):
                    result.append(f"    \033[32m- {method['name']}\033[0m")
        return result
        
    def display(self):
        """显示整个界面"""
        self.clear_screen()
        
        # 显示状态信息
        if self.current_language:
            self.print_header(f"[API Analysis] 正在分析 {self.current_language} 项目", "1;36")
        else:
            self.print_header("[API Analysis] 等待选择项目", "1;36")
        
        # 显示各个区域
        sections = {
            "Environment": self.display_environment(),
            "Analysis": self.display_analysis()
        }
        
        for title, content in sections.items():
            self.print_section(title, content)
            print()
            
    def run(self):
        """运行界面"""
        try:
            while True:
                self.display()
                cmd = input("\033[1;32mapi>\033[0m ").strip()
                
                if cmd in ["q", "quit"]:
                    break
                elif cmd == "refresh":
                    continue
                elif cmd == "help":
                    print("\nCommands:")
                    print("  analyze <language> <directory>  分析指定语言的项目")
                    print("  refresh                        刷新显示")
                    print("  q/quit                        退出程序")
                    print("  help                          显示帮助")
                    print("\nSupported languages:")
                    print("  rust, python, javascript, java, c, cpp")
                    input("\nPress Enter to continue...")
                elif cmd.startswith("analyze"):
                    parts = cmd.split()
                    if len(parts) >= 3:
                        self.run_analyzer(parts[1], parts[2])
                    else:
                        print("用法: analyze <language> <directory>")
                    input("Press Enter to continue...")
                else:
                    print(f"Unknown command: {cmd}")
                    print("Type 'help' for available commands")
                    input("Press Enter to continue...")
                    
        except KeyboardInterrupt:
            print("\nExiting...")
            sys.exit(0)

if __name__ == "__main__":
    dashboard = Dashboard()
    dashboard.run() 