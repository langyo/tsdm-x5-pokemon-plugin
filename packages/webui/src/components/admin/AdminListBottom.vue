<script setup lang="ts">
defineProps<{
  totalCount: number
  loadedCount: number
}>()

const emit = defineEmits<{
  reload: []
  filter: []
  create: []
}>()
</script>

<template>
  <div class="fixed bottom-0 left-48 right-0 h-14 bg-white border-t border-border flex items-center justify-between px-6 z-30">
    <span class="text-sm text-gray-500">
      共 {{ totalCount }} 条 / 已加载 {{ loadedCount }} 条
    </span>
    <div class="flex gap-3">
      <button class="btn border border-border text-sm text-gray-600 hover:bg-gray-100" @click="emit('reload')">
        刷新
      </button>
      <button class="btn border border-border text-sm text-gray-600 hover:bg-gray-100" @click="emit('filter')">
        筛选
      </button>
      <button class="btn-primary text-sm" @click="emit('create')">
        新增
      </button>
    </div>
  </div>
</template>
