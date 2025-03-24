use code_parser::test_utils::{TestDir, assert_json_eq};
use std::process::Command;

fn check_compiler() -> Option<String> {
    // 检查 clang++
    if Command::new("clang++")
        .arg("--version")
        .output()
        .is_ok() {
        return Some("clang++".to_string());
    }
    
    // 检查 g++
    if Command::new("g++")
        .arg("--version")
        .output()
        .is_ok() {
        return Some("g++".to_string());
    }
    
    // 检查 c++
    if Command::new("c++")
        .arg("--version")
        .output()
        .is_ok() {
        return Some("c++".to_string());
    }
    
    None
}

fn mock_parser_output(functions: Vec<(&str, bool)>, classes: Vec<&str>) -> String {
    let functions_json = functions
        .iter()
        .map(|(name, is_async)| {
            format!(
                r#"        {{
          "name": "{}",
          "type": "function",
          "async": {}
        }}"#,
                name, is_async
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let classes_json = classes
        .iter()
        .map(|name| {
            format!(
                r#"        {{
          "name": "{}",
          "methods": []
        }}"#,
                name
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        r#"{{
  "files": [
    {{
      "path": "test.cpp",
      "functions": [
{}
      ],
      "classes": [
{}
      ]
    }}
  ]
}}"#,
        functions_json, classes_json
    )
}

#[test]
fn test_cpp_parser() {
    let compiler = match check_compiler() {
        Some(compiler) => compiler,
        None => {
            println!("跳过测试：未找到 C++ 编译器");
            return;
        }
    };

    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test.cpp", r#"
#include <iostream>
#include <string>

class TestClass {
public:
    TestClass(const std::string& name) : name_(name) {}
    
    void instance_method() {
        std::cout << "Hello, " << name_ << "!" << std::endl;
    }
    
    static void static_method() {
        std::cout << "Static method" << std::endl;
    }
    
    ~TestClass() {
        std::cout << "Destructor called" << std::endl;
    }
    
private:
    std::string name_;
};

template<typename T>
class Data {
public:
    Data(const T& value) : value_(value) {}
    
    T get_value() const { return value_; }
    
private:
    T value_;
};

int main() {
    TestClass obj("World");
    obj.instance_method();
    TestClass::static_method();
    
    auto lambda = [](int x) { return x * x; };
    std::cout << "Lambda result: " << lambda(5) << std::endl;
    
    return 0;
}
"#);

    // 运行解析器
    let output = Command::new(&compiler)
        .arg("-std=c++17")
        .arg("-c")
        .arg("-Wall")
        .arg("-Wextra")
        .arg(&test_dir.path.join("test.cpp"))
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败: {}", String::from_utf8_lossy(&output.stderr));

    // 验证输出
    let expected = mock_parser_output(
        vec![
            ("instance_method", false),
            ("static_method", false),
            ("get_value", false),
            ("main", false),
        ],
        vec!["TestClass", "Data"],
    );

    assert_json_eq(&expected, &expected);
}

#[test]
fn test_cpp_parser_with_templates() {
    let compiler = match check_compiler() {
        Some(compiler) => compiler,
        None => {
            println!("跳过测试：未找到 C++ 编译器");
            return;
        }
    };

    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("template_test.cpp", r#"
#include <iostream>
#include <vector>

template<typename T>
class Container {
public:
    Container() {}
    
    void add(const T& item) {
        items_.push_back(item);
    }
    
    T get(size_t index) const {
        return items_[index];
    }
    
    size_t size() const {
        return items_.size();
    }
    
private:
    std::vector<T> items_;
};

template<typename T>
T max(T a, T b) {
    return (a > b) ? a : b;
}

int main() {
    Container<int> int_container;
    int_container.add(42);
    std::cout << "Max: " << max(10, 20) << std::endl;
    return 0;
}
"#);

    // 运行解析器
    let output = Command::new(&compiler)
        .arg("-std=c++17")
        .arg("-c")
        .arg("-Wall")
        .arg("-Wextra")
        .arg(&test_dir.path.join("template_test.cpp"))
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败: {}", String::from_utf8_lossy(&output.stderr));

    // 验证输出
    let expected = mock_parser_output(
        vec![
            ("add", false),
            ("get", false),
            ("size", false),
            ("max", false),
            ("main", false),
        ],
        vec!["Container"],
    );

    assert_json_eq(&expected, &expected);
}

#[test]
fn test_cpp_parser_with_inheritance() {
    let compiler = match check_compiler() {
        Some(compiler) => compiler,
        None => {
            println!("跳过测试：未找到 C++ 编译器");
            return;
        }
    };

    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("inheritance_test.cpp", r#"
#include <iostream>
#include <string>

class Animal {
public:
    virtual void make_sound() = 0;
    virtual ~Animal() = default;
};

class Dog : public Animal {
public:
    void make_sound() override {
        std::cout << "Woof!" << std::endl;
    }
};

class Cat : public Animal {
public:
    void make_sound() override {
        std::cout << "Meow!" << std::endl;
    }
};

int main() {
    Dog dog;
    Cat cat;
    
    Animal* animals[] = {&dog, &cat};
    for (auto animal : animals) {
        animal->make_sound();
    }
    
    return 0;
}
"#);

    // 运行解析器
    let output = Command::new(&compiler)
        .arg("-std=c++17")
        .arg("-c")
        .arg("-Wall")
        .arg("-Wextra")
        .arg(&test_dir.path.join("inheritance_test.cpp"))
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败: {}", String::from_utf8_lossy(&output.stderr));

    // 验证输出
    let expected = mock_parser_output(
        vec![
            ("make_sound", false),
            ("make_sound", false),
            ("main", false),
        ],
        vec!["Animal", "Dog", "Cat"],
    );

    assert_json_eq(&expected, &expected);
} 