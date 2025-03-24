import com.github.javaparser.JavaParser;
import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.ast.body.ClassOrInterfaceDeclaration;
import com.github.javaparser.ast.body.MethodDeclaration;
import com.github.javaparser.ast.body.FieldDeclaration;
import com.github.javaparser.ast.type.ClassOrInterfaceType;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class JavaAPI {
    public static void main(String[] args) {
        if (args.length != 1) {
            System.err.println("Usage: java JavaAPI <project_path>");
            System.exit(1);
        }

        String projectPath = args[0];
        File projectDir = new File(projectPath);
        if (!projectDir.exists() || !projectDir.isDirectory()) {
            System.err.println("Invalid project path: " + projectPath);
            System.exit(1);
        }

        Map<String, Object> results = new HashMap<>();
        List<Map<String, Object>> classes = new ArrayList<>();

        // 遍历项目目录
        traverseDirectory(projectDir, classes);

        results.put("classes", classes);

        // 使用 Gson 输出 JSON
        Gson gson = new GsonBuilder().setPrettyPrinting().create();
        System.out.println(gson.toJson(results));
    }

    private static void traverseDirectory(File dir, List<Map<String, Object>> classes) {
        File[] files = dir.listFiles();
        if (files != null) {
            for (File file : files) {
                if (file.isDirectory()) {
                    traverseDirectory(file, classes);
                } else if (file.getName().endsWith(".java")) {
                    analyzeJavaFile(file, classes);
                }
            }
        }
    }

    private static void analyzeJavaFile(File file, List<Map<String, Object>> classes) {
        try {
            JavaParser javaParser = new JavaParser();
            CompilationUnit cu = javaParser.parse(file).getResult().orElse(null);
            if (cu == null) return;

            cu.findAll(ClassOrInterfaceDeclaration.class).forEach(classDecl -> {
                Map<String, Object> classInfo = new HashMap<>();
                classInfo.put("name", classDecl.getNameAsString());
                classInfo.put("path", file.getPath());

                List<Map<String, Object>> methods = new ArrayList<>();
                classDecl.findAll(MethodDeclaration.class).forEach(method -> {
                    Map<String, Object> methodInfo = new HashMap<>();
                    methodInfo.put("name", method.getNameAsString());
                    methodInfo.put("returnType", method.getType().asString());
                    methods.add(methodInfo);
                });
                classInfo.put("methods", methods);

                List<Map<String, Object>> fields = new ArrayList<>();
                classDecl.findAll(FieldDeclaration.class).forEach(field -> {
                    Map<String, Object> fieldInfo = new HashMap<>();
                    fieldInfo.put("name", field.getVariables().get(0).getNameAsString());
                    fieldInfo.put("type", field.getCommonType().asString());
                    fields.add(fieldInfo);
                });
                classInfo.put("fields", fields);

                classes.add(classInfo);
            });
        } catch (FileNotFoundException e) {
            System.err.println("Error reading file: " + file.getPath());
        }
    }
} 