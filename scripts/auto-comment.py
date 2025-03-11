import os
import argparse
from volcenginesdkarkruntime import Ark

def get_files_with_extensions(directory, extensions, ignore_dirs=None):
    """递归获取指定目录下所有指定扩展名的文件
    
    Args:
        directory: 要搜索的目录路径
        extensions: 文件扩展名列表
        ignore_dirs: 要忽略的目录列表
        
    Returns:
        符合条件的文件路径列表
    """
    if ignore_dirs is None:
        ignore_dirs = []
        
    # 将忽略目录转换为绝对路径和规范化路径
    normalized_ignore_dirs = [os.path.normpath(os.path.abspath(d)) for d in ignore_dirs]
    
    file_paths = []
    for root, dirs, files in os.walk(directory):
        # 检查当前目录是否在忽略列表中
        root_path = os.path.normpath(os.path.abspath(root))
        if any(root_path == d or root_path.startswith(d + os.sep) for d in normalized_ignore_dirs):
            # 如果当前目录在忽略列表中，跳过该目录
            continue
            
        for file in files:
            if any(file.endswith(ext) for ext in extensions):
                file_paths.append(os.path.join(root, file))
    return file_paths

def read_file_content(file_path):
    """读取文件内容"""
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            return file.read()
    except UnicodeDecodeError:
        # 如果utf-8解码失败，尝试其他编码
        try:
            with open(file_path, 'r', encoding='latin1') as file:
                return file.read()
        except Exception as e:
            print(f"无法读取文件 {file_path}: {e}")
            return None

def write_file_content(file_path, content):
    """写入文件内容"""
    try:
        with open(file_path, 'w', encoding='utf-8') as file:
            file.write(content)
        return True
    except Exception as e:
        print(f"无法写入文件 {file_path}: {e}")
        return False

def extract_code_from_markdown(content):
    """从可能的Markdown代码块中提取纯代码内容"""
    # 检查内容是否被代码块包围
    code_block_patterns = [
        r'```[\w]*\n([\s\S]*?)```',  # 标准的代码块格式 ```language\n代码```
        r'`{3,}([\s\S]*?)`{3,}',     # 没有语言标识的代码块格式
        r'```[\w]*\s*([\s\S]*?)```'  # 语言标识后可能有空格的情况
    ]
    
    for pattern in code_block_patterns:
        import re
        matches = re.findall(pattern, content)
        if matches:
            # 返回第一个匹配的代码块内容
            return matches[0].strip()
    
    # 如果没有匹配到代码块，返回原内容
    return content

def get_commented_content(client, model, content, file_ext):
    """调用API获取注释后的内容"""
    prompt = f"""你是一位专业的代码注释助手，你的任务是为提供的代码片段创建清晰、准确和有用的注释。请按照以下要求:

1. 分析代码片段的功能和目的
2. 为函数、类和重要代码块添加注释
3. 遵循代码原本的注释风格(如果存在)
4. 不要修改现有代码或现有注释
5. 针对以下注释类型提供内容:
   - 文件顶部/模块级注释：简明描述文件的整体功能和用途
   - 类注释：解释类的目的、主要功能和使用场景
   - 函数/方法注释：描述功能、参数、返回值和可能的异常
   - 关键代码块注释：解释复杂逻辑或算法
   - 变量注释：仅在变量用途不明显时添加

6. 注释应该使用以下风格(根据编程语言调整):
   - Python: 使用docstring风格 (""" """)
   - JavaScript/TypeScript: 使用JSDoc风格 (/** */)
   - C/C++: 使用标准C风格注释 (/* */ 或 //)
   - Java: 使用Javadoc风格注释 (/** */)

7. 注释应简洁明了，避免冗余或显而易见的内容
8. 对于复杂逻辑，解释"为什么"而不仅仅是"做了什么"
9. 使用专业且准确的技术术语
10. 保持注释与实际代码功能一致

请生成完整的包含注释的代码。不需要解释你的注释决策。

以下是需要注释的代码:\n{content}"""
    
    try:
        completion = client.chat.completions.create(
            model=model,
            messages=[
                {"role": "system", "content": "你是一位资深的程序员，专长于代码分析和添加清晰的注释"},
                {"role": "user", "content": prompt},
            ],
        )
        respsonseContent = completion.choices[0].message.content
        return extract_code_from_markdown(respsonseContent)
    except Exception as e:
        print(f"API调用失败: {e}")
        return None

def main():
    parser = argparse.ArgumentParser(description='为代码文件添加注释')
    parser.add_argument('directory', help='要处理的目录')
    parser.add_argument('--extensions', nargs='+', default=['.py', '.rs'], help='要处理的文件扩展名，例如: .py .rs')
    parser.add_argument('--model', default=os.environ.get("MODEL_NAME"), help='要使用的模型名称')
    parser.add_argument('--ignore-dirs', nargs='+', default=[], help='要忽略的目录，例如: venv node_modules')
    args = parser.parse_args()
    
    # 确保扩展名格式正确（加上点号）
    extensions = [ext if ext.startswith('.') else f'.{ext}' for ext in args.extensions]
    
    # 初始化API客户端
    if not os.environ.get("ARK_API_KEY"):
        print("环境变量ARK_API_KEY未设置，请设置后再运行")
        return
    
    client = Ark(
        base_url="https://ark.cn-beijing.volces.com/api/v3",
        api_key=os.environ.get("ARK_API_KEY"),
        timeout=120,
        max_retries=2,
    )
    
    # 获取所有符合条件的文件，排除忽略目录
    file_paths = get_files_with_extensions(args.directory, extensions, args.ignore_dirs)
    print(f"找到 {len(file_paths)} 个文件")
    
    # 处理每个文件
    for file_path in file_paths:
        print(f"处理文件: {file_path}")
        content = read_file_content(file_path)
        if content is None:
            continue
        
        file_ext = os.path.splitext(file_path)[1]
        commented_content = get_commented_content(client, args.model, content, file_ext)
        
        if commented_content:
            if write_file_content(file_path, commented_content):
                print(f"成功更新文件: {file_path}")
            else:
                print(f"更新文件失败: {file_path}")
        else:
            print(f"获取注释内容失败: {file_path}")

if __name__ == "__main__":
    main()