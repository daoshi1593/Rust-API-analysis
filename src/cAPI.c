#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <ctype.h>
#include <libclang/libclang.h>

#define MAX_PATH 4096
#define MAX_LINE 1024
#define MAX_FUNCTIONS 1000

// 函数信息结构
typedef struct {
    char name[256];
    char return_type[256];
    char parameters[1024];
} FunctionInfo;

// 文件信息结构
typedef struct {
    char path[MAX_PATH];
    FunctionInfo functions[MAX_FUNCTIONS];
    int function_count;
} FileInfo;

// 全局变量
FileInfo current_file;
int file_count = 0;
FileInfo* all_files = NULL;

// 检查文件扩展名
int is_c_file(const char* filename) {
    const char* ext = strrchr(filename, '.');
    return ext && (strcmp(ext, ".c") == 0 || strcmp(ext, ".h") == 0);
}

// 解析函数声明
void parse_function_declaration(const char* line) {
    char* line_copy = strdup(line);
    char* token = strtok(line_copy, " \t\n");
    char return_type[256] = "";
    char function_name[256] = "";
    char parameters[1024] = "";
    
    // 解析返回类型
    if (token) {
        strcpy(return_type, token);
        token = strtok(NULL, " \t\n");
    }
    
    // 解析函数名
    if (token) {
        strcpy(function_name, token);
        token = strtok(NULL, "(");
    }
    
    // 解析参数
    if (token) {
        strcpy(parameters, token);
        // 移除结尾的右括号
        char* end = strrchr(parameters, ')');
        if (end) *end = '\0';
    }
    
    // 保存函数信息
    if (current_file.function_count < MAX_FUNCTIONS) {
        strcpy(current_file.functions[current_file.function_count].name, function_name);
        strcpy(current_file.functions[current_file.function_count].return_type, return_type);
        strcpy(current_file.functions[current_file.function_count].parameters, parameters);
        current_file.function_count++;
    }
    
    free(line_copy);
}

// 分析单个文件
void analyze_file(const char* filepath) {
    FILE* file = fopen(filepath, "r");
    if (!file) {
        printf("无法打开文件: %s\n", filepath);
        return;
    }
    
    strcpy(current_file.path, filepath);
    current_file.function_count = 0;
    
    char line[MAX_LINE];
    while (fgets(line, sizeof(line), file)) {
        // 移除注释
        char* comment = strstr(line, "//");
        if (comment) *comment = '\0';
        
        // 检查是否是函数声明
        if (strstr(line, "(") && strstr(line, ")")) {
            parse_function_declaration(line);
        }
    }
    
    fclose(file);
    
    // 保存文件信息
    if (file_count == 0) {
        all_files = malloc(sizeof(FileInfo));
    } else {
        all_files = realloc(all_files, (file_count + 1) * sizeof(FileInfo));
    }
    all_files[file_count++] = current_file;
}

// 递归遍历目录
void walk_directory(const char* dirpath) {
    DIR* dir = opendir(dirpath);
    if (!dir) {
        printf("无法打开目录: %s\n", dirpath);
        return;
    }
    
    struct dirent* entry;
    while ((entry = readdir(dir)) != NULL) {
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }
        
        char path[MAX_PATH];
        snprintf(path, sizeof(path), "%s/%s", dirpath, entry->d_name);
        
        struct stat statbuf;
        if (stat(path, &statbuf) == -1) {
            continue;
        }
        
        if (S_ISDIR(statbuf.st_mode)) {
            walk_directory(path);
        } else if (is_c_file(entry->d_name)) {
            analyze_file(path);
        }
    }
    
    closedir(dir);
}

// 输出JSON格式的结果
void print_json() {
    printf("{\n  \"files\": [\n");
    
    for (int i = 0; i < file_count; i++) {
        printf("    {\n");
        printf("      \"path\": \"%s\",\n", all_files[i].path);
        printf("      \"functions\": [\n");
        
        for (int j = 0; j < all_files[i].function_count; j++) {
            printf("        {\n");
            printf("          \"name\": \"%s\",\n", all_files[i].functions[j].name);
            printf("          \"return_type\": \"%s\",\n", all_files[i].functions[j].return_type);
            printf("          \"parameters\": \"%s\"\n", all_files[i].functions[j].parameters);
            printf("        }%s\n", j < all_files[i].function_count - 1 ? "," : "");
        }
        
        printf("      ]\n");
        printf("    }%s\n", i < file_count - 1 ? "," : "");
    }
    
    printf("  ]\n}\n");
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        printf("请提供目录路径\n");
        return 1;
    }
    
    walk_directory(argv[1]);
    print_json();
    
    if (all_files) {
        free(all_files);
    }
    
    return 0;
} 