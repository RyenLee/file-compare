use serde::Serialize;

/// 关键词匹配结果
#[derive(Debug, Clone, Serialize)]
pub struct KeywordMatch {
    pub keyword: String,
    pub occurrences: usize,
    pub positions: Vec<usize>, // 字符索引
}

/// 单个文件的匹配结果
#[derive(Debug, Clone, Serialize)]
pub struct FileMatchResult {
    pub folder: String, // "A" 或 "B"
    pub file_name: String,
    pub matches: Vec<KeywordMatch>,
}

/// 比对输出结果
#[derive(Debug, Serialize)]
pub struct ComparisonOutput {
    pub folder_a_name: String,
    pub folder_b_name: String,
    pub results: Vec<FileMatchResult>,
}
