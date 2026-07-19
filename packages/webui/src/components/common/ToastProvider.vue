<script setup lang="ts">
import { reactive, inject, provide } from 'vue'

interface Toast {
  id: number
  message: string
  type: 'info' | 'success' | 'error'
}

let nextId = 0
const toasts = reactive<Toast[]>([])

function addToast(message: string, type: 'info' | 'success' | 'error' = 'info') {
  const id = nextId++
  toasts.push({ id, message, type })
  setTimeout(() => {
    const idx = toasts.findIndex((t) => t.id === id)
    if (idx !== -1) toasts.splice(idx, 1)
  }, 3000)
}

const toast = { info: (msg: string) => addToast(msg, 'info'), success: (msg: string) => addToast(msg, 'success'), error: (msg: string) => addToast(msg, 'error') }

const key = Symbol('toast')
provide(key, toast)

export function useToast() {
  return inject<typeof toast>(key)!
}
</script>

<template>
  <div class="fixed top-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
    <div
      v-for="t in toasts"
      :key="t.id"
      class="px-4 py-2 rounded-lg text-sm text-white shadow-lg pointer-events-auto transition-all"
      :class="{
        'bg-blue-500': t.type === 'info',
        'bg-green-500': t.type === 'success',
        'bg-red-500': t.type === 'error',
      }"
    >
      {{ t.message }}
    </div>
  </div>
</template>
