// Rust 数据结构对应的 TypeScript 类型

export interface KeywordMatch {
  keyword: string;
  occurrences: number;
  positions: number[]; // 字符索引
}

export interface FileMatchResult {
  folder: string; // "A" 或 "B"
  file_name: string;
  matches: KeywordMatch[];
}

export interface ComparisonOutput {
  folder_a_name: string;
  folder_b_name: string;
  results: FileMatchResult[];
}

export interface CompareError {
  message: string;
}
