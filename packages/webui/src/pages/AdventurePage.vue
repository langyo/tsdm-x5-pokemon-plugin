<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import AppLayout from '@/components/layout/AppLayout.vue'
import Card from '@/components/common/Card.vue'
import Modal from '@/components/common/Modal.vue'
import TypeBadge from '@/components/common/TypeBadge.vue'
import BattlePage from '@/components/battle/BattlePage.vue'
import { usePokemonStore, useBattleStore, useUserStore } from '@/stores'
import { api } from '@/api/client'
import { spriteUrl } from '@/utils/pokemon'

const pokemonStore = usePokemonStore()
const battleStore = useBattleStore()
const userStore = useUserStore()

const loading = ref(false)
const error = ref('')
const maps = ref<any[]>([])
const selectedMap = ref<any>(null)
const modalOpen = ref(false)
const modalTab = ref<'info' | 'wild'>('info')
const startingMapId = ref<number | null>(null)

const regionGroups = computed(() => {
  const groups: Record<string, any[]> = {}
  for (const m of maps.value) {
    const region = m.region ?? '其他区域'
    if (!groups[region]) groups[region] = []
    groups[region].push(m)
  }
  return groups
})

function difficultyLabel(diff: number): string {
  const labels: Record<number, string> = { 1: '简单', 2: '普通', 3: '困难', 4: '噩梦' }
  return labels[diff] ?? '未知'
}

function difficultyColor(diff: number): string {
  const colors: Record<number, string> = { 1: 'bg-green-500', 2: 'bg-yellow-500', 3: 'bg-orange-500', 4: 'bg-red-500' }
  return colors[diff] ?? 'bg-gray-500'
}

function openModal(map: any) {
  selectedMap.value = map
  modalTab.value = 'info'
  modalOpen.value = true
}

async function fetchMaps() {
  loading.value = true
  error.value = ''
  try {
    const data = await api.get('battle', { action: 'maps' })
    maps.value = data.maps ?? data.data ?? []
  } catch (e: any) {
    error.value = e.message || '加载地图失败'
  } finally {
    loading.value = false
  }
}

async function recoverBattle() {
  try {
    const data = await api.get('battle', { action: 'recover' })
    if (data.scene) {
      battleStore.setScene(data.scene)
    }
  } catch {
    // No active battle to recover
  }
}

async function startAdventure(mapId: number) {
  startingMapId.value = mapId
  try {
    const data = await api.post('battle', { action: 'start', map_id: mapId })
    battleStore.setScene(data.scene ?? data)
    modalOpen.value = false
  } catch (e: any) {
    error.value = e.message || '开始冒险失败'
  } finally {
    startingMapId.value = null
  }
}

async function startBossChallenge(mapId: number) {
  startingMapId.value = mapId
  try {
    const data = await api.post('battle', { action: 'start', map_id: mapId, boss: true })
    battleStore.setScene(data.scene ?? data)
    modalOpen.value = false
  } catch (e: any) {
    error.value = e.message || '挑战BOSS失败'
  } finally {
    startingMapId.value = null
  }
}

onMounted(async () => {
  await Promise.all([fetchMaps(), recoverBattle(), pokemonStore.fetchList()])
})
</script>

