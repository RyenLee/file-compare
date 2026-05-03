<script setup lang="ts">
import { Search } from "lucide-vue-next";

const props = defineProps<{
  modelValue: string;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  emit("update:modelValue", target.value);
};
</script>

<template>
  <div class="card p-6">
    <div class="flex items-center gap-3 mb-4">
      <div
        class="w-10 h-10 rounded-xl bg-gradient-to-br from-emerald-500 to-teal-600 flex items-center justify-center shadow-lg"
      >
        <Search class="w-5 h-5 text-white" />
      </div>
      <div>
        <h3 class="font-semibold text-slate-800">关键词设置</h3>
        <p class="text-sm text-slate-500">支持空格或逗号分隔多个关键词</p>
      </div>
    </div>

    <input
      type="text"
      :value="modelValue"
      @input="handleInput"
      :disabled="disabled"
      placeholder="例如：文档 测试 备份 或 文档,测试,备份"
      class="input-field"
    />

    <div class="mt-3 flex flex-wrap gap-2">
      <span
        v-for="(keyword, index) in modelValue
          .split(/[,\s]+/)
          .filter((k) => k.trim())"
        :key="index"
        class="px-3 py-1 bg-emerald-100 text-emerald-700 text-sm rounded-full font-medium"
      >
        {{ keyword }}
      </span>
    </div>
  </div>
</template>
