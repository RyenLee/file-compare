use crate::file_utils::{find_keyword_positions, read_folder_entries};
use crate::models::{ComparisonOutput, FileMatchResult, KeywordMatch};
use std::path::PathBuf;
use tokio::task;

/// 比较两个文件夹中的文件名，查找关键词匹配
#[tauri::command]
pub async fn compare_folders(
    folder_a: String,
    folder_b: String,
    keywords: Vec<String>,
) -> Result<ComparisonOutput, String> {
    // 验证输入
    if folder_a.trim().is_empty() {
        return Err("文件夹 A 路径不能为空".to_string());
    }
    if folder_b.trim().is_empty() {
        return Err("文件夹 B 路径不能为空".to_string());
    }
    if keywords.is_empty() {
        return Err("请提供至少一个关键词".to_string());
    }

    let folder_a_path = PathBuf::from(&folder_a);
    let folder_b_path = PathBuf::from(&folder_b);

    // 使用 spawn_blocking 避免阻塞主线程
    let folder_a_entries = task::spawn_blocking(move || {
        read_folder_entries(&folder_a_path)
    }).await.map_err(|e| format!("任务执行失败: {}", e))??;

    let folder_b_entries = task::spawn_blocking(move || {
        read_folder_entries(&folder_b_path)
    }).await.map_err(|e| format!("任务执行失败: {}", e))??;

    // 构建结果
    let mut results: Vec<FileMatchResult> = Vec::new();

    // 处理文件夹 A 的条目
    for file_name in folder_a_entries {
        let mut matches: Vec<KeywordMatch> = Vec::new();
        for keyword in &keywords {
            let positions = find_keyword_positions(&file_name, keyword);
            if !positions.is_empty() {
                matches.push(KeywordMatch {
                    keyword: keyword.clone(),
                    occurrences: positions.len(),
                    positions,
                });
            }
        }
        results.push(FileMatchResult {
            folder: "A".to_string(),
            file_name,
            matches,
        });
    }

    // 处理文件夹 B 的条目
    for file_name in folder_b_entries {
        let mut matches: Vec<KeywordMatch> = Vec::new();
        for keyword in &keywords {
            let positions = find_keyword_positions(&file_name, keyword);
            if !positions.is_empty() {
                matches.push(KeywordMatch {
                    keyword: keyword.clone(),
                    occurrences: positions.len(),
                    positions,
                });
            }
        }
        results.push(FileMatchResult {
            folder: "B".to_string(),
            file_name,
            matches,
        });
    }

    Ok(ComparisonOutput {
        folder_a_name: folder_a,
        folder_b_name: folder_b,
        results,
    })
}
