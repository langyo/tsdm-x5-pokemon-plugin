<script setup lang="ts">
import { watch } from 'vue'

const props = defineProps<{
  open: boolean
  x: number
  y: number
}>()

const emit = defineEmits<{ close: [] }>()

watch(
  () => props.open,
  (val) => {
    if (val) {
      document.addEventListener('click', onClickOutside)
      document.addEventListener('keydown', onKey)
    } else {
      document.removeEventListener('click', onClickOutside)
      document.removeEventListener('keydown', onKey)
    }
  }
)

function onClickOutside(e: MouseEvent) {
  emit('close')
}

function onKey(e: KeyboardEvent) {
  if (e.key === 'Escape') emit('close')
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="open"
      class="fixed z-50 min-w-32 bg-white rounded-lg shadow-lg border border-border py-1"
      :style="{ left: x + 'px', top: y + 'px' }"
      @click.stop
    >
      <slot />
    </div>
  </Teleport>
</template>
