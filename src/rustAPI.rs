use std::env;
use std::fs::{self, File as FsFile};
use std::io::Write;
use std::path::Path;
use syn::{visit::Visit, File, parse_file, Item, ImplItem, TraitItem};
use walkdir::WalkDir;
use anyhow::{Result, anyhow};

struct FunctionVisitor {
    functions: Vec<String>,
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    // 提取普通函数
    fn visit_item(&mut self, node: &'ast Item) {
        if let Item::Fn(item_fn) = node {
            self.functions.push(item_fn.sig.ident.to_string());
        }
        syn::visit::visit_item(self, node);
    }

    // 提取impl块中的方法
    fn visit_impl_item(&mut self, node: &'ast ImplItem) {
        match node {
            ImplItem::Fn(method) => {
                self.functions.push(method.sig.ident.to_string());
            }
            _ => {}
        }
        syn::visit::visit_impl_item(self, node);
    }

    // 提取trait中的方法
    fn visit_trait_item(&mut self, node: &'ast TraitItem) {
        match node {
            TraitItem::Fn(method) => {
                self.functions.push(method.sig.ident.to_string());
            }
            _ => {}
        }
        syn::visit::visit_trait_item(self, node);
    }
}

fn process_file(path: &Path) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    let syntax_tree: File = parse_file(&content)?;
    let mut visitor = FunctionVisitor {
        functions: Vec::new(),
    };
    visitor.visit_file(&syntax_tree);
    Ok(visitor.functions)
}

fn main() -> Result<()> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 获取目录路径
    let dir = if args.len() > 1 {
        // 使用命令行传入的第一个参数作为目录路径
        args[1].clone()
    } else {
        // 如果没有提供命令行参数，尝试从toRead.txt读取
        match fs::read_to_string("src/toRead.txt") {
            Ok(content) => content.trim().to_string(),
            Err(_) => {
                // 如果文件不存在，使用当前目录
                println!("未提供目录参数且无法读取toRead.txt，将使用当前目录");
                ".".to_string()
            }
        }
    };

    println!("读取到目录路径: {}", dir);

    // 检查路径是否存在
    let dir_path = Path::new(&dir);
    if !dir_path.exists() {
        return Err(anyhow!("目录 '{}' 不存在", dir));
    }
    if !dir_path.is_dir() {
        return Err(anyhow!("路径 '{}' 不是一个目录", dir));
    }

    // 构建日志文件路径
    let log_path = if args.len() > 2 {
        // 如果提供了第二个参数，将其作为输出文件路径
        args[2].clone()
    } else {
        // 否则在目标目录下创建fns_log文件
        dir_path.join("fns_log").to_string_lossy().to_string()
    };

    println!("日志文件将写入: {}", log_path);
    // 创建或清空日志文件
    let mut log_file = FsFile::create(&log_path)?;

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            let functions = process_file(entry.path())?;
            if !functions.is_empty() {
                // 写入文件路径
                writeln!(log_file, "文件: {}", entry.path().display())?;
                // 写入函数名称
                for func in functions {
                    writeln!(log_file, "  - {}", func)?;
                }
            }
        }
    }

    println!("函数列表已写入到 {}", log_path);
    Ok(())
}