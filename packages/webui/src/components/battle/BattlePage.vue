<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useBattleStore, usePokemonStore } from '@/stores'
import { api } from '@/api/client'
import { battleSpriteUrl, hpClass } from '@/utils/pokemon'
import TypeBadge from '@/components/common/TypeBadge.vue'
import Card from '@/components/common/Card.vue'
import BattleSkillsTab from '@/components/battle/BattleSkillsTab.vue'
import BattleItemsTab from '@/components/battle/BattleItemsTab.vue'
import BattleResultSection from '@/components/battle/BattleResultSection.vue'
import BattleSwitchModal from '@/components/battle/BattleSwitchModal.vue'

const battleStore = useBattleStore()
const pokemonStore = usePokemonStore()

const activeTab = ref<'skills' | 'items' | 'capture' | 'flee'>('skills')
const waiting = ref(false)
const error = ref('')

const switchModalOpen = ref(false)
const switchReason = ref<'switch' | 'replace'>('switch')

const scene = computed(() => battleStore.scene)
const isActive = computed(() => scene.value?.status === 'Active')
const opponent = computed(() => scene.value?.opponent ?? null)
const myPokemon = computed(() => scene.value?.my_pokemon ?? null)
const turn = computed(() => scene.value?.turn ?? scene.value?.round ?? 0)
const logs = computed(() => {
  return scene.value?.logs ?? scene.value?.events ?? []
})

const opponentHpPct = computed(() => {
  if (!opponent.value) return 0
  const hp = opponent.value.hp ?? 0
  const max = opponent.value.maxHp ?? opponent.value.max_hp ?? 1
  return Math.max(0, Math.min(100, (hp / max) * 100))
})

const myHpPct = computed(() => {
  if (!myPokemon.value) return 0
  const hp = myPokemon.value.hp ?? 0
  const max = myPokemon.value.maxHp ?? myPokemon.value.max_hp ?? 1
  return Math.max(0, Math.min(100, (hp / max) * 100))
})

const captureItems = computed(() => {
  return items.value.filter((i: any) => i.type === 2)
})

const items = ref<any[]>([])
const itemsLoading = ref(false)

async function fetchCaptureItems() {
  itemsLoading.value = true
  try {
    const data: any = await api.get('battle', { action: 'get_battle_items' })
    items.value = data.items ?? data.data ?? []
  } catch {
    // Silently fail, will show empty
  } finally {
    itemsLoading.value = false
  }
}

function onAction() {
  waiting.value = true
  error.value = ''
}

async function doAction(apiCall: () => Promise<any>) {
  waiting.value = true
  error.value = ''
  try {
    const data = await apiCall()
    battleStore.setScene(data.scene ?? data)
    if (isActive.value) {
      await pokemonStore.fetchList()
    }
  } catch (e: any) {
    error.value = e.message || '操作失败'
  } finally {
    waiting.value = false
  }
}

async function onFlee() {
  await doAction(() => api.post('battle', { action: 'flee' }))
}

async function openSwitch(reason: 'switch' | 'replace') {
  switchReason.value = reason
  switchModalOpen.value = true
}

async function onCapture(pokeballId: number) {
  await doAction(() => api.post('battle', { action: 'capture', pokeball_id: pokeballId }))
}

onMounted(() => {
  if (isActive.value) {
    fetchCaptureItems()
  }
})
</script>

