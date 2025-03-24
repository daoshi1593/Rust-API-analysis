use std::error::Error;
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::fs;
use serde::{Deserialize, Serialize};
use clap::{App, Arg, SubCommand};

// 支持的语言列表
const SUPPORTED_LANGUAGES: &[&str] = &["rust", "python", "javascript", "java", "c", "cpp"];

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ignore_dirs: Vec<String>,
    output_format: String,
    max_depth: i32,
    log_level: String,
    output_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ignore_dirs: vec!["tests".to_string(), "node_modules".to_string(), "target".to_string(), "venv".to_string(), "__pycache__".to_string()],
            output_format: "text".to_string(),
            max_depth: 5,
            log_level: "info".to_string(),
            output_dir: "~/.parser/output".to_string(),
        }
    }
}

fn load_config() -> Config {
    let config_path = dirs::home_dir()
        .unwrap()
        .join(".parser")
        .join("config.json");

    if let Ok(contents) = fs::read_to_string(&config_path) {
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        Config::default()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = load_config();
    
    let matches = App::new("代码解析工具集")
        .version("1.1.0")
        .author("Your Name")
        .about("分析多种编程语言项目的代码结构")
        .subcommand(SubCommand::with_name("analyze")
            .about("分析指定目录的代码")
            .arg(Arg::with_name("language")
                .help("要分析的语言")
                .required(true)
                .possible_values(SUPPORTED_LANGUAGES))
            .arg(Arg::with_name("directory")
                .help("要分析的目录路径")
                .required(true))
            .arg(Arg::with_name("ignore-dirs")
                .help("要忽略的目录，用逗号分隔")
                .long("ignore-dirs"))
            .arg(Arg::with_name("output")
                .help("输出文件路径")
                .long("output"))
            .arg(Arg::with_name("recursive")
                .help("递归分析依赖")
                .long("recursive"))
            .arg(Arg::with_name("format")
                .help("输出格式 (text/json/html)")
                .long("format")
                .possible_values(&["text", "json", "html"]))
            .arg(Arg::with_name("max-depth")
                .help("最大递归深度")
                .long("max-depth")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("config")
            .about("配置工具")
            .arg(Arg::with_name("show")
                .help("显示当前配置")
                .long("show"))
            .arg(Arg::with_name("set")
                .help("设置配置项")
                .long("set")
                .takes_value(true)))
        .get_matches();

    match matches.subcommand() {
        ("analyze", Some(analyze_matches)) => {
            let language = analyze_matches.value_of("language").unwrap();
            let directory = analyze_matches.value_of("directory").unwrap();
            
            // 处理命令行参数
            let mut config = config;
            if let Some(ignore_dirs) = analyze_matches.value_of("ignore-dirs") {
                config.ignore_dirs = ignore_dirs.split(',').map(String::from).collect();
            }
            if let Some(output) = analyze_matches.value_of("output") {
                config.output_dir = output.to_string();
            }
            if let Some(max_depth) = analyze_matches.value_of("max-depth") {
                config.max_depth = max_depth.parse().unwrap_or(5);
            }
            if let Some(format) = analyze_matches.value_of("format") {
                config.output_format = format.to_string();
            }
            
            // 根据语言选择并运行对应的解析器
            match language {
                "python" => run_python_parser(directory, &config)?,
                "rust" => run_rust_parser(directory, &config)?,
                "javascript" | "js" => run_javascript_parser(directory, &config)?,
                "java" => run_java_parser(directory, &config)?,
                "c" => run_c_parser(directory, &config)?,
                "cpp" | "c++" => run_cpp_parser(directory, &config)?,
                _ => {
                    eprintln!("不支持的语言: {}", language);
                    print_supported_languages();
                    return Err(format!("不支持的语言: {}", language).into());
                }
            }
        }
        ("config", Some(config_matches)) => {
            if config_matches.is_present("show") {
                println!("当前配置:");
                println!("{}", serde_json::to_string_pretty(&config)?);
            } else if let Some(set_value) = config_matches.value_of("set") {
                // 处理配置设置
                let parts: Vec<&str> = set_value.split('=').collect();
                if parts.len() == 2 {
                    let key = parts[0];
                    let value = parts[1];
                    // 更新配置
                    match key {
                        "output_format" => config.output_format = value.to_string(),
                        "max_depth" => config.max_depth = value.parse().unwrap_or(5),
                        "log_level" => config.log_level = value.to_string(),
                        "output_dir" => config.output_dir = value.to_string(),
                        _ => println!("未知的配置项: {}", key),
                    }
                    // 保存配置
                    save_config(&config)?;
                }
            }
        }
        _ => {
            println!("{}", matches.usage());
        }
    }

    Ok(())
}

fn save_config(config: &Config) -> Result<(), Box<dyn Error>> {
    let config_dir = dirs::home_dir()
        .unwrap()
        .join(".parser");
    
    fs::create_dir_all(&config_dir)?;
    
    let config_path = config_dir.join("config.json");
    let contents = serde_json::to_string_pretty(config)?;
    fs::write(config_path, contents)?;
    
    Ok(())
}

fn print_supported_languages() {
    eprintln!("支持的语言:");
    for lang in SUPPORTED_LANGUAGES {
        eprintln!("  - {}", lang);
    }
}

fn run_python_parser(directory: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    println!("运行Python代码解析器...");

    let parser_path = Path::new("pythonAPI.py");
    if !parser_path.exists() {
        return Err(format!("未找到Python解析器脚本: {}", parser_path.display()).into());
    }

    let mut command = Command::new("python3");
    command.arg(parser_path)
           .arg(directory)
           .arg("--format")
           .arg(&config.output_format)
           .arg("--max-depth")
           .arg(config.max_depth.to_string());

    for dir in &config.ignore_dirs {
        command.arg("--ignore-dir").arg(dir);
    }

    let status = command.status()?;
    check_status(status, "Python解析器")
}

fn run_rust_parser(directory: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    println!("运行Rust代码解析器...");

    let parser_path = Path::new("rustAPI.rs");
    if !parser_path.exists() {
        return Err(format!("未找到Rust解析器脚本: {}", parser_path.display()).into());
    }

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
    let mut command = Command::new(output_path);
    command.arg(directory)
           .arg("--format")
           .arg(&config.output_format)
           .arg("--max-depth")
           .arg(config.max_depth.to_string());

    for dir in &config.ignore_dirs {
        command.arg("--ignore-dir").arg(dir);
    }

    let status = command.status()?;
    check_status(status, "Rust解析器")
}

fn run_javascript_parser(directory: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    println!("运行JavaScript代码解析器...");

    let parser_path = Path::new("javascriptAPI.js");
    if !parser_path.exists() {
        return Err(format!("未找到JavaScript解析器脚本: {}", parser_path.display()).into());
    }

    let mut command = Command::new("node");
    command.arg(parser_path)
           .arg(directory)
           .arg("--format")
           .arg(&config.output_format)
           .arg("--max-depth")
           .arg(config.max_depth.to_string());

    for dir in &config.ignore_dirs {
        command.arg("--ignore-dir").arg(dir);
    }

    let status = command.status()?;
    check_status(status, "JavaScript解析器")
}

fn run_java_parser(directory: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    println!("运行Java代码解析器...");

    let parser_path = Path::new("javaAPI.java");
    if !parser_path.exists() {
        return Err(format!("未找到Java解析器脚本: {}", parser_path.display()).into());
    }

    println!("编译Java解析器...");
    let compile_status = Command::new("javac").arg(parser_path).status()?;

    if !compile_status.success() {
        return Err(format!("Java解析器编译失败，退出代码: {:?}", compile_status.code()).into());
    }

    println!("执行Java解析器...");
    let mut command = Command::new("java");
    command.current_dir("parsers")
           .arg("JavaAPI")
           .arg(directory)
           .arg("--format")
           .arg(&config.output_format)
           .arg("--max-depth")
           .arg(config.max_depth.to_string());

    for dir in &config.ignore_dirs {
        command.arg("--ignore-dir").arg(dir);
    }

    let status = command.status()?;
    check_status(status, "Java解析器")
}

fn run_c_parser(directory: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    println!("运行C代码解析器...");

    let parser_path = Path::new("cAPI.c");
    if !parser_path.exists() {
        return Err(format!("未找到C解析器脚本: {}", parser_path.display()).into());
    }

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

    println!("执行C解析器...");
    let mut command = Command::new(output_path);
    command.arg(directory)
           .arg("--format")
           .arg(&config.output_format)
           .arg("--max-depth")
           .arg(config.max_depth.to_string());

    for dir in &config.ignore_dirs {
        command.arg("--ignore-dir").arg(dir);
    }

    let status = command.status()?;
    check_status(status, "C解析器")
}

fn run_cpp_parser(directory: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    println!("运行C++代码解析器...");

    let parser_path = Path::new("cppAPI.cpp");
    if !parser_path.exists() {
        return Err(format!("未找到C++解析器脚本: {}", parser_path.display()).into());
    }

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

    println!("执行C++解析器...");
    let mut command = Command::new(output_path);
    command.arg(directory)
           .arg("--format")
           .arg(&config.output_format)
           .arg("--max-depth")
           .arg(config.max_depth.to_string());

    for dir in &config.ignore_dirs {
        command.arg("--ignore-dir").arg(dir);
    }

    let status = command.status()?;
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
