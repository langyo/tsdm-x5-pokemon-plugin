<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import AppLayout from '@/components/layout/AppLayout.vue'
import Card from '@/components/common/Card.vue'
import { usePokemonStore } from '@/stores'
import { api } from '@/api/client'
import { spriteUrl, hpClass } from '@/utils/pokemon'

const pokemonStore = usePokemonStore()

const loading = ref(false)
const healingId = ref<number | null>(null)
const healingAll = ref(false)
const error = ref('')

const stateLabels: Record<number, string> = { 0: '正常', 1: '中毒', 2: '麻痹', 3: '烧伤', 4: '冰冻', 5: '睡眠' }

const injuredPokemons = computed(() =>
  pokemonStore.list.filter((p: any) => p.hp < (p.maxHp ?? p.hp) || (p.state ?? 0) !== 0)
)

async function fetchPokemons() {
  loading.value = true
  error.value = ''
  try {
    await pokemonStore.fetchList()
  } catch (e: any) {
    error.value = e.message || '加载失败'
  } finally {
    loading.value = false
  }
}

async function heal(pokemon: any) {
  healingId.value = pokemon.id
  try {
    await api.post('user', { action: 'heal', pokemon_id: pokemon.id })
    await pokemonStore.fetchList()
  } catch (e: any) {
    error.value = e.message || '治疗失败'
  } finally {
    healingId.value = null
  }
}

async function healAndLeave(pokemon: any) {
  healingId.value = pokemon.id
  try {
    await api.post('user', { action: 'heal', pokemon_id: pokemon.id, leave_battle: true })
    await pokemonStore.fetchList()
  } catch (e: any) {
    error.value = e.message || '治疗失败'
  } finally {
    healingId.value = null
  }
}

async function healAll() {
  if (!injuredPokemons.value.length) return
  healingAll.value = true
  error.value = ''
  try {
    await api.post('user', { action: 'heal_all' })
    await pokemonStore.fetchList()
  } catch (e: any) {
    error.value = e.message || '治疗失败'
  } finally {
    healingAll.value = false
  }
}

onMounted(() => {
  fetchPokemons()
})
</script>

<template>
  <AppLayout>
    <div class="flex items-center justify-between mb-4">
      <h2 class="text-lg font-bold">宠物中心</h2>
      <button
        v-if="injuredPokemons.length > 0"
        class="px-4 py-2 rounded-lg text-sm font-bold text-white transition-colors"
        :class="healingAll ? 'bg-gray-400 cursor-not-allowed' : 'bg-red-500 hover:bg-red-600'"
        :disabled="healingAll"
        @click="healAll"
      >
        {{ healingAll ? '治疗中...' : '一键治疗' }}
      </button>
    </div>

    <div v-if="error" class="card text-center py-8 text-red-500 mb-4">{{ error }}</div>

    <div v-if="loading" class="text-center py-12 text-gray-400">加载中...</div>

    <div v-else-if="!injuredPokemons.length" class="card text-center py-12">
      <div class="text-4xl mb-3">&#10004;</div>
      <p class="text-gray-400">所有宠物都很健康！</p>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-3">
      <Card v-for="pokemon in injuredPokemons" :key="pokemon.id" class="flex items-center gap-4">
        <img
          :src="spriteUrl(pokemon.pmno)"
          :alt="pokemon.name"
          class="w-14 h-14 object-contain shrink-0"
        />
        <div class="flex-1 min-w-0">
          <div class="flex items-center gap-2">
            <span class="font-bold text-sm truncate">{{ pokemon.name }}</span>
            <span class="text-xs text-gray-500">Lv.{{ pokemon.level }}</span>
            <span
              v-if="(pokemon.state ?? 0) !== 0"
              class="inline-block rounded-full px-2 py-0.5 text-xs font-bold text-white bg-purple-500"
            >
              {{ stateLabels[pokemon.state] ?? '异常' }}
            </span>
          </div>
          <div class="w-full h-2 bg-gray-200 rounded-full mt-1.5 mb-1 overflow-hidden">
            <div
              class="h-full rounded-full transition-all"
              :class="hpClass(pokemon.hp, pokemon.maxHp ?? pokemon.hp)"
              :style="{ width: Math.max(0, Math.min(100, (pokemon.maxHp ?? pokemon.hp) > 0 ? (pokemon.hp / (pokemon.maxHp ?? pokemon.hp)) * 100 : 0)) + '%' }"
            />
          </div>
          <div class="text-xs text-gray-400">
            HP: {{ pokemon.hp }} / {{ pokemon.maxHp ?? pokemon.hp }}
            <span v-if="pokemon.site === 1" class="text-red-400 ml-2">战斗中</span>
          </div>
        </div>
        <button
          v-if="pokemon.site === 1"
          class="shrink-0 px-3 py-1.5 rounded-lg text-sm font-bold text-white transition-colors"
          :class="healingId === pokemon.id ? 'bg-gray-400 cursor-not-allowed' : 'bg-orange-500 hover:bg-orange-600'"
          :disabled="healingId === pokemon.id"
          @click="healAndLeave(pokemon)"
        >
          {{ healingId === pokemon.id ? '...' : '脱战并治疗' }}
        </button>
        <button
          v-else
          class="shrink-0 px-3 py-1.5 rounded-lg text-sm font-bold text-white transition-colors"
          :class="healingId === pokemon.id ? 'bg-gray-400 cursor-not-allowed' : 'bg-green-500 hover:bg-green-600'"
          :disabled="healingId === pokemon.id"
          @click="heal(pokemon)"
        >
          {{ healingId === pokemon.id ? '...' : '治疗' }}
        </button>
      </Card>
    </div>
  </AppLayout>
</template>
