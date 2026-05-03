<script setup lang="ts">
import { FolderOpen } from "lucide-vue-next";

defineProps<{
  label: string;
  folder: string;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "select"): void;
}>();
</script>

<template>
  <div class="card p-6 hover:shadow-xl transition-shadow duration-300">
    <div class="flex items-center gap-3 mb-4">
      <div
        class="w-10 h-10 rounded-xl bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center shadow-lg"
      >
        <FolderOpen class="w-5 h-5 text-white" />
      </div>
      <div>
        <h3 class="font-semibold text-slate-800">文件夹 {{ label }}</h3>
        <p class="text-sm text-slate-500">点击选择文件夹</p>
      </div>
    </div>

    <div
      class="min-h-[80px] p-4 bg-slate-50 rounded-xl border-2 border-dashed border-slate-200 hover:border-blue-300 transition-colors cursor-pointer"
      :class="{ 'opacity-50 cursor-not-allowed': disabled }"
      @click="!disabled && emit('select')"
    >
      <template v-if="folder">
        <div class="flex items-center gap-2">
          <FolderOpen class="w-4 h-4 text-green-500 flex-shrink-0" />
          <span class="text-sm text-slate-700 truncate font-mono" :title="folder">
            {{ folder }}
          </span>
        </div>
      </template>
      <template v-else>
        <div class="flex flex-col items-center justify-center h-full py-2">
          <FolderOpen class="w-8 h-8 text-slate-300 mb-2" />
          <span class="text-sm text-slate-400">尚未选择文件夹</span>
        </div>
      </template>
    </div>
  </div>
</template>
