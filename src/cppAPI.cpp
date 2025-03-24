#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <filesystem>
#include <clang-c/Index.h>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

// 函数信息结构
struct FunctionInfo {
    std::string name;
    std::string return_type;
    std::string parameters;
    bool is_template;
    bool is_virtual;
    bool is_const;
};

// 类信息结构
struct ClassInfo {
    std::string name;
    std::vector<FunctionInfo> methods;
    std::vector<std::string> base_classes;
    bool is_template;
};

// 文件信息结构
struct FileInfo {
    std::string path;
    std::vector<FunctionInfo> functions;
    std::vector<ClassInfo> classes;
};

// 全局变量
std::vector<FileInfo> all_files;

// 检查文件扩展名
bool is_cpp_file(const std::string& filename) {
    return filename.ends_with(".cpp") || 
           filename.ends_with(".hpp") || 
           filename.ends_with(".cc") || 
           filename.ends_with(".hh");
}

// 使用 libclang 分析文件
void analyze_with_libclang(const std::string& filepath, FileInfo& file_info) {
    CXIndex index = clang_createIndex(0, 0);
    CXTranslationUnit unit = clang_parseTranslationUnit(
        index,
        filepath.c_str(),
        nullptr,
        0,
        nullptr,
        0,
        CXTranslationUnit_None
    );

    if (unit == nullptr) {
        std::cerr << "无法解析文件: " << filepath << std::endl;
        return;
    }

    CXCursor cursor = clang_getTranslationUnitCursor(unit);
    clang_visitChildren(
        cursor,
        [](CXCursor c, CXCursor parent, CXClientData client_data) {
            FileInfo* file_info = static_cast<FileInfo*>(client_data);
            
            if (clang_getCursorKind(c) == CXCursor_FunctionDecl) {
                FunctionInfo func;
                CXString name = clang_getCursorSpelling(c);
                CXString type = clang_getCursorTypeSpelling(clang_getCursorType(c));
                
                func.name = clang_getCString(name);
                func.return_type = clang_getCString(type);
                func.is_template = clang_Cursor_isFunctionTemplate(c);
                
                file_info->functions.push_back(func);
                
                clang_disposeString(name);
                clang_disposeString(type);
            }
            else if (clang_getCursorKind(c) == CXCursor_ClassDecl) {
                ClassInfo cls;
                CXString name = clang_getCursorSpelling(c);
                cls.name = clang_getCString(name);
                cls.is_template = clang_Cursor_isTemplate(c);
                
                file_info->classes.push_back(cls);
                
                clang_disposeString(name);
            }
            
            return CXChildVisit_Recurse;
        },
        &file_info
    );

    clang_disposeTranslationUnit(unit);
    clang_disposeIndex(index);
}

// 分析单个文件
void analyze_file(const std::string& filepath) {
    FileInfo file_info;
    file_info.path = filepath;
    
    analyze_with_libclang(filepath, file_info);
    all_files.push_back(file_info);
}

// 递归遍历目录
void walk_directory(const std::string& dirpath) {
    for (const auto& entry : std::filesystem::recursive_directory_iterator(dirpath)) {
        if (entry.is_regular_file() && is_cpp_file(entry.path().string())) {
            analyze_file(entry.path().string());
        }
    }
}

// 输出JSON格式的结果
void print_json() {
    json result;
    result["files"] = json::array();
    
    for (const auto& file : all_files) {
        json file_json;
        file_json["path"] = file.path;
        
        // 添加函数信息
        file_json["functions"] = json::array();
        for (const auto& func : file.functions) {
            json func_json;
            func_json["name"] = func.name;
            func_json["return_type"] = func.return_type;
            func_json["is_template"] = func.is_template;
            func_json["is_virtual"] = func.is_virtual;
            func_json["is_const"] = func.is_const;
            file_json["functions"].push_back(func_json);
        }
        
        // 添加类信息
        file_json["classes"] = json::array();
        for (const auto& cls : file.classes) {
            json cls_json;
            cls_json["name"] = cls.name;
            cls_json["is_template"] = cls.is_template;
            cls_json["base_classes"] = cls.base_classes;
            
            cls_json["methods"] = json::array();
            for (const auto& method : cls.methods) {
                json method_json;
                method_json["name"] = method.name;
                method_json["return_type"] = method.return_type;
                method_json["is_template"] = method.is_template;
                method_json["is_virtual"] = method.is_virtual;
                method_json["is_const"] = method.is_const;
                cls_json["methods"].push_back(method_json);
            }
            
            file_json["classes"].push_back(cls_json);
        }
        
        result["files"].push_back(file_json);
    }
    
    std::cout << result.dump(2) << std::endl;
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cerr << "请提供目录路径" << std::endl;
        return 1;
    }
    
    walk_directory(argv[1]);
    print_json();
    
    return 0;
} 