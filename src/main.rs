use std::fs::{self, File as FsFile};
use std::io::Write;
use std::path::Path;
use syn::{visit::Visit, File};
use walkdir::WalkDir;

struct FunctionVisitor {
    functions: Vec<String>,
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    // 提取普通函数
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.functions.push(node.sig.ident.to_string());
        syn::visit::visit_item_fn(self, node);
    }

    // 提取impl块中的方法
    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.functions.push(node.sig.ident.to_string());
        syn::visit::visit_impl_item_fn(self, node);
    }

    // 提取trait中的方法
    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        self.functions.push(node.sig.ident.to_string());
        syn::visit::visit_trait_item_fn(self, node);
    }
}

fn process_file(path: &Path) -> Result<Vec<String>, anyhow::Error> {
    let content = fs::read_to_string(path)?;
    let syntax_tree: File = syn::parse_file(&content)?;
    let mut visitor = FunctionVisitor {
        functions: Vec::new(),
    };
    visitor.visit_file(&syntax_tree);
    Ok(visitor.functions)
}

fn main() -> Result<(), anyhow::Error> {
    //读取toRead.txt文件中的路径
    // 函数中的代码
    let dir = fs::read_to_string("src/toRead.txt")
        .map_err(|e| anyhow::anyhow!("无法读取 toRead.txt: {}", e))?
        .trim()
        .to_string(); // 去除可能的空白字符和换行符

    println!("读取到目录路径: {}", dir);

    // 检查路径是否存在
    let dir_path = Path::new(&dir);
    if !dir_path.exists() {
        return Err(anyhow::anyhow!("目录 '{}' 不存在", dir));
    }
    if !dir_path.is_dir() {
        return Err(anyhow::anyhow!("路径 '{}' 不是一个目录", dir));
    }

    // 构建日志文件路径
    let log_path = dir_path.join("fns_log").to_string_lossy().to_string();
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
