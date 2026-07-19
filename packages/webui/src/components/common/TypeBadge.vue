<script setup lang="ts">
import { getTypeColor } from '@/utils/pokemon'

defineProps<{ type: string }>()
</script>

<template>
  <span
    class="inline-block rounded-full px-2 py-0.5 text-xs font-bold text-white"
    :style="{ backgroundColor: getTypeColor(type) }"
  >
    {{ type }}
  </span>
</template>
