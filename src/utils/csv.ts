import type { ComparisonOutput } from "../types";

/**
 * 检查文件名是否包含指定关键词（忽略大小写）
 */
function fileNameMatchesKeywords(fileName: string, keywords: string[]): boolean {
  const lowerFileName = fileName.toLowerCase();
  return keywords.some((keyword) => lowerFileName.includes(keyword.toLowerCase()));
}

/**
 * 将比对结果导出为 CSV 格式
 * 仅导出文件名中包含指定关键词的文件
 */
export function exportToCsv(data: ComparisonOutput, keywords: string[]): string {
  const BOM = "\uFEFF"; // UTF-8 BOM
  const headers = ["来源", "文件名", "文件夹A路径", "文件夹B路径", "关键词", "出现次数", "匹配位置"];
  const rows: string[][] = [];

  // CSV 转义函数
  const escapeCSV = (value: string): string => {
    if (value.includes(",") || value.includes('"') || value.includes("\n")) {
      return `"${value.replace(/"/g, '""')}"`;
    }
    return value;
  };

  for (const result of data.results) {
    // 过滤：仅保留文件名中包含关键词的文件
    if (!fileNameMatchesKeywords(result.file_name, keywords)) {
      continue;
    }

    if (result.matches.length === 0) {
      // 文件名匹配但内容无匹配
      rows.push([
        result.folder,
        escapeCSV(result.file_name),
        escapeCSV(data.folder_a_name),
        escapeCSV(data.folder_b_name),
        "",
        "0",
        "",
      ]);
    } else {
      for (const match of result.matches) {
        rows.push([
          result.folder,
          escapeCSV(result.file_name),
          escapeCSV(data.folder_a_name),
          escapeCSV(data.folder_b_name),
          escapeCSV(match.keyword),
          match.occurrences.toString(),
          match.positions.join(";"),
        ]);
      }
    }
  }

  const csvContent = [
    headers.join(","),
    ...rows.map((row) => row.join(",")),
  ].join("\n");

  return BOM + csvContent; // 添加 BOM 以支持 Excel 正确读取 UTF-8
}
