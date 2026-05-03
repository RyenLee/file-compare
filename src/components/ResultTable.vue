<script setup lang="ts">
import { ref, computed } from "vue";
import { Table2, ChevronDown, ChevronRight } from "lucide-vue-next";
import type { ComparisonOutput } from "../types";

const props = defineProps<{
  results: ComparisonOutput | null;
  loading?: boolean;
}>();

// 按关键词分组的类型
interface KeywordGroup {
  keyword: string;
  fileCount: number;
  totalOccurrences: number;
  files: Array<{
    folder: string;
    fileName: string;
    occurrences: number;
    positions: number[];
  }>;
}

// 按关键词聚合数据
const groupedByKeyword = computed((): KeywordGroup[] => {
  if (!props.results) return [];
  
  const map = new Map<string, KeywordGroup>();
  
  for (const result of props.results.results) {
    for (const match of result.matches) {
      if (!map.has(match.keyword)) {
        map.set(match.keyword, {
          keyword: match.keyword,
          fileCount: 0,
          totalOccurrences: 0,
          files: []
        });
      }
      const group = map.get(match.keyword)!;
      group.fileCount++;
      group.totalOccurrences += match.occurrences;
      group.files.push({
        folder: result.folder,
        fileName: result.file_name,
        occurrences: match.occurrences,
        positions: match.positions
      });
    }
  }
  
  // 转换为数组并按关键词排序
  return Array.from(map.values()).sort((a, b) => a.keyword.localeCompare(b.keyword));
});

// 展开状态管理
const expandedKeywords = ref<Set<string>>(new Set());

const toggleKeyword = (keyword: string) => {
  if (expandedKeywords.value.has(keyword)) {
    expandedKeywords.value.delete(keyword);
  } else {
    expandedKeywords.value.add(keyword);
  }
  // 触发响应式更新
  expandedKeywords.value = new Set(expandedKeywords.value);
};

const isExpanded = (keyword: string) => expandedKeywords.value.has(keyword);

// 格式化位置显示
const formatPositions = (positions: number[]): string => {
  if (positions.length === 0) return "-";
  return positions.map((p) => p.toString()).join(", ");
};
</script>

<template>
  <div class="card p-6">
    <div class="flex items-center gap-3 mb-4">
      <div
        class="w-10 h-10 rounded-xl bg-gradient-to-br from-purple-500 to-pink-600 flex items-center justify-center shadow-lg"
      >
        <Table2 class="w-5 h-5 text-white" />
      </div>
      <div>
        <h3 class="font-semibold text-slate-800">比对结果</h3>
        <p class="text-sm text-slate-500">
          {{ results ? `共 ${groupedByKeyword.length} 个关键词，` + results.results.filter(r => r.matches.length > 0).length + ' 个文件匹配' : "暂无数据" }}
        </p>
      </div>
    </div>

    <div v-if="loading" class="flex items-center justify-center py-12">
      <div class="flex flex-col items-center gap-3">
        <div class="w-10 h-10 border-4 border-blue-200 border-t-blue-500 rounded-full animate-spin"></div>
        <span class="text-slate-500">正在扫描文件夹...</span>
      </div>
    </div>

    <div v-else-if="!results || groupedByKeyword.length === 0" class="flex items-center justify-center py-12">
      <div class="flex flex-col items-center gap-3 text-slate-400">
        <Table2 class="w-12 h-12" />
        <span>暂无比对结果</span>
      </div>
    </div>

    <div v-else class="space-y-3">
      <div
        v-for="group in groupedByKeyword"
        :key="group.keyword"
        class="border border-slate-200 rounded-xl overflow-hidden"
      >
        <!-- 关键词折叠头 -->
        <div
          @click="toggleKeyword(group.keyword)"
          class="flex items-center justify-between px-4 py-3 bg-slate-50 hover:bg-slate-100 cursor-pointer transition-colors"
        >
          <div class="flex items-center gap-3">
            <component
              :is="isExpanded(group.keyword) ? ChevronDown : ChevronRight"
              class="w-5 h-5 text-slate-500 transition-transform"
            />
            <span class="inline-flex items-center px-3 py-1 bg-emerald-100 text-emerald-700 rounded-full text-sm font-medium">
              {{ group.keyword }}
            </span>
          </div>
          <div class="flex items-center gap-4 text-sm text-slate-600">
            <span class="flex items-center gap-1">
              <span class="font-semibold">{{ group.fileCount }}</span>
              <span>个文件</span>
            </span>
            <span class="flex items-center gap-1">
              <span class="font-semibold">{{ group.totalOccurrences }}</span>
              <span>次匹配</span>
            </span>
          </div>
        </div>

        <!-- 展开的文件列表 -->
        <div v-if="isExpanded(group.keyword)" class="divide-y divide-slate-100">
          <div
            v-for="(file, index) in group.files"
            :key="index"
            class="flex items-center px-4 py-3 hover:bg-blue-50/50 transition-colors"
          >
            <div class="w-16">
              <span
                class="inline-flex items-center px-2.5 py-1 rounded-full text-xs font-bold"
                :class="file.folder === 'A' ? 'bg-blue-100 text-blue-700' : 'bg-indigo-100 text-indigo-700'"
              >
                {{ file.folder }}
              </span>
            </div>
            <div class="flex-1 min-w-0">
              <span class="font-mono text-sm text-slate-700 truncate block" :title="file.fileName">
                {{ file.fileName }}
              </span>
            </div>
            <div class="w-20 text-center">
              <span
                class="inline-flex items-center justify-center px-2 py-1 rounded-full text-xs font-bold"
                :class="file.occurrences > 0 ? 'bg-amber-100 text-amber-700' : 'bg-slate-100 text-slate-500'"
              >
                {{ file.occurrences }}次
              </span>
            </div>
            <div class="flex-1 min-w-0 max-w-[200px]">
              <span class="font-mono text-xs text-slate-600 truncate block" :title="formatPositions(file.positions)">
                {{ formatPositions(file.positions) }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
