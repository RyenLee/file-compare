use std::fs;
use std::path::Path;

/// 读取文件夹的一级子文件/目录名（不递归）
pub fn read_folder_entries(folder_path: &Path) -> Result<Vec<String>, String> {
    if !folder_path.exists() {
        return Err(format!("文件夹不存在: {}", folder_path.display()));
    }

    if !folder_path.is_dir() {
        return Err(format!("路径不是文件夹: {}", folder_path.display()));
    }

    let entries = fs::read_dir(folder_path).map_err(|e| format!("无法读取文件夹: {}", e))?;

    let mut names: Vec<String> = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_string());
            }
        }
    }

    Ok(names)
}

/// 在字符串中查找关键词的所有出现位置（按字符索引，非字节）
pub fn find_keyword_positions(text: &str, keyword: &str) -> Vec<usize> {
    if keyword.is_empty() {
        return vec![];
    }

    let mut positions: Vec<usize> = Vec::new();
    let text_chars: Vec<char> = text.chars().collect();
    let keyword_chars: Vec<char> = keyword.chars().collect();

    if keyword_chars.len() > text_chars.len() {
        return positions;
    }

    for i in 0..=(text_chars.len() - keyword_chars.len()) {
        let mut match_count = 0;
        for j in 0..keyword_chars.len() {
            if text_chars[i + j] == keyword_chars[j] {
                match_count += 1;
            } else {
                break;
            }
        }
        if match_count == keyword_chars.len() {
            positions.push(i);
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_positions() {
        // 测试英文
        assert_eq!(find_keyword_positions("hello world", "o"), vec![4, 7]);
        assert_eq!(find_keyword_positions("aaa", "aa"), vec![0, 1]);
        assert_eq!(find_keyword_positions("test", "xyz"), Vec::<usize>::new());

        // 测试中文（Unicode）
        assert_eq!(find_keyword_positions("你好世界", "好"), vec![1]);
        assert_eq!(find_keyword_positions("测试测试", "测试"), vec![0, 2]);
    }

    #[test]
    fn test_read_folder() {
        let entries = read_folder_entries(Path::new("."));
        assert!(entries.is_ok());
    }
}
