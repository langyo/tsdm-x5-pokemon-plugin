<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { api } from '@/api/client'
import { usePokemonStore } from '@/stores'

const router = useRouter()
const pokemonStore = usePokemonStore()

const loading = ref(false)
const error = ref<string | null>(null)
const success = ref(false)

async function handleInitialize() {
  loading.value = true
  error.value = null
  try {
    await api.post('user', { action: 'initialize' })
    success.value = true
    setTimeout(() => {
      location.reload()
    }, 1500)
  } catch (e: any) {
    error.value = e.message || '初始化失败'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100">
    <div class="text-center bg-white rounded-2xl shadow-lg border border-border p-8 max-w-md w-full mx-4">
      <div class="text-6xl mb-4">
        <img src="https://img.tsdm39.com/Pokemon/spm/25.gif" alt="Pikachu" class="w-24 h-24 mx-auto object-contain" />
      </div>
      <h1 class="text-2xl font-bold mb-2 text-gray-800">欢迎来到 TSDM Pokemon</h1>
      <p class="text-gray-500 mb-6 text-sm leading-relaxed">
        初始化你的训练师账号，获取你的第一只宝可梦，<br />开启属于你的冒险之旅！
      </p>

      <div v-if="error" class="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg text-sm text-red-600">
        {{ error }}
      </div>

      <div v-if="success" class="mb-4 p-3 bg-green-50 border border-green-200 rounded-lg text-sm text-green-600">
        初始化成功！即将进入游戏...
      </div>

      <button
        class="bg-primary text-white text-lg px-8 py-3 rounded-xl font-bold hover:opacity-90 transition-opacity disabled:opacity-50"
        :disabled="loading || success"
        @click="handleInitialize"
      >
        <span v-if="loading" class="inline-flex items-center gap-2">
          <span class="animate-spin w-4 h-4 border-2 border-white border-t-transparent rounded-full" />
          初始化中...
        </span>
        <span v-else>开始冒险</span>
      </button>
    </div>
  </div>
</template>