<template>
  <AppLayout>
    <!-- Battle View -->
    <BattlePage v-if="battleStore.scene" />

    <!-- Map View -->
    <div v-else>
      <div v-if="error" class="card text-center py-8 text-red-500 mb-4">{{ error }}</div>

      <div v-if="loading" class="text-center py-12 text-gray-400">加载地图中...</div>

      <div v-else-if="!maps.length" class="card text-center py-12 text-gray-400">暂无可用地图</div>

      <div v-else>
        <div v-for="(regionMaps, region) in regionGroups" :key="region" class="mb-6">
          <h3 class="text-lg font-bold mb-3">{{ region }}</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
            <Card
              v-for="map in regionMaps"
              :key="map.id"
              class="cursor-pointer hover:shadow-md transition-shadow"
              @click="openModal(map)"
            >
              <div class="flex items-center justify-between mb-2">
                <h4 class="font-bold">{{ map.name }}</h4>
                <span
                  class="inline-block rounded-full px-2 py-0.5 text-xs font-bold text-white"
                  :class="difficultyColor(map.difficulty ?? 1)"
                >
                  {{ difficultyLabel(map.difficulty ?? 1) }}
                </span>
              </div>
              <div class="text-xs text-gray-400">
                <span>Lv.{{ map.min_level ?? 1 }} - Lv.{{ map.max_level ?? 99 }}</span>
                <span v-if="map.wild_count !== undefined" class="ml-3">
                  {{ map.wild_count }} 种野生宝可梦
                </span>
              </div>
              <div v-if="map.description" class="text-xs text-gray-500 mt-2 line-clamp-2">{{ map.description }}</div>
            </Card>
          </div>
        </div>
      </div>
    </div>

    <!-- Map Detail Modal -->
    <Modal :open="modalOpen" :title="selectedMap?.name ?? '地图详情'" wide @close="modalOpen = false">
      <div v-if="selectedMap" class="min-h-[200px]">
        <div class="flex gap-2 mb-4">
          <button
            class="px-3 py-1 rounded text-sm transition-colors"
            :class="modalTab === 'info' ? 'bg-primary text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
            @click="modalTab = 'info'"
          >
            详情
          </button>
          <button
            class="px-3 py-1 rounded text-sm transition-colors"
            :class="modalTab === 'wild' ? 'bg-primary text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
            @click="modalTab = 'wild'"
          >
            野生宝可梦
          </button>
        </div>

        <div v-if="modalTab === 'info'">
          <div class="space-y-2 text-sm mb-4">
            <div class="flex gap-2">
              <span class="text-gray-400 w-20 shrink-0">等级范围</span>
              <span>Lv.{{ selectedMap.min_level ?? 1 }} - Lv.{{ selectedMap.max_level ?? 99 }}</span>
            </div>
            <div class="flex gap-2">
              <span class="text-gray-400 w-20 shrink-0">难度</span>
              <span
                class="inline-block rounded-full px-2 py-0.5 text-xs font-bold text-white"
                :class="difficultyColor(selectedMap.difficulty ?? 1)"
              >
                {{ difficultyLabel(selectedMap.difficulty ?? 1) }}
              </span>
            </div>
            <div v-if="selectedMap.description" class="flex gap-2">
              <span class="text-gray-400 w-20 shrink-0">描述</span>
              <span>{{ selectedMap.description }}</span>
            </div>
          </div>

          <div class="flex gap-3">
            <button
              class="flex-1 py-2 rounded-lg text-sm font-bold text-white transition-colors"
              :class="startingMapId === selectedMap.id ? 'bg-gray-400 cursor-not-allowed' : 'bg-primary hover:bg-primary/80'"
              :disabled="startingMapId === selectedMap.id"
              @click="startAdventure(selectedMap.id)"
            >
              {{ startingMapId === selectedMap.id ? '进入中...' : '开始冒险' }}
            </button>
            <button
              v-if="selectedMap.has_boss"
              class="flex-1 py-2 rounded-lg text-sm font-bold text-white transition-colors"
              :class="startingMapId === selectedMap.id ? 'bg-gray-400 cursor-not-allowed' : 'bg-red-500 hover:bg-red-600'"
              :disabled="startingMapId === selectedMap.id"
              @click="startBossChallenge(selectedMap.id)"
            >
              {{ startingMapId === selectedMap.id ? '进入中...' : '挑战BOSS' }}
            </button>
          </div>
        </div>

        <div v-else>
          <div v-if="!selectedMap.wild_pokemons?.length" class="text-center py-8 text-gray-400">
            暂无野生宝可梦数据
          </div>
          <div v-else class="grid grid-cols-4 sm:grid-cols-6 gap-2">
            <div
              v-for="wp in selectedMap.wild_pokemons"
              :key="wp.pmno ?? wp.id"
              class="bg-gray-50 rounded-lg p-2 text-center"
            >
              <img
                v-if="wp.pmno"
                :src="spriteUrl(wp.pmno)"
                :alt="wp.name"
                class="w-10 h-10 object-contain mx-auto"
              />
              <div class="text-xs font-bold mt-1 truncate">{{ wp.name }}</div>
              <div v-if="wp.type1" class="flex justify-center gap-1 mt-0.5">
                <TypeBadge :type="wp.type1" />
                <TypeBadge v-if="wp.type2" :type="wp.type2" />
              </div>
              <div class="text-xs text-gray-400 mt-0.5">
                Lv.{{ wp.min_level ?? 1 }}-{{ wp.max_level ?? 99 }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </Modal>
  </AppLayout>
</template>
