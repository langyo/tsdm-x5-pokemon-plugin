<script setup lang="ts">
defineProps<{ title?: string }>()
</script>

<template>
  <div class="rounded-xl shadow-sm border border-border bg-white p-4">
    <div v-if="title" class="text-base font-bold mb-3">{{ title }}</div>
    <slot />
  </div>
</template>
