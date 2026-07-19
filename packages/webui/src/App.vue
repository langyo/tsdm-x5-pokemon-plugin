<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useAppStore } from '@/stores'
import LoadingOverlay from '@/components/common/LoadingOverlay.vue'

const app = useAppStore()
</script>

<template>
  <LoadingOverlay v-if="app.loading" :message="app.loadingMessage" />
  <RouterView />
</template>
