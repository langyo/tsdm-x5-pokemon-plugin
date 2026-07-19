<script setup lang="ts">
import { watch } from 'vue'

const props = defineProps<{
  open: boolean
  title?: string
  wide?: boolean
}>()

const emit = defineEmits<{ close: [] }>()

watch(
  () => props.open,
  (val) => {
    if (val) {
      document.addEventListener('keydown', onKey)
    } else {
      document.removeEventListener('keydown', onKey)
    }
  }
)

function onKey(e: KeyboardEvent) {
  if (e.key === 'Escape') emit('close')
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="open"
      class="fixed inset-0 z-40 flex items-center justify-center"
    >
      <div class="absolute inset-0 bg-black/50" @click="emit('close')" />
      <div
        class="relative bg-white rounded-xl shadow-lg border border-border z-10 mx-4"
        :class="wide ? 'w-full max-w-3xl' : 'w-full max-w-md'"
      >
        <div class="flex items-center justify-between px-5 py-3 border-b border-border">
          <h3 class="text-base font-bold">{{ title || '' }}</h3>
          <button
            class="text-gray-400 hover:text-gray-600 text-lg leading-none p-1"
            @click="emit('close')"
          >
            &#x2715;
          </button>
        </div>
        <div class="p-5">
          <slot />
        </div>
      </div>
    </div>
  </Teleport>
</template>
