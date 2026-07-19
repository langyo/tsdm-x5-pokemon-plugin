<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { api } from '@/api/client'
import { useUserStore } from '@/stores'
import AppLayout from '@/components/layout/AppLayout.vue'
import Card from '@/components/common/Card.vue'
import Modal from '@/components/common/Modal.vue'
import { spriteUrl } from '@/utils/pokemon'

const router = useRouter()
const userStore = useUserStore()

const tabs = [
  { key: 1, label: '回复药' },
  { key: 2, label: '精灵球' },
  { key: 3, label: '进化石' },
  { key: 4, label: '强化道具' },
  { key: 5, label: '装备道具' },
]

const activeType = ref(1)
const items = ref<any[]>([])
const loading = ref(false)
const error = ref<string | null>(null)
const page = ref(1)
const totalPages = ref(1)

const useLoadingId = ref<number | null>(null)

const useModalOpen = ref(false)
const useTargetItem = ref<any>(null)
const usablePokemons = ref<any[]>([])
const usePokemonLoading = ref(false)
const usePokemonError = ref<string | null>(null)
const confirmingPokemonId = ref<number | null>(null)

async function fetchInventory() {
  loading.value = true
  error.value = null
  try {
    const data = await api.get<any>('user', { action: 'inventory', page: page.value, type: activeType.value })
    items.value = data.items ?? data.data ?? []
    totalPages.value = data.total_pages ?? data.totalPages ?? 1
  } catch (e: any) {
    error.value = e.message || '加载失败'
    items.value = []
  } finally {
    loading.value = false
  }
}

function switchTab(type: number) {
  activeType.value = type
  page.value = 1
  fetchInventory()
}

function prevPage() {
  if (page.value > 1) {
    page.value--
    fetchInventory()
  }
}

function nextPage() {
  if (page.value < totalPages.value) {
    page.value++
    fetchInventory()
  }
}

function handleUse(item: any) {
  if (activeType.value === 5) {
    router.push({ path: '/my-pokemon', query: { tab: 'equipment' } })
    return
  }
  useTargetItem.value = item
  useModalOpen.value = true
  fetchUsablePokemons(item.id)
}

async function fetchUsablePokemons(itemId: number) {
  usePokemonLoading.value = true
  usePokemonError.value = null
  usablePokemons.value = []
  try {
    const data = await api.get<any>('user', { action: 'get_usable_pokemon', item_id: itemId })
    usablePokemons.value = data.pokemons ?? data.data ?? []
  } catch (e: any) {
    usePokemonError.value = e.message || '加载失败'
  } finally {
    usePokemonLoading.value = false
  }
}

async function confirmUse(pokemonId: number) {
  confirmingPokemonId.value = pokemonId
  try {
    await api.post('user', { action: 'use_item', item_id: useTargetItem.value.id, pokemon_id: pokemonId })
    useModalOpen.value = false
    await Promise.all([fetchInventory(), userStore.fetchProfile()])
  } catch (e: any) {
    alert(e.message || '使用失败')
  } finally {
    confirmingPokemonId.value = null
  }
}

function getItemIcon(item: any): string {
  if (item.icon && item.icon.startsWith('http')) return item.icon
  if (item.icon) return item.icon
  return ''
}

const isEquipmentTab = computed(() => activeType.value === 5)

onMounted(() => {
  fetchInventory()
})
</script>

