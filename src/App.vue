<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Loader2 } from "lucide-vue-next";
import FolderSelector from "./components/FolderSelector.vue";
import KeywordInput from "./components/KeywordInput.vue";
import ResultTable from "./components/ResultTable.vue";
import ExportButton from "./components/ExportButton.vue";
import type { ComparisonOutput } from "./types";

const folderA = ref("");
const folderB = ref("");
const keywords = ref("");
const results = ref<ComparisonOutput | null>(null);
const loading = ref(false);
const errorMessage = ref("");
const successMessage = ref("");

const canCompare = computed(() => {
  return folderA.value && folderB.value && keywords.value.trim().length > 0;
});

const selectFolder = async (folder: "A" | "B") => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: folder === "A" ? "选择文件夹 A" : "选择文件夹 B",
    });

    if (selected && typeof selected === "string") {
      if (folder === "A") {
        folderA.value = selected;
      } else {
        folderB.value = selected;
      }
    }
  } catch (err) {
    console.error("选择文件夹失败:", err);
    errorMessage.value = "选择文件夹失败";
    clearMessages();
  }
};

const compareFolders = async () => {
  if (!canCompare.value) {
    errorMessage.value = "请选择两个文件夹并输入关键词";
    clearMessages();
    return;
  }

  loading.value = true;
  errorMessage.value = "";
  results.value = null;

  try {
    // 解析关键词：支持空格和逗号分隔
    const keywordList = keywords.value
      .split(/[,\s]+/)
      .map((k) => k.trim())
      .filter((k) => k.length > 0);

    if (keywordList.length === 0) {
      errorMessage.value = "请输入至少一个关键词";
      loading.value = false;
      return;
    }

    const response = await invoke<ComparisonOutput>("compare_folders", {
      folderA: folderA.value,
      folderB: folderB.value,
      keywords: keywordList,
    });

    results.value = response;
  } catch (err) {
    console.error("比对失败:", err);
    errorMessage.value = String(err);
  } finally {
    loading.value = false;
  }
};

const clearMessages = () => {
  setTimeout(() => {
    errorMessage.value = "";
    successMessage.value = "";
  }, 5000);
};

const handleExportSuccess = (path: string) => {
  successMessage.value = `导出成功: ${path}`;
  clearMessages();
};

const handleExportError = (error: string) => {
  errorMessage.value = `导出失败: ${error}`;
  clearMessages();
};
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-100">
    <!-- Header -->
    <header class="bg-white/80 backdrop-blur-md border-b border-white/50 sticky top-0 z-50">
      <div class="max-w-6xl mx-auto px-6 py-5">
        <div class="flex items-center gap-4">
          <div
            class="w-12 h-12 rounded-2xl bg-gradient-to-br from-blue-500 via-indigo-500 to-purple-600 flex items-center justify-center shadow-lg shadow-indigo-500/30"
          >
            <svg class="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
              />
            </svg>
          </div>
          <div>
            <h1 class="text-2xl font-bold bg-gradient-to-r from-slate-800 to-slate-600 bg-clip-text text-transparent">
              文件名字符比对工具
            </h1>
            <p class="text-sm text-slate-500">快速比对两个文件夹中的文件名，查找关键词匹配</p>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="max-w-6xl mx-auto px-6 py-8">
      <!-- Folder Selection -->
      <section class="mb-8 animate-slide-in">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <FolderSelector label="A" :folder="folderA" :disabled="loading" @select="selectFolder('A')" />
          <FolderSelector label="B" :folder="folderB" :disabled="loading" @select="selectFolder('B')" />
        </div>
      </section>

      <!-- Keyword Input -->
      <section class="mb-8 animate-slide-in" style="animation-delay: 100ms">
        <KeywordInput v-model="keywords" :disabled="loading" />
      </section>

      <!-- Action Buttons -->
      <section class="flex items-center gap-4 mb-8 animate-slide-in" style="animation-delay: 200ms">
        <button
          @click="compareFolders"
          :disabled="!canCompare || loading"
          class="btn-primary flex items-center gap-2"
        >
          <Loader2 v-if="loading" class="w-5 h-5 animate-spin" />
          <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <span>{{ loading ? "比对中..." : "开始比对" }}</span>
        </button>

        <ExportButton
          :results="results"
          :keywords="keywords"
          :disabled="loading"
          @export-success="handleExportSuccess"
          @export-error="handleExportError"
        />
      </section>

      <!-- Messages -->
      <section v-if="errorMessage" class="mb-6 animate-slide-in">
        <div class="bg-red-50 border border-red-200 text-red-700 px-5 py-4 rounded-xl flex items-center gap-3">
          <svg class="w-5 h-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <span>{{ errorMessage }}</span>
        </div>
      </section>

      <section v-if="successMessage" class="mb-6 animate-slide-in">
        <div class="bg-green-50 border border-green-200 text-green-700 px-5 py-4 rounded-xl flex items-center gap-3">
          <svg class="w-5 h-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <span>{{ successMessage }}</span>
        </div>
      </section>

      <!-- Results -->
      <section class="animate-slide-in" style="animation-delay: 300ms">
        <ResultTable :results="results" :loading="loading" />
      </section>
    </main>

    <!-- Footer -->
    <footer class="text-center py-6 text-sm text-slate-400">
      基于 Tauri 2 + Vue 3 + Rust 构建
    </footer>
  </div>
</template>
