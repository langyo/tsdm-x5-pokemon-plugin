<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AppLayout from '@/components/layout/AppLayout.vue'
import Card from '@/components/common/Card.vue'
import { useUserStore, usePokemonStore } from '@/stores'
import { api } from '@/api/client'
import { spriteUrl } from '@/utils/pokemon'

const userStore = useUserStore()
const pokemonStore = usePokemonStore()

const tabs = [
  { key: '1', label: '回复药', type: 1 },
  { key: '2', label: '精灵球', type: 2 },
  { key: '3', label: '进化石', type: 3 },
  { key: '4', label: '强化道具', type: 4 },
  { key: '5', label: '装备道具', type: 5 },
  { key: 'pets', label: '宠物', type: null },
]

const activeTab = ref('1')
const items = ref<any[]>([])
const loading = ref(false)
const error = ref('')
const page = ref(1)
const totalPages = ref(1)
const buyingId = ref<number | null>(null)

const currentTab = () => tabs.find((t) => t.key === activeTab.value)!

async function fetchItems() {
  loading.value = true
  error.value = ''
  try {
    const tab = currentTab()
    if (tab.key === 'pets') {
      const data = await api.get('shop', { action: 'pets', page: page.value })
      items.value = data.pets ?? data.data ?? []
      totalPages.value = data.total_pages ?? data.totalPages ?? 1
    } else {
      const data = await api.get('shop', { action: 'list', type: tab.type, page: page.value })
      items.value = data.items ?? data.data ?? []
      totalPages.value = data.total_pages ?? data.totalPages ?? 1
    }
  } catch (e: any) {
    error.value = e.message || '加载失败'
    items.value = []
  } finally {
    loading.value = false
  }
}

function switchTab(key: string) {
  activeTab.value = key
  page.value = 1
  fetchItems()
}

async function buy(item: any) {
  buyingId.value = item.id ?? item.pet_id
  try {
    const tab = currentTab()
    if (tab.key === 'pets') {
      await api.post('shop', { action: 'buy_pet', pet_id: item.id ?? item.pet_id })
    } else {
      await api.post('shop', { action: 'buy', item_id: item.id })
    }
    await Promise.all([userStore.fetchProfile(), pokemonStore.fetchList()])
  } catch (e: any) {
    alert(e.message || '购买失败')
  } finally {
    buyingId.value = null
  }
}

function prevPage() {
  if (page.value > 1) {
    page.value--
    fetchItems()
  }
}
function nextPage() {
  if (page.value < totalPages.value) {
    page.value++
    fetchItems()
  }
}

onMounted(() => {
  fetchItems()
})
</script>

<template>
  <AppLayout>
    <div class="mb-4 flex items-center gap-2">
      <span class="text-sm text-gray-500">余额:</span>
      <span class="font-bold text-primary">{{ userStore.money }}</span>
    </div>

    <div class="flex gap-2 mb-4 flex-wrap">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        class="px-4 py-1.5 rounded-full text-sm transition-colors"
        :class="activeTab === tab.key ? 'bg-primary text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
        @click="switchTab(tab.key)"
      >
        {{ tab.label }}
      </button>
    </div>

    <div v-if="loading" class="text-center py-12 text-gray-400">加载中...</div>

    <div v-else-if="error" class="card text-center py-12 text-red-500">{{ error }}</div>

    <div v-else-if="!items.length" class="card text-center py-12 text-gray-400">暂无物品</div>

    <div v-else class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
      <Card v-for="item in items" :key="item.id ?? item.pet_id ?? item.pmno" class="flex flex-col">
        <div class="flex items-center gap-3 mb-2">
          <img
            v-if="currentTab().key === 'pets'"
            :src="spriteUrl(item.pmno ?? item.image ?? 0)"
            :alt="item.name"
            class="w-12 h-12 object-contain"
          />
          <div
            v-else-if="item.icon"
            class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center text-lg"
          >
            {{ item.icon }}
          </div>
          <div v-else class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center">
            <span class="text-xs text-gray-400">道具</span>
          </div>
          <div class="flex-1 min-w-0">
            <div class="font-bold text-sm truncate">{{ item.name }}</div>
            <div class="text-primary font-bold text-sm">{{ item.price }}</div>
          </div>
        </div>
        <div v-if="item.description" class="text-xs text-gray-400 mb-3 line-clamp-2">{{ item.description }}</div>
        <button
          class="mt-auto w-full py-1.5 rounded-lg text-sm font-bold text-white transition-colors"
          :class="buyingId === (item.id ?? item.pet_id) ? 'bg-gray-400 cursor-not-allowed' : 'bg-primary hover:bg-primary/80'"
          :disabled="buyingId === (item.id ?? item.pet_id)"
          @click="buy(item)"
        >
          {{ buyingId === (item.id ?? item.pet_id) ? '购买中...' : '购买' }}
        </button>
      </Card>
    </div>

    <div v-if="totalPages > 1 && !loading && !error" class="flex items-center justify-center gap-4 mt-6">
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
  </AppLayout>
</template>
