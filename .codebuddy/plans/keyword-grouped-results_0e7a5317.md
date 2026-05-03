---
name: keyword-grouped-results
overview: 将比对结果从当前的文件列表形式改为按关键词分组折叠展示
todos:
  - id: modify-result-table
    content: 修改 ResultTable.vue，实现按关键词分组折叠展示功能
    status: completed
---

## 用户需求
修改比对结果界面，从当前"关键词作为表格字段显示"改为"按关键词分组折叠展示"。

## 当前实现
- `src/types/index.ts`: 数据结构包含 `results: FileMatchResult[]`，每个元素有 `folder`, `file_name`, `matches: KeywordMatch[]`
- `src/components/ResultTable.vue`: 表格列 `来源|文件名|关键词|出现次数|匹配位置`，每行显示"文件+关键词"组合

## 期望实现
- 按关键词（keyword）作为分组维度，每个关键词独立展示
- 折叠状态显示：关键词名称、匹配文件数量、总出现次数
- 展开状态显示：该关键词下所有匹配的文件列表（来源、文件名、出现次数、匹配位置）
- 支持展开/折叠交互


## 技术方案

### 实现方式
在 `ResultTable.vue` 中重构数据展示逻辑：
1. 将 `results.results` 数组按关键词聚合，计算每个关键词的统计信息
2. 使用 Vue 的响应式展开状态管理（ref 存储展开的关键词列表）
3. 提供折叠/展开交互按钮

### 聚合计算逻辑
```typescript
// 按关键词聚合
const groupedByKeyword = computed(() => {
  const map = new Map<string, { files: FileMatchResult[], totalOccurrences: number }>();
  for (const result of results.value?.results || []) {
    for (const match of result.matches) {
      if (!map.has(match.keyword)) {
        map.set(match.keyword, { files: [], totalOccurrences: 0 });
      }
      const group = map.get(match.keyword)!;
      group.files.push(result);
      group.totalOccurrences += match.occurrences;
    }
  }
  return Array.from(map.entries()).map(([keyword, data]) => ({
    keyword,
    fileCount: data.files.length,
    totalOccurrences: data.totalOccurrences,
    files: data.files
  }));
});
```

### 展开状态管理
```typescript
const expandedKeywords = ref<Set<string>>(new Set());
const toggleKeyword = (keyword: string) => { ... };
const isExpanded = (keyword: string) => expandedKeywords.value.has(keyword);
```

