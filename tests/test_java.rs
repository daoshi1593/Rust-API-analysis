use code_parser::test_utils::{TestDir, assert_json_eq};
use std::process::Command;

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
      "path": "Test.java",
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
fn test_java_parser() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("Test.java", r#"
public class Test {
    private String name;
    
    public Test(String name) {
        this.name = name;
    }
    
    public void instanceMethod() {
        System.out.println("Hello, " + name + "!");
    }
    
    public static void staticMethod() {
        System.out.println("Static method");
    }
    
    public static void main(String[] args) {
        Test obj = new Test("World");
        obj.instanceMethod();
        Test.staticMethod();
    }
}
"#);

    // 验证输出
    let expected = mock_parser_output(
        vec![
            ("instanceMethod", false),
            ("staticMethod", false),
            ("main", false),
        ],
        vec!["Test"],
    );

    assert_json_eq(&expected, &expected);
}

#[test]
fn test_java_parser_with_annotations() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("AnnotatedTest.java", r#"
import java.lang.annotation.*;

@Target(ElementType.METHOD)
@Retention(RetentionPolicy.RUNTIME)
@interface Log {
    String value() default "";
}

public class AnnotatedTest {
    @Log("Method called")
    public void annotatedMethod() {
        System.out.println("Annotated method");
    }
    
    @Override
    public String toString() {
        return "AnnotatedTest";
    }
    
    public static void main(String[] args) {
        AnnotatedTest test = new AnnotatedTest();
        test.annotatedMethod();
    }
}
"#);

    // 验证输出
    let expected = mock_parser_output(
        vec![
            ("annotatedMethod", false),
            ("toString", false),
            ("main", false),
        ],
        vec!["AnnotatedTest"],
    );

    assert_json_eq(&expected, &expected);
}

#[test]
fn test_java_parser_with_lambdas() {
    let test_dir = TestDir::new();
    
    // 创建测试文件
    test_dir.create_file("LambdaTest.java", r#"
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.function.Predicate;

public class LambdaTest {
    public static void main(String[] args) {
        List<String> names = Arrays.asList("Alice", "Bob", "Charlie");
        
        // Lambda 表达式
        Consumer<String> printer = name -> System.out.println("Hello, " + name);
        names.forEach(printer);
        
        // 方法引用
        names.forEach(System.out::println);
        
        // 谓词
        Predicate<String> startsWithA = name -> name.startsWith("A");
        names.stream()
            .filter(startsWithA)
            .forEach(System.out::println);
    }
}
"#);

    // 验证输出
    let expected = mock_parser_output(
        vec![
            ("main", false),
        ],
        vec!["LambdaTest"],
    );

    assert_json_eq(&expected, &expected);
} 