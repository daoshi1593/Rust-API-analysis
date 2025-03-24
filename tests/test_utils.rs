use std::fs;
use std::path::Path;
use std::io::Write;

pub struct TestDir {
    pub path: String,
}

impl TestDir {
    pub fn new() -> Self {
        let temp_dir = std::env::temp_dir();
        let test_dir = temp_dir.join("code_parser_test");
        fs::create_dir_all(&test_dir).unwrap();
        
        Self {
            path: test_dir.to_str().unwrap().to_string(),
        }
    }

    pub fn create_file(&self, name: &str, content: &str) {
        let file_path = Path::new(&self.path).join(name);
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    pub fn create_dir(&self, name: &str) -> String {
        let dir_path = Path::new(&self.path).join(name);
        fs::create_dir_all(&dir_path).unwrap();
        dir_path.to_str().unwrap().to_string()
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).unwrap();
    }
}

pub fn assert_json_eq(actual: &str, expected: &str) {
    let actual_json: serde_json::Value = serde_json::from_str(actual).unwrap();
    let expected_json: serde_json::Value = serde_json::from_str(expected).unwrap();
    assert_eq!(actual_json, expected_json);
} 