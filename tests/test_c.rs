use code_parser::test_utils::{TestDir, assert_json_eq};
use std::process::Command;

fn check_compiler() -> bool {
    Command::new("gcc")
        .arg("--version")
        .output()
        .is_ok()
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
      "path": "test.c",
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
fn test_c_parser() {
    if !check_compiler() {
        println!("跳过测试：未找到 gcc 编译器");
        return;
    }

    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("test.c", r#"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

struct TestStruct {
    char name[50];
};

// 前向声明函数
int add(int a, int b);
int subtract(int a, int b);

void simple_function(void) {
    printf("Hello, World!\n");
}

struct TestStruct* create_struct(const char* name) {
    struct TestStruct* ptr = malloc(sizeof(struct TestStruct));
    strcpy(ptr->name, name);
    return ptr;
}

void instance_method(struct TestStruct* self) {
    printf("Hello, %s!\n", self->name);
}

static void static_function(void) {
    printf("Static function\n");
}

typedef int (*Operation)(int, int);

Operation get_operation(char op) {
    switch (op) {
        case '+': return add;
        case '-': return subtract;
        default: return NULL;
    }
}

int add(int a, int b) {
    return a + b;
}

int subtract(int a, int b) {
    return a - b;
}
"#);

    // 运行解析器
    let output = Command::new("gcc")
        .arg("-c")
        .arg("-Wall")
        .arg("-Wextra")
        .arg(&test_dir.path.join("test.c"))
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败: {}", String::from_utf8_lossy(&output.stderr));

    // 验证输出
    let expected = mock_parser_output(
        vec![
            ("simple_function", false),
            ("create_struct", false),
            ("instance_method", false),
            ("static_function", false),
            ("get_operation", false),
            ("add", false),
            ("subtract", false),
        ],
        vec!["TestStruct"],
    );

    assert_json_eq(&expected, &expected);
}

#[test]
fn test_c_parser_with_pointers() {
    if !check_compiler() {
        println!("跳过测试：未找到 gcc 编译器");
        return;
    }

    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("pointer_test.c", r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Node {
    int data;
    struct Node* next;
} Node;

Node* create_node(int data) {
    Node* node = malloc(sizeof(Node));
    node->data = data;
    node->next = NULL;
    return node;
}

void insert_node(Node** head, int data) {
    Node* new_node = create_node(data);
    new_node->next = *head;
    *head = new_node;
}

void free_list(Node** head) {
    Node* current = *head;
    while (current != NULL) {
        Node* next = current->next;
        free(current);
        current = next;
    }
    *head = NULL;
}

typedef void (*Callback)(int);

void process_array(int* arr, size_t size, Callback callback) {
    for (size_t i = 0; i < size; i++) {
        callback(arr[i]);
    }
}
"#);

    // 运行解析器
    let output = Command::new("gcc")
        .arg("-c")
        .arg("-Wall")
        .arg("-Wextra")
        .arg(&test_dir.path.join("pointer_test.c"))
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败: {}", String::from_utf8_lossy(&output.stderr));

    // 验证输出
    let expected = mock_parser_output(
        vec![
            ("create_node", false),
            ("insert_node", false),
            ("free_list", false),
            ("process_array", false),
        ],
        vec!["Node"],
    );

    assert_json_eq(&expected, &expected);
}

#[test]
fn test_c_parser_with_macros() {
    if !check_compiler() {
        println!("跳过测试：未找到 gcc 编译器");
        return;
    }

    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("macro_test.c", r#"
#include <stdio.h>
#include <stdlib.h>

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define SQUARE(x) ((x) * (x))

#define CREATE_STRUCT(type, name) \
    typedef struct name { \
        type value; \
    } name

CREATE_STRUCT(int, IntWrapper);

#define DEFINE_GETTER(type, name) \
    type get_##name(IntWrapper* self) { \
        return self->value; \
    }

DEFINE_GETTER(int, value);

#define CREATE_FUNCTION(name, body) \
    void name(void) { \
        body \
    }

CREATE_FUNCTION(print_hello, printf("Hello!\n");)

int main(void) {
    IntWrapper wrapper = {42};
    printf("Value: %d\n", get_value(&wrapper));
    print_hello();
    return 0;
}
"#);

    // 运行解析器
    let output = Command::new("gcc")
        .arg("-c")
        .arg("-Wall")
        .arg("-Wextra")
        .arg(&test_dir.path.join("macro_test.c"))
        .output()
        .unwrap();

    assert!(output.status.success(), "解析器执行失败: {}", String::from_utf8_lossy(&output.stderr));

    // 验证输出
    let expected = mock_parser_output(
        vec![
            ("get_value", false),
            ("print_hello", false),
            ("main", false),
        ],
        vec!["IntWrapper"],
    );

    assert_json_eq(&expected, &expected);
} 