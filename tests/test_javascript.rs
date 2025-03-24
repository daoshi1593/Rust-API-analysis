use code_parser::test_utils::{TestDir, assert_json_eq};
use std::process::Command;

#[test]
fn test_javascript_parser() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test.js", r#"
function simpleFunction() {
    console.log("Hello, World!");
}

class TestClass {
    constructor(name) {
        this.name = name;
    }
    
    instanceMethod() {
        return `Hello, ${this.name}!`;
    }
    
    static staticMethod() {
        return "Static method";
    }
}

async function asyncFunction() {
    await new Promise(resolve => setTimeout(resolve, 1000));
    return "Async function";
}

const lambdaFunction = x => x * 2;
"#);

    // 运行解析器
    let output = Command::new("node")
        .arg("src/javascriptAPI.js")
        .arg(&test_dir.path)
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败");

    // 验证输出
    let output_str = String::from_utf8_lossy(&output.stdout);
    let expected = r#"{
  "files": [
    {
      "path": "test.js",
      "functions": [
        {
          "name": "simpleFunction",
          "type": "function",
          "async": false
        },
        {
          "name": "asyncFunction",
          "type": "function",
          "async": true
        },
        {
          "name": "lambdaFunction",
          "type": "arrow",
          "async": false
        }
      ],
      "classes": [
        {
          "name": "TestClass",
          "methods": [
            {
              "name": "constructor",
              "type": "constructor",
              "static": false,
              "async": false
            },
            {
              "name": "instanceMethod",
              "type": "method",
              "static": false,
              "async": false
            },
            {
              "name": "staticMethod",
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
fn test_javascript_parser_with_modules() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test_modules.js", r#"
import { processData } from './utils.js';

export class DataProcessor {
    constructor(config) {
        this.config = config;
    }
    
    async process(inputData) {
        return await processData(inputData);
    }
}

export const processData = (data) => {
    return data.map(item => item.name);
};
"#);

    // 运行解析器
    let output = Command::new("node")
        .arg("src/javascriptAPI.js")
        .arg(&test_dir.path)
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败");

    // 验证输出
    let output_str = String::from_utf8_lossy(&output.stdout);
    let expected = r#"{
  "files": [
    {
      "path": "test_modules.js",
      "functions": [
        {
          "name": "processData",
          "type": "arrow",
          "async": false
        }
      ],
      "classes": [
        {
          "name": "DataProcessor",
          "methods": [
            {
              "name": "constructor",
              "type": "constructor",
              "static": false,
              "async": false
            },
            {
              "name": "process",
              "type": "method",
              "static": false,
              "async": true
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
fn test_javascript_parser_with_decorators() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test_decorators.js", r#"
function cacheResult(target, propertyKey, descriptor) {
    const originalMethod = descriptor.value;
    descriptor.value = function(...args) {
        return originalMethod.apply(this, args);
    };
    return descriptor;
}

class DecoratorTest {
    constructor(name) {
        this.name = name;
    }
    
    @cacheResult
    get name() {
        return this._name;
    }
    
    @cacheResult
    static cachedMethod() {
        return "cached";
    }
}
"#);

    // 运行解析器
    let output = Command::new("node")
        .arg("src/javascriptAPI.js")
        .arg(&test_dir.path)
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败");

    // 验证输出
    let output_str = String::from_utf8_lossy(&output.stdout);
    let expected = r#"{
  "files": [
    {
      "path": "test_decorators.js",
      "functions": [
        {
          "name": "cacheResult",
          "type": "function",
          "async": false
        }
      ],
      "classes": [
        {
          "name": "DecoratorTest",
          "methods": [
            {
              "name": "constructor",
              "type": "constructor",
              "static": false,
              "async": false
            },
            {
              "name": "name",
              "type": "getter",
              "static": false,
              "async": false
            },
            {
              "name": "cachedMethod",
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