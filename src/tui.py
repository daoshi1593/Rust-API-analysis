from textual.app import App, ComposeResult
from textual.containers import Container, Vertical
from textual.widgets import Header, Footer, Button, Static, Input
from textual.screen import Screen
from textual import events
import subprocess
import os

class MainMenu(Screen):
    """主菜单界面"""
    
    def compose(self) -> ComposeResult:
        yield Header()
        yield Container(
            Static("API 分析工具", classes="title"),
            Vertical(
                Button("检查环境", id="check_env", variant="primary"),
                Button("分析 Rust 项目", id="analyze_rust", variant="primary"),
                Button("分析 Python 项目", id="analyze_python", variant="primary"),
                Button("分析 JavaScript 项目", id="analyze_js", variant="primary"),
                Button("分析 Java 项目", id="analyze_java", variant="primary"),
                Button("分析 C/C++ 项目", id="analyze_cpp", variant="primary"),
                Button("退出", id="exit", variant="error"),
                classes="buttons",
            ),
            classes="main",
        )
        yield Footer()

    def on_button_pressed(self, event: Button.Pressed) -> None:
        button_id = event.button.id
        if button_id == "check_env":
            self.app.push_screen(EnvCheck())
        elif button_id == "exit":
            self.app.exit()
        else:
            self.app.push_screen(ProjectAnalysis(button_id))

class EnvCheck(Screen):
    """环境检查界面"""
    
    def compose(self) -> ComposeResult:
        yield Header()
        yield Container(
            Static("环境检查", classes="title"),
            Static(self.check_environment(), classes="env_status"),
            Button("返回", id="back", variant="primary"),
            classes="env_check",
        )
        yield Footer()

    def check_environment(self) -> str:
        status = []
        tools = {
            "Rust": ("rustc", "rustc --version | cut -d' ' -f2"),
            "Python": ("python3", "python3 --version | cut -d' ' -f2"),
            "Node.js": ("node", "node --version | cut -d'v' -f2"),
            "Java": ("java", "java -version 2>&1 | head -n 1 | cut -d'\"' -f2"),
            "Clang": ("clang", "clang --version | head -n 1 | cut -d' ' -f3")
        }
        
        for name, (cmd, version_cmd) in tools.items():
            try:
                # 检查命令是否存在
                subprocess.run([cmd, "--version"], capture_output=True, check=True)
                # 获取版本信息
                version = subprocess.check_output(version_cmd, shell=True).decode().strip()
                status.append(f"✓ {name}: 已安装 (v{version})")
            except subprocess.CalledProcessError:
                status.append(f"✗ {name}: 未安装")
            except Exception as e:
                status.append(f"✗ {name}: 检查失败 ({str(e)})")
        
        return "\n".join(status)

    def on_button_pressed(self, event: Button.Pressed) -> None:
        self.app.pop_screen()

class ProjectAnalysis(Screen):
    """项目分析界面"""
    
    def __init__(self, project_type: str):
        super().__init__()
        self.project_type = project_type
        self.commands = {
            "analyze_rust": "./src/rustAPI",
            "analyze_python": "python src/pythonAPI.py",
            "analyze_js": "node src/javascriptAPI.js",
            "analyze_java": "java -cp . src.JavaAPI",
            "analyze_cpp": "./src/parser"
        }

    def compose(self) -> ComposeResult:
        yield Header()
        yield Container(
            Static(f"分析 {self.project_type.replace('analyze_', '').title()} 项目", classes="title"),
            Input(placeholder="请输入项目目录路径", id="path_input"),
            Button("开始分析", id="analyze", variant="primary"),
            Button("返回", id="back", variant="default"),
            classes="analysis",
        )
        yield Footer()

    def on_button_pressed(self, event: Button.Pressed) -> None:
        if event.button.id == "back":
            self.app.pop_screen()
        elif event.button.id == "analyze":
            path = self.query_one("#path_input").value
            if not path or not os.path.isdir(path):
                self.notify("目录不存在！", severity="error")
                return
            
            cmd = f"{self.commands[self.project_type]} {path}"
            try:
                subprocess.run(cmd.split(), cwd=path, check=True)
                self.notify("分析完成！", severity="success")
            except subprocess.CalledProcessError as e:
                self.notify(f"分析失败：{str(e)}", severity="error")

class APIAnalyzer(App):
    """API 分析工具主应用"""
    
    CSS = """
    Screen {
        align: center middle;
        background: #1a1a1a;
        color: #ffffff;
    }
    
    Header {
        background: #2d2d2d;
        color: #ffffff;
        text-style: bold;
        padding: 1;
        text-align: center;
    }
    
    Footer {
        background: #2d2d2d;
        color: #ffffff;
        padding: 1;
        text-align: center;
    }
    
    .title {
        content-align: center middle;
        text-style: bold;
        margin: 1 2;
        padding: 1;
        color: #ffffff;
        font-size: 1.2em;
    }
    
    .main {
        width: 60;
        height: auto;
        background: #2d2d2d;
        padding: 1;
        border-radius: 1;
    }
    
    .buttons {
        height: auto;
        margin: 1 2;
    }
    
    .env_check {
        width: 60;
        height: auto;
        background: #2d2d2d;
        padding: 1;
        border-radius: 1;
    }
    
    .env_status {
        margin: 1 2;
        padding: 1;
        color: #ffffff;
    }
    
    .analysis {
        width: 60;
        height: auto;
        background: #2d2d2d;
        padding: 1;
        border-radius: 1;
    }
    
    Button {
        margin: 1 2;
        width: 100%;
        background: #3d3d3d;
        color: #ffffff;
        border: none;
        border-radius: 1;
        padding: 1;
        transition: background 500ms;
    }
    
    Button:hover {
        background: #4d4d4d;
    }
    
    Button:focus {
        background: #5d5d5d;
        border: none;
    }
    
    Button#exit {
        background: #3d1a1a;
    }
    
    Button#exit:hover {
        background: #4d2a2a;
    }
    
    Button#exit:focus {
        background: #5d3a3a;
    }
    
    Input {
        margin: 1 2;
        width: 100%;
        background: #3d3d3d;
        color: #ffffff;
        border: none;
        border-radius: 1;
        padding: 1;
    }
    
    Input:focus {
        background: #4d4d4d;
        border: none;
    }
    
    #path_input {
        background: #3d3d3d;
        color: #ffffff;
    }
    
    #path_input::placeholder {
        color: #888888;
    }
    """
    
    def on_mount(self) -> None:
        self.push_screen(MainMenu())

if __name__ == "__main__":
    app = APIAnalyzer()
    app.run() 