<template>
  <AppLayout>
    <!-- Category Tabs -->
    <div class="flex gap-2 mb-4 flex-wrap">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        class="px-4 py-1.5 rounded-full text-sm transition-colors"
        :class="activeType === tab.key ? 'bg-primary text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
        @click="switchTab(tab.key)"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="text-center py-16">
      <div class="animate-spin w-8 h-8 border-3 border-primary border-t-transparent rounded-full mx-auto mb-3" />
      <p class="text-gray-500 text-sm">加载背包...</p>
    </div>

    <!-- Error -->
    <div v-else-if="error" class="text-center py-16">
      <p class="text-red-500 mb-4">{{ error }}</p>
      <button
        class="bg-primary text-white px-4 py-2 rounded-lg text-sm"
        @click="fetchInventory"
      >
        重试
      </button>
    </div>

    <!-- Empty -->
    <div v-else-if="!items.length" class="text-center py-16">
      <p class="text-gray-400 text-lg">背包空空如也</p>
      <p class="text-gray-400 text-sm mt-2">去商店购买一些道具吧</p>
    </div>

    <!-- Items Grid -->
    <div v-else class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
      <Card v-for="item in items" :key="item.id">
        <div class="flex items-center gap-3 mb-2">
          <template v-if="getItemIcon(item)">
            <img
              v-if="getItemIcon(item).startsWith('http')"
              :src="getItemIcon(item)"
              :alt="item.name"
              class="w-10 h-10 object-contain"
            />
            <div
              v-else
              class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center text-lg"
            >
              {{ getItemIcon(item) }}
            </div>
          </template>
          <div
            v-else
            class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center"
          >
            <span class="text-xs text-gray-400">道具</span>
          </div>
          <div class="flex-1 min-w-0">
            <div class="font-bold text-sm truncate">{{ item.name }}</div>
            <div class="text-xs text-gray-500">x{{ item.nums ?? item.quantity ?? 1 }}</div>
          </div>
        </div>
        <div v-if="item.description" class="text-xs text-gray-400 mb-3 line-clamp-2">{{ item.description }}</div>
        <button
          class="w-full py-1.5 rounded-lg text-sm font-bold text-white transition-colors"
          :class="useLoadingId === item.id ? 'bg-gray-400 cursor-not-allowed' : 'bg-primary hover:bg-primary/80'"
          :disabled="useLoadingId === item.id"
          @click="handleUse(item)"
        >
          {{ useLoadingId === item.id ? '...' : isEquipmentTab ? '去装备' : '使用' }}
        </button>
      </Card>
    </div>

    <!-- Pagination -->
    <div v-if="totalPages > 1 && !loading && !error && items.length" class="flex items-center justify-center gap-4 mt-6">
      <button
        class="px-3 py-1 rounded text-sm"
        :class="page <= 1 ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
        :disabled="page <= 1"
        @click="prevPage"
      >
        上一页
      </button>
      <span class="text-sm text-gray-500">{{ page }} / {{ totalPages }}</span>
      <button
        class="px-3 py-1 rounded text-sm"
        :class="page >= totalPages ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
        :disabled="page >= totalPages"
        @click="nextPage"
      >
        下一页
      </button>
    </div>

    <!-- Use Item Modal -->
    <Modal :open="useModalOpen" title="选择宝可梦" @close="useModalOpen = false">
      <div v-if="usePokemonLoading" class="text-center py-8">
        <div class="animate-spin w-6 h-6 border-2 border-primary border-t-transparent rounded-full mx-auto mb-2" />
        <p class="text-sm text-gray-400">加载中...</p>
      </div>

      <div v-else-if="usePokemonError" class="text-center py-8">
        <p class="text-red-500 text-sm">{{ usePokemonError }}</p>
      </div>

      <div v-else-if="!usablePokemons.length" class="text-center py-8">
        <p class="text-gray-400 text-sm">没有可使用的宝可梦</p>
      </div>

      <div v-else class="space-y-2 max-h-80 overflow-y-auto">
        <div
          v-for="pokemon in usablePokemons"
          :key="pokemon.id"
          class="flex items-center gap-3 p-3 rounded-lg border border-border cursor-pointer transition-colors"
          :class="confirmingPokemonId === pokemon.id ? 'bg-gray-100' : 'hover:bg-gray-50'"
          @click="confirmUse(pokemon.id)"
        >
          <img
            :src="spriteUrl(pokemon.pmno)"
            :alt="pokemon.name"
            class="w-10 h-10 object-contain shrink-0"
          />
          <div class="flex-1 min-w-0">
            <div class="text-sm font-bold truncate">{{ pokemon.name || `No.${pokemon.pmno}` }}</div>
            <div class="text-xs text-gray-400">Lv.{{ pokemon.level }}</div>
          </div>
          <span
            v-if="confirmingPokemonId === pokemon.id"
            class="text-xs text-gray-400"
          >使用中...</span>
          <span
            v-else
            class="text-xs text-primary font-medium shrink-0"
          >使用</span>
        </div>
      </div>
    </Modal>
  </AppLayout>
</template>
