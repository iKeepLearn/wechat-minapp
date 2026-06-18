pub mod cli;
pub mod utils;

use std::collections::HashSet;
use std::fs;

// API 分类和对应的 URL 路径
#[derive(Debug, Clone)]
pub struct ApiDoc {
    category: String,    // 一级分类，如 "基础能力"
    subcategory: String, // 二级分类，如 "base"
    name: String,        // 接口名称，如 "获取接口调用凭据"
    url: String,         // 完整 URL
}

// 进度记录
#[derive(Debug, Clone)]
pub struct ProgressRecord {
    fetched_urls: HashSet<String>, // 已获取的 URL
    total: usize,                  // 总数量
}

impl ProgressRecord {
    pub fn new() -> Self {
        Self {
            fetched_urls: HashSet::new(),
            total: 0,
        }
    }

    // 从文件加载进度
    pub fn load(progress_file: &str) -> Self {
        if let Ok(content) = fs::read_to_string(progress_file) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(urls) = json.get("fetched_urls").and_then(|v| v.as_array()) {
                    let mut fetched = HashSet::new();
                    for url in urls {
                        if let Some(s) = url.as_str() {
                            fetched.insert(s.to_string());
                        }
                    }
                    let total = json.get("total").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                    return Self {
                        fetched_urls: fetched,
                        total,
                    };
                }
            }
        }
        Self::new()
    }

    // 保存进度
    pub fn save(&self, progress_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let urls: Vec<String> = self.fetched_urls.iter().cloned().collect();
        let json = serde_json::json!({
            "fetched_urls": urls,
            "total": self.total,
            "fetched_count": self.fetched_urls.len(),
            "last_updated": chrono::Local::now().to_rfc3339(),
        });
        fs::write(progress_file, serde_json::to_string_pretty(&json)?)?;
        Ok(())
    }

    // 检查是否已获取
    pub fn is_fetched(&self, url: &str) -> bool {
        self.fetched_urls.contains(url)
    }

    // 标记为已获取
    pub fn mark_fetched(&mut self, url: &str) {
        self.fetched_urls.insert(url.to_string());
    }
}
