use std::env;
use std::error::Error;
use std::path::Path;
use std::process::{Command, ExitStatus};

// 支持的语言列表
const SUPPORTED_LANGUAGES: &[&str] = &["rust", "python", "javascript", "java", "c", "cpp"];

fn main() -> Result<(), Box<dyn Error>> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 验证参数数量
    if args.len() < 3 {
        print_usage();
        return Err("参数不足".into());
    }

    let language = args[1].to_lowercase();
    let directory = &args[2];

    // 检查目录是否存在
    let dir_path = Path::new(directory);
    if !dir_path.exists() {
        return Err(format!("目录 '{}' 不存在", directory).into());
    }
    if !dir_path.is_dir() {
        return Err(format!("路径 '{}' 不是一个目录", directory).into());
    }

    // 根据语言选择并运行对应的解析器
    match language.as_str() {
        "python" => run_python_parser(directory)?,
        "rust" => run_rust_parser(directory)?,
        "javascript" | "js" => run_javascript_parser(directory)?,
        "java" => run_java_parser(directory)?,
        "c" => run_c_parser(directory)?,
        "cpp" | "c++" => run_cpp_parser(directory)?,
        _ => {
            eprintln!("不支持的语言: {}", language);
            print_supported_languages();
            return Err(format!("不支持的语言: {}", language).into());
        }
    }

    Ok(())
}

fn print_usage() {
    eprintln!("用法: <语言> <目录路径>");
    eprintln!("例如: python /path/to/project");
    print_supported_languages();
}

fn print_supported_languages() {
    eprintln!("支持的语言:");
    for lang in SUPPORTED_LANGUAGES {
        eprintln!("  - {}", lang);
    }
}

fn run_python_parser(directory: &str) -> Result<(), Box<dyn Error>> {
    println!("运行Python代码解析器...");

    // 查找pythonAPI.py文件
    let parser_path = Path::new("pythonAPI.py");
    if !parser_path.exists() {
        return Err(format!("未找到Python解析器脚本: {}", parser_path.display()).into());
    }

    // 运行Python解析器脚本
    let status = Command::new("python3")
        // 可以根据系统调整为python3
        .arg(parser_path)
        .arg(directory)
        .status()?;

    check_status(status, "Python解析器")
}

fn run_rust_parser(directory: &str) -> Result<(), Box<dyn Error>> {
    println!("运行Rust代码解析器...");

    // 查找rustAPI.rs文件
    let parser_path = Path::new("rustAPI.rs");
    if !parser_path.exists() {
        return Err(format!("未找到Rust解析器脚本: {}", parser_path.display()).into());
    }

    // 即时编译并运行Rust文件
    println!("编译Rust解析器...");
    let output_path = "rustAPI";

    let compile_status = Command::new("rustc")
        .arg(parser_path)
        .arg("-o")
        .arg(output_path)
        .status()?;

    if !compile_status.success() {
        return Err(format!("Rust解析器编译失败，退出代码: {:?}", compile_status.code()).into());
    }

    println!("执行Rust解析器...");
    let status = Command::new(output_path).arg(directory).status()?;

    check_status(status, "Rust解析器")
}

fn run_javascript_parser(directory: &str) -> Result<(), Box<dyn Error>> {
    println!("运行JavaScript代码解析器...");

    let parser_path = Path::new("javascriptAPI.js");
    if !parser_path.exists() {
        return Err(format!("未找到JavaScript解析器脚本: {}", parser_path.display()).into());
    }

    let status = Command::new("node")
        .arg(parser_path)
        .arg(directory)
        .status()?;

    check_status(status, "JavaScript解析器")
}

fn run_java_parser(directory: &str) -> Result<(), Box<dyn Error>> {
    println!("运行Java代码解析器...");

    let parser_path = Path::new("javaAPI.java");
    if !parser_path.exists() {
        return Err(format!("未找到Java解析器脚本: {}", parser_path.display()).into());
    }

    // 先编译Java文件
    println!("编译Java解析器...");
    let compile_status = Command::new("javac").arg(parser_path).status()?;

    if !compile_status.success() {
        return Err(format!("Java解析器编译失败，退出代码: {:?}", compile_status.code()).into());
    }

    // 然后运行Java类
    // 注意：假设类名为JavaAPI，与文件名一致（不含扩展名）
    println!("执行Java解析器...");
    let status = Command::new("java")
        .current_dir("parsers") // 切换到parsers目录执行
        .arg("JavaAPI") // 类名，不包含.class后缀
        .arg(directory)
        .status()?;

    check_status(status, "Java解析器")
}

fn run_c_parser(directory: &str) -> Result<(), Box<dyn Error>> {
    println!("运行C代码解析器...");

    let parser_path = Path::new("cAPI.c");
    if !parser_path.exists() {
        return Err(format!("未找到C解析器脚本: {}", parser_path.display()).into());
    }

    // 编译C文件
    println!("编译C解析器...");
    let output_path = "cAPI";

    let compile_status = Command::new("gcc")
        .arg(parser_path)
        .arg("-o")
        .arg(output_path)
        .status()?;

    if !compile_status.success() {
        return Err(format!("C解析器编译失败，退出代码: {:?}", compile_status.code()).into());
    }

    // 执行编译后的程序
    println!("执行C解析器...");
    let status = Command::new(output_path).arg(directory).status()?;

    check_status(status, "C解析器")
}

fn run_cpp_parser(directory: &str) -> Result<(), Box<dyn Error>> {
    println!("运行C++代码解析器...");

    let parser_path = Path::new("cppAPI.cpp");
    if !parser_path.exists() {
        return Err(format!("未找到C++解析器脚本: {}", parser_path.display()).into());
    }

    // 编译C++文件
    println!("编译C++解析器...");
    let output_path = "cppAPI";

    let compile_status = Command::new("g++")
        .arg(parser_path)
        .arg("-o")
        .arg(output_path)
        .status()?;

    if !compile_status.success() {
        return Err(format!("C++解析器编译失败，退出代码: {:?}", compile_status.code()).into());
    }

    // 执行编译后的程序
    println!("执行C++解析器...");
    let status = Command::new(output_path).arg(directory).status()?;

    check_status(status, "C++解析器")
}

fn check_status(status: ExitStatus, parser_name: &str) -> Result<(), Box<dyn Error>> {
    if status.success() {
        println!("{}成功完成！", parser_name);
        Ok(())
    } else {
        Err(format!("{}执行失败，退出代码: {:?}", parser_name, status.code()).into())
    }
}
