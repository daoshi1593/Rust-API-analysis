#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const parser = require('@babel/parser');
const traverse = require('@babel/traverse').default;
const t = require('@babel/types');
const generate = require('@babel/generator').default;

// 获取命令行参数
const directory = process.argv[2];
if (!directory) {
	console.error('请提供目录路径');
	process.exit(1);
}

// 解析结果
const results = {
	files: []
};

// 递归遍历目录
function walkDir(dir) {
	const files = fs.readdirSync(dir);

	files.forEach(file => {
		const filePath = path.join(dir, file);
		const stat = fs.statSync(filePath);

		if (stat.isDirectory()) {
			walkDir(filePath);
		} else if (file.match(/\.(js|jsx|ts|tsx)$/)) {
			analyzeFile(filePath);
		}
	});
}

// 分析单个文件
function analyzeFile(filePath) {
	const content = fs.readFileSync(filePath, 'utf-8');
	const ast = parser.parse(content, {
		sourceType: 'module',
		plugins: [
			'jsx',
			'typescript',
			'decorators-legacy',
			'classProperties',
			'exportDefaultFrom',
			'exportNamespaceFrom'
		]
	});

	const fileInfo = {
		path: filePath,
		functions: [],
		classes: []
	};

	traverse(ast, {
		FunctionDeclaration(path) {
			fileInfo.functions.push({
				name: path.node.id.name,
				type: 'function',
				async: path.node.async
			});
		},
		FunctionExpression(path) {
			if (path.parent.type === 'VariableDeclarator') {
				fileInfo.functions.push({
					name: path.parent.id.name,
					type: 'function',
					async: path.node.async
				});
			}
		},
		ArrowFunctionExpression(path) {
			if (path.parent.type === 'VariableDeclarator') {
				fileInfo.functions.push({
					name: path.parent.id.name,
					type: 'arrow',
					async: path.node.async
				});
			}
		},
		ClassDeclaration(path) {
			const classInfo = {
				name: path.node.id.name,
				methods: []
			};

			path.node.body.body.forEach(member => {
				if (member.type === 'ClassMethod') {
					classInfo.methods.push({
						name: member.key.name,
						type: member.kind,
						static: member.static,
						async: member.async
					});
				}
			});

			fileInfo.classes.push(classInfo);
		}
	});

	results.files.push(fileInfo);
}

// 开始分析
try {
	walkDir(directory);

	// 输出结果
	console.log(JSON.stringify(results, null, 2));
} catch (error) {
	console.error('分析过程中出错:', error);
	process.exit(1);
} 