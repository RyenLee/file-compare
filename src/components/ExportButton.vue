<script setup lang="ts">
import { Download } from "lucide-vue-next";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";
import type { ComparisonOutput } from "../types";
import { exportToCsv } from "../utils/csv";

const props = defineProps<{
  results: ComparisonOutput | null;
  keywords: string;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "export-start"): void;
  (e: "export-success", path: string): void;
  (e: "export-error", error: string): void;
}>();

const handleExport = async () => {
  if (!props.results || props.results.results.length === 0) {
    emit("export-error", "没有可导出的数据");
    return;
  }

  emit("export-start");

  try {
    const filePath = await save({
      defaultPath: "comparison-results.csv",
      filters: [
        {
          name: "CSV 文件",
          extensions: ["csv"],
        },
      ],
    });

    if (filePath) {
      const keywordList = props.keywords
        .split(/[,\s]+/)
        .map((k) => k.trim())
        .filter((k) => k.length > 0);
      const csvContent = exportToCsv(props.results, keywordList);
      await writeTextFile(filePath, csvContent);
      emit("export-success", filePath);
    }
  } catch (error) {
    console.error("导出失败:", error);
    emit("export-error", String(error));
  }
};
</script>

<template>
  <button
    @click="handleExport"
    :disabled="disabled || !results || results.results.length === 0"
    class="btn-secondary flex items-center gap-2"
  >
    <Download class="w-4 h-4" />
    <span>导出 CSV</span>
  </button>
</template>
