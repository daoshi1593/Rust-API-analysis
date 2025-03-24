use code_parser::test_utils::{TestDir, assert_json_eq};
use std::process::Command;

#[test]
fn test_python_parser() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test.py", r#"
def simple_function():
    print("Hello, World!")

class TestClass:
    def __init__(self, name):
        self.name = name
    
    def instance_method(self):
        return f"Hello, {self.name}!"
    
    @staticmethod
    def static_method():
        return "Static method"
    
    @classmethod
    def class_method(cls):
        return "Class method"

async def async_function():
    await asyncio.sleep(1)
    return "Async function"

lambda_function = lambda x: x * 2
"#);

    // 运行解析器
    let output = Command::new("python3")
        .arg("src/pythonAPI.py")
        .arg(&test_dir.path)
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败");

    // 验证输出
    let output_str = String::from_utf8_lossy(&output.stdout);
    let expected = r#"{
  "files": [
    {
      "path": "test.py",
      "functions": [
        {
          "name": "simple_function",
          "type": "function",
          "async": false
        },
        {
          "name": "async_function",
          "type": "function",
          "async": true
        },
        {
          "name": "lambda_function",
          "type": "arrow",
          "async": false
        }
      ],
      "classes": [
        {
          "name": "TestClass",
          "methods": [
            {
              "name": "__init__",
              "type": "constructor",
              "static": false,
              "async": false
            },
            {
              "name": "instance_method",
              "type": "method",
              "static": false,
              "async": false
            },
            {
              "name": "static_method",
              "type": "method",
              "static": true,
              "async": false
            },
            {
              "name": "class_method",
              "type": "method",
              "static": false,
              "async": false
            }
          ]
        }
      ]
    }
  ]
}"#;

    assert_json_eq(&output_str, expected);
}

#[test]
fn test_python_parser_with_imports() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test_imports.py", r#"
from typing import List, Dict
import os
import sys

def process_data(data: List[Dict[str, str]]) -> List[str]:
    return [item["name"] for item in data]

class DataProcessor:
    def __init__(self, config: Dict[str, str]):
        self.config = config
    
    def process(self, input_data: List[Dict[str, str]]) -> List[str]:
        return process_data(input_data)
"#);

    // 运行解析器
    let output = Command::new("python3")
        .arg("src/pythonAPI.py")
        .arg(&test_dir.path)
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败");

    // 验证输出
    let output_str = String::from_utf8_lossy(&output.stdout);
    let expected = r#"{
  "files": [
    {
      "path": "test_imports.py",
      "functions": [
        {
          "name": "process_data",
          "type": "function",
          "async": false
        }
      ],
      "classes": [
        {
          "name": "DataProcessor",
          "methods": [
            {
              "name": "__init__",
              "type": "constructor",
              "static": false,
              "async": false
            },
            {
              "name": "process",
              "type": "method",
              "static": false,
              "async": false
            }
          ]
        }
      ]
    }
  ]
}"#;

    assert_json_eq(&output_str, expected);
}

#[test]
fn test_python_parser_with_decorators() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test_decorators.py", r#"
def cache_result(func):
    def wrapper(*args, **kwargs):
        return func(*args, **kwargs)
    return wrapper

@cache_result
def cached_function(x: int) -> int:
    return x * x

class DecoratorTest:
    @property
    def name(self) -> str:
        return "test"
    
    @classmethod
    @cache_result
    def cached_class_method(cls) -> str:
        return "cached"
"#);

    // 运行解析器
    let output = Command::new("python3")
        .arg("src/pythonAPI.py")
        .arg(&test_dir.path)
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败");

    // 验证输出
    let output_str = String::from_utf8_lossy(&output.stdout);
    let expected = r#"{
  "files": [
    {
      "path": "test_decorators.py",
      "functions": [
        {
          "name": "cache_result",
          "type": "function",
          "async": false
        },
        {
          "name": "cached_function",
          "type": "function",
          "async": false
        }
      ],
      "classes": [
        {
          "name": "DecoratorTest",
          "methods": [
            {
              "name": "name",
              "type": "property",
              "static": false,
              "async": false
            },
            {
              "name": "cached_class_method",
              "type": "method",
              "static": true,
              "async": false
            }
          ]
        }
      ]
    }
  ]
}"#;

    assert_json_eq(&output_str, expected);
} 