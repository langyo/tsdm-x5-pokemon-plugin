<script setup lang="ts">
defineProps<{ message?: string }>()
</script>

<template>
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-white rounded-xl px-8 py-6 shadow-lg text-center">
      <div class="animate-spin w-8 h-8 border-3 border-primary border-t-transparent rounded-full mx-auto mb-3" />
      <p class="text-sm text-gray-500">{{ message || '加载中...' }}</p>
    </div>
  </div>
</template>