<template>
  <div v-if="scene" class="flex flex-col h-full max-w-2xl mx-auto">
    <!-- Error Banner -->
    <div v-if="error" class="bg-red-50 border border-red-200 text-red-600 rounded-lg px-4 py-2 mb-3 text-sm">
      {{ error }}
      <button class="ml-2 underline cursor-pointer" @click="error = ''">关闭</button>
    </div>

    <!-- Battle is Active -->
    <template v-if="isActive">
      <!-- Opponent Section -->
      <div class="flex items-center gap-4 p-4 bg-gray-50 rounded-xl mb-3">
        <img
          v-if="opponent?.pmno"
          :src="battleSpriteUrl(opponent.pmno)"
          :alt="opponent.name"
          class="w-20 h-20 object-contain shrink-0"
        />
        <div v-else class="w-20 h-20 shrink-0 bg-gray-200 rounded-xl flex items-center justify-center text-gray-400 text-xs">
          未知
        </div>
        <div class="flex-1 min-w-0">
          <div class="flex items-center gap-2 mb-1">
            <span class="text-base font-bold truncate">{{ opponent?.name ?? '???' }}</span>
            <span class="text-xs text-gray-400 shrink-0">Lv.{{ opponent?.level ?? '?' }}</span>
            <span v-if="opponent?.sex !== undefined && opponent?.sex > 0" class="text-xs shrink-0">
              {{ opponent.sex === 1 ? '♂' : '♀' }}
            </span>
          </div>
          <div class="flex gap-1 mb-2">
            <TypeBadge v-if="opponent?.type1" :type="opponent.type1" />
            <TypeBadge v-if="opponent?.type2" :type="opponent.type2" />
          </div>
          <div class="flex items-center gap-2">
            <div class="flex-1 h-3 bg-gray-300 rounded-full overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-500"
                :class="hpClass(opponent?.hp ?? 0, opponent?.maxHp ?? opponent?.max_hp ?? 1)"
                :style="{ width: `${opponentHpPct}%` }"
              />
            </div>
            <span class="text-xs text-gray-500 shrink-0">
              {{ opponent?.hp ?? 0 }}/{{ opponent?.maxHp ?? opponent?.max_hp ?? 0 }}
            </span>
          </div>
        </div>
      </div>

      <!-- Turn / Map Info -->
      <div class="flex items-center justify-between mb-3 text-xs text-gray-400">
        <span>{{ scene.map_name ?? scene.map ?? '未知' }}</span>
        <span>回合 {{ turn }}</span>
      </div>

      <!-- My Pokemon Section -->
      <div class="flex items-center gap-4 p-4 bg-green-50 rounded-xl mb-4">
        <div class="flex-1 min-w-0">
          <div class="flex items-center gap-2 mb-1">
            <span class="text-base font-bold truncate">{{ myPokemon?.name ?? '???' }}</span>
            <span class="text-xs text-gray-400 shrink-0">Lv.{{ myPokemon?.level ?? '?' }}</span>
            <span v-if="myPokemon?.sex !== undefined && myPokemon?.sex > 0" class="text-xs shrink-0">
              {{ myPokemon.sex === 1 ? '♂' : '♀' }}
            </span>
          </div>
          <div class="flex gap-1 mb-2">
            <TypeBadge v-if="myPokemon?.type1" :type="myPokemon.type1" />
            <TypeBadge v-if="myPokemon?.type2" :type="myPokemon.type2" />
          </div>
          <div class="flex items-center gap-2">
            <div class="flex-1 h-3 bg-gray-300 rounded-full overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-500"
                :class="hpClass(myPokemon?.hp ?? 0, myPokemon?.maxHp ?? myPokemon?.max_hp ?? 1)"
                :style="{ width: `${myHpPct}%` }"
              />
            </div>
            <span class="text-xs text-gray-500 shrink-0">
              {{ myPokemon?.hp ?? 0 }}/{{ myPokemon?.maxHp ?? myPokemon?.max_hp ?? 0 }}
            </span>
          </div>
          <div v-if="myPokemon?.exp !== undefined && myPokemon?.maxExp !== undefined" class="mt-1">
            <div class="flex items-center gap-2">
              <div class="flex-1 h-1.5 bg-gray-300 rounded-full overflow-hidden">
                <div
                  class="h-full bg-blue-400 rounded-full transition-all"
                  :style="{ width: `${Math.max(0, Math.min(100, ((myPokemon.exp) / (myPokemon.maxExp || 1)) * 100))}%` }"
                />
              </div>
              <span class="text-xs text-blue-400 shrink-0">EXP</span>
            </div>
          </div>
        </div>
        <img
          v-if="myPokemon?.pmno"
          :src="battleSpriteUrl(myPokemon.pmno)"
          :alt="myPokemon.name"
          class="w-20 h-20 object-contain shrink-0 scale-x-[-1]"
        />
        <div v-else class="w-20 h-20 shrink-0 bg-gray-200 rounded-xl flex items-center justify-center text-gray-400 text-xs">
          未知
        </div>
      </div>

      <!-- Battle Actions Section -->
      <Card class="flex-1 flex flex-col min-h-0">
        <!-- Switch Pokemon Quick Button -->
        <div class="flex items-center justify-between mb-3">
          <span class="text-sm font-bold">战斗操作</span>
          <button
            class="text-xs text-blue-500 hover:text-blue-700 cursor-pointer"
            @click="openSwitch('switch')"
          >
            更换宝可梦
          </button>
        </div>

        <!-- Tab Buttons -->
        <div class="flex gap-1 mb-3 border-b border-border">
          <button
            v-for="tab in [
              { key: 'skills', label: '技能' },
              { key: 'items', label: '道具' },
              { key: 'capture', label: '捕捉' },
              { key: 'flee', label: '逃跑' },
            ]"
            :key="tab.key"
            class="flex-1 py-2 text-sm font-medium transition-colors cursor-pointer"
            :class="activeTab === tab.key
              ? 'text-primary border-b-2 border-primary -mb-0.5'
              : 'text-gray-500 hover:text-gray-700'"
            @click="activeTab = tab.key as any"
          >
            {{ tab.label }}
          </button>
        </div>

        <!-- Tab Content -->
        <div class="flex-1 overflow-y-auto">
          <!-- Skills Tab -->
          <BattleSkillsTab
            v-if="activeTab === 'skills'"
            :waiting="waiting"
            @action="onAction"
          />

          <!-- Items Tab -->
          <BattleItemsTab
            v-if="activeTab === 'items'"
            :waiting="waiting"
            @action="onAction"
          />

          <!-- Capture Tab -->
          <div v-if="activeTab === 'capture'">
            <div v-if="itemsLoading" class="text-center py-8 text-gray-400 text-sm">
              加载精灵球...
            </div>
            <div v-else-if="!captureItems.length" class="text-center py-8 text-gray-400 text-sm">
              没有可用的精灵球
            </div>
            <div v-else class="grid grid-cols-2 gap-2">
              <button
                v-for="ball in captureItems"
                :key="ball.id"
                class="flex flex-col items-center gap-1 p-3 rounded-lg border border-border bg-white hover:bg-gray-50 transition-colors cursor-pointer text-center"
                :class="waiting ? 'opacity-50 cursor-not-allowed' : ''"
                :disabled="waiting"
                @click="onCapture(ball.id)"
              >
                <span class="text-sm font-bold truncate w-full">{{ ball.name }}</span>
                <span class="text-xs text-gray-400">x{{ ball.count ?? 0 }}</span>
              </button>
            </div>
          </div>

          <!-- Flee Tab -->
          <div v-if="activeTab === 'flee'">
            <div class="text-center py-4">
              <p class="text-sm text-gray-500 mb-4">确定要逃跑吗？</p>
              <button
                class="px-6 py-2 rounded-lg text-sm font-bold text-white transition-colors cursor-pointer"
                :class="waiting ? 'bg-gray-400 cursor-not-allowed' : 'bg-red-500 hover:bg-red-600'"
                :disabled="waiting"
                @click="onFlee"
              >
                {{ waiting ? '逃跑中...' : '逃跑' }}
              </button>
            </div>
          </div>
        </div>
      </Card>

      <!-- Battle Log -->
      <Card v-if="logs.length" class="mt-3">
        <div class="text-xs font-bold mb-2 text-gray-500">战斗日志</div>
        <div class="space-y-1 max-h-32 overflow-y-auto">
          <div
            v-for="(log, i) in logs"
            :key="i"
            class="text-xs text-gray-600 py-0.5"
          >
            {{ typeof log === 'string' ? log : (log.text ?? log.message ?? JSON.stringify(log)) }}
          </div>
        </div>
      </Card>

      <!-- Switch Modal -->
      <BattleSwitchModal
        :open="switchModalOpen"
        :reason="switchReason"
        @close="switchModalOpen = false"
        @action="onAction"
      />
    </template>

    <!-- Battle Ended -->
    <div v-else>
      <BattleResultSection />
    </div>
  </div>

  <!-- No active scene -->
  <div v-else class="text-center py-12 text-gray-400">
    <p>没有活跃的战斗</p>
  </div>
</template>
