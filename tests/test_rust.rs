use code_parser::test_utils::{TestDir, assert_json_eq};
use std::process::Command;

#[test]
fn test_rust_parser() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test.rs", r#"
pub fn simple_function() {
    println!("Hello, World!");
}

pub struct TestStruct {
    name: String,
}

impl TestStruct {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    
    pub fn instance_method(&self) -> String {
        format!("Hello, {}!", self.name)
    }
    
    pub fn static_method() -> &'static str {
        "Static method"
    }
}

pub async fn async_function() -> String {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    "Async function".to_string()
}

pub const LAMBDA: fn(i32) -> i32 = |x| x * 2;
"#);

    // 运行解析器
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("analyze")
        .arg("--lang")
        .arg("rust")
        .arg(&test_dir.path)
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败");

    // 验证输出
    let output_str = String::from_utf8_lossy(&output.stdout);
    let expected = r#"{
  "files": [
    {
      "path": "test.rs",
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
          "name": "LAMBDA",
          "type": "arrow",
          "async": false
        }
      ],
      "classes": [
        {
          "name": "TestStruct",
          "methods": [
            {
              "name": "new",
              "type": "constructor",
              "static": true,
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
fn test_rust_parser_with_traits() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test_traits.rs", r#"
use std::fmt::Display;

pub trait Printable {
    fn print(&self);
}

pub struct Data<T: Display> {
    value: T,
}

impl<T: Display> Data<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
    
    pub fn get_value(&self) -> &T {
        &self.value
    }
}

impl<T: Display> Printable for Data<T> {
    fn print(&self) {
        println!("{}", self.value);
    }
}
"#);

    // 运行解析器
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("analyze")
        .arg("--lang")
        .arg("rust")
        .arg(&test_dir.path)
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败");

    // 验证输出
    let output_str = String::from_utf8_lossy(&output.stdout);
    let expected = r#"{
  "files": [
    {
      "path": "test_traits.rs",
      "functions": [],
      "classes": [
        {
          "name": "Data",
          "methods": [
            {
              "name": "new",
              "type": "constructor",
              "static": true,
              "async": false
            },
            {
              "name": "get_value",
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
fn test_rust_parser_with_macros() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test_macros.rs", r#"
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Function {} was called", stringify!($func_name));
        }
    };
}

create_function!(generated_function);

#[derive(Debug)]
pub struct MacroStruct {
    name: String,
}

impl MacroStruct {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
"#);

    // 运行解析器
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("analyze")
        .arg("--lang")
        .arg("rust")
        .arg(&test_dir.path)
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败");

    // 验证输出
    let output_str = String::from_utf8_lossy(&output.stdout);
    let expected = r#"{
  "files": [
    {
      "path": "test_macros.rs",
      "functions": [
        {
          "name": "generated_function",
          "type": "function",
          "async": false
        }
      ],
      "classes": [
        {
          "name": "MacroStruct",
          "methods": [
            {
              "name": "new",
              "type": "constructor",
              "static": true,
              "async": false
            },
            {
              "name": "get_name",
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