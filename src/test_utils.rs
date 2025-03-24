use std::fs;
use std::io::Write;
use serde_json::Value;
use std::env;

pub struct TestDir {
    pub path: std::path::PathBuf,
}

impl TestDir {
    pub fn new() -> Self {
        let temp_dir = env::temp_dir().join("code_parser_test");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap_or_default();
        }
        fs::create_dir_all(&temp_dir).unwrap();
        Self {
            path: temp_dir,
        }
    }

    pub fn create_file(&self, name: &str, contents: &str) {
        let file_path = self.path.join(name);
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    pub fn create_dir(&self, name: &str) {
        let dir_path = self.path.join(name);
        fs::create_dir(dir_path).unwrap();
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_dir_all(&self.path).unwrap_or_default();
        }
    }
}

pub fn assert_json_eq(actual: &str, expected: &str) {
    let actual_value: Value = serde_json::from_str(actual).unwrap();
    let expected_value: Value = serde_json::from_str(expected).unwrap();
    assert_eq!(actual_value, expected_value, "JSON 不匹配");
} 