<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { api } from '@/api/client'
import { usePokemonStore } from '@/stores'
import AppLayout from '@/components/layout/AppLayout.vue'
import Card from '@/components/common/Card.vue'
import TypeBadge from '@/components/common/TypeBadge.vue'
import Modal from '@/components/common/Modal.vue'
import StatsRadar from '@/components/pokemon/StatsRadar.vue'
import { spriteUrl, hpClass, genderLabel } from '@/utils/pokemon'

const pokemonStore = usePokemonStore()

const loading = ref(true)
const error = ref<string | null>(null)
const selectedIndex = ref(0)
const selectedTab = ref<'stats' | 'equipment' | 'skills'>('stats')

const editingName = ref(false)
const renameValue = ref('')
const renameLoading = ref(false)

const learnModalOpen = ref(false)
const learnableSkills = ref<any[]>([])
const learnSkillsLoading = ref(false)

const pokemons = computed(() => pokemonStore.bagPokemons)
const selected = computed(() => pokemons.value[selectedIndex.value] ?? null)

const types = computed(() => {
  if (!selected.value) return []
  const t = selected.value.type ?? selected.value.pmtype ?? ''
  return t.split(/[,/]/).filter(Boolean).map((s: string) => s.trim())
})

const hpPct = computed(() => {
  if (!selected.value) return 0
  const hp = selected.value.hp ?? 0
  const maxHp = selected.value.maxhp ?? selected.value.maxHp ?? 1
  return maxHp > 0 ? Math.min(100, (hp / maxHp) * 100) : 0
})

const expPct = computed(() => {
  if (!selected.value) return 0
  const exp = selected.value.exp ?? selected.value.experience ?? 0
  const next = selected.value.nextexp ?? selected.value.nextExp ?? 1
  return next > 0 ? Math.min(100, (exp / next) * 100) : 0
})

const stats = computed(() => {
  if (!selected.value) return { hp: 0, atk: 0, def: 0, spatk: 0, spdef: 0, speed: 0 }
  return {
    hp: selected.value.maxhp ?? selected.value.maxHp ?? 0,
    atk: selected.value.atk ?? selected.value.attack ?? 0,
    def: selected.value.def ?? selected.value.defense ?? 0,
    spatk: selected.value.spatk ?? selected.value.spAttack ?? 0,
    spdef: selected.value.spdef ?? selected.value.spDefense ?? 0,
    speed: selected.value.speed ?? 0,
  }
})

const equipmentSlots = computed(() => {
  if (!selected.value) return []
  return [
    { slot: 'weapon', name: '武器', item: selected.value.weapon ?? null },
    { slot: 'armor', name: '防具', item: selected.value.armor ?? null },
    { slot: 'accessory', name: '饰品', item: selected.value.accessory ?? null },
    { slot: 'item', name: '道具', item: selected.value.equipItem ?? selected.value.item ?? null },
  ]
})

const skillSlots = computed(() => {
  if (!selected.value) return []
  const skills = selected.value.skills ?? selected.value.curSkills ?? []
  const slots: any[] = []
  for (let i = 0; i < 4; i++) {
    slots.push(skills[i] ?? null)
  }
  return slots
})

onMounted(async () => {
  loading.value = true
  error.value = null
  try {
    await pokemonStore.fetchList()
  } catch (e: any) {
    error.value = e.message || '加载失败'
  } finally {
    loading.value = false
  }
})

function selectPokemon(index: number) {
  selectedIndex.value = index
}

async function handleRename() {
  if (!selected.value || !renameValue.value.trim()) return
  renameLoading.value = true
  try {
    await api.post('pokemon', { action: 'rename', id: selected.value.id, name: renameValue.value.trim() })
    selected.value.name = renameValue.value.trim()
    editingName.value = false
  } catch (e: any) {
    alert(e.message || '改名失败')
  } finally {
    renameLoading.value = false
  }
}

function openRename() {
  renameValue.value = selected.value?.name ?? ''
  editingName.value = true
}

async function handleEvolve() {
  if (!selected.value) return
  if (selected.value.level < 15) {
    alert('等级需达到15级才能进化')
    return
  }
  try {
    await api.post('pokemon', { action: 'evolve', id: selected.value.id })
    await pokemonStore.fetchList()
  } catch (e: any) {
    alert(e.message || '进化失败')
  }
}

async function handleUnequip(slot: string) {
  if (!selected.value) return
  try {
    await api.post('pokemon', { action: 'unequip', id: selected.value.id, slot })
    await pokemonStore.fetchList()
  } catch (e: any) {
    alert(e.message || '卸载失败')
  }
}

async function handleUseSkill(skill: any) {
  if (!skill) return
  try {
    await api.post('pokemon', { action: 'useskill', id: selected.value.id, skillId: skill.id })
    await pokemonStore.fetchList()
  } catch (e: any) {
    alert(e.message || '使用技能失败')
  }
}

async function handleForgetSkill(skill: any) {
  if (!skill) return
  if (!confirm(`确定要遗忘技能「${skill.name}」吗？`)) return
  try {
    await api.post('pokemon', { action: 'forgetskill', id: selected.value.id, skillId: skill.id })
    await pokemonStore.fetchList()
  } catch (e: any) {
    alert(e.message || '遗忘失败')
  }
}

async function openLearnModal() {
  learnModalOpen.value = true
  learnSkillsLoading.value = true
  try {
    const data = await api.get<any>('pokemon', { action: 'learnable_skills', id: selected.value.id })
    learnableSkills.value = data.skills ?? data.data ?? []
  } catch (e: any) {
    alert(e.message || '获取可学技能失败')
    learnableSkills.value = []
  } finally {
    learnSkillsLoading.value = false
  }
}

async function handleLearnSkill(skill: any) {
  try {
    await api.post('pokemon', { action: 'learnskill', id: selected.value.id, skillId: skill.id })
    learnModalOpen.value = false
    await pokemonStore.fetchList()
  } catch (e: any) {
    alert(e.message || '学习技能失败')
  }
}

function siteLabel(site: number): string {
  if (site === 1) return '携带'
  if (site === 2) return '同行'
  if (site === 3) return '寄存'
  return '未知'
}
</script>

<template>
  <AppLayout>
    <div v-if="loading" class="text-center py-16">
      <div class="animate-spin w-8 h-8 border-3 border-primary border-t-transparent rounded-full mx-auto mb-3" />
      <p class="text-gray-500 text-sm">加载宝可梦列表...</p>
    </div>

    <div v-else-if="error" class="text-center py-16">
      <p class="text-red-500 mb-4">{{ error }}</p>
      <button
        class="bg-primary text-white px-4 py-2 rounded-lg text-sm"
        @click="loading = true; error = null; pokemonStore.fetchList().then(() => loading = false).catch((e: any) => { error = e.message; loading = false })"
      >
        重试
      </button>
    </div>

    <div v-else-if="!pokemons.length" class="text-center py-16">
      <p class="text-gray-400 text-lg">还没有宝可梦</p>
      <p class="text-gray-400 text-sm mt-2">去野外冒险捕捉你的第一只宝可梦吧</p>
    </div>

    <div v-else class="flex gap-4">
      <!-- Left Sidebar -->
      <div class="w-52 shrink-0">
        <Card title="我的宝可梦">
          <div class="space-y-1 max-h-[calc(100vh-180px)] overflow-y-auto">
            <div
              v-for="(pokemon, idx) in pokemons"
              :key="pokemon.id"
              class="flex items-center gap-2 p-2 rounded-lg cursor-pointer transition-colors"
              :class="idx === selectedIndex ? 'bg-primary/10 border border-primary/30' : 'hover:bg-gray-50 border border-transparent'"
              @click="selectPokemon(idx)"
            >
              <img
                :src="spriteUrl(pokemon.pmno)"
                :alt="String(pokemon.pmno)"
                class="w-10 h-10 object-contain shrink-0"
              />
              <div class="min-w-0 flex-1">
                <div class="text-xs font-medium truncate">
                  {{ pokemon.name || `No.${pokemon.pmno}` }}
                  <span class="text-gray-400 font-normal">{{ genderLabel(pokemon.sex) }}</span>
                </div>
                <div class="text-xs text-gray-400">Lv.{{ pokemon.level }}</div>
                <div class="w-full h-1 bg-gray-200 rounded-full mt-0.5 overflow-hidden">
                  <div
                    class="h-full rounded-full"
                    :class="hpClass(pokemon.hp, pokemon.maxhp ?? pokemon.maxHp ?? 1)"
                    :style="{ width: Math.max(0, Math.min(100, (pokemon.maxhp ?? pokemon.maxHp) ? ((pokemon.hp ?? 0) / (pokemon.maxhp ?? pokemon.maxHp ?? 1)) * 100 : 100)) + '%' }"
                  />
                </div>
              </div>
            </div>
          </div>
        </Card>
      </div>

      <!-- Right Panel -->
      <div class="flex-1 min-w-0">
        <template v-if="selected">
          <!-- Pokemon Header -->
          <div class="flex items-center gap-4 mb-4">
            <img
              :src="spriteUrl(selected.pmno)"
              :alt="String(selected.pmno)"
              class="w-16 h-16 object-contain"
            />
            <div>
              <div class="flex items-center gap-2">
                <span class="text-lg font-bold">{{ selected.name || `No.${selected.pmno}` }}</span>
                <span class="text-gray-400">{{ genderLabel(selected.sex) }}</span>
                <span class="text-xs bg-gray-100 text-gray-500 px-2 py-0.5 rounded-full">Lv.{{ selected.level }}</span>
              </div>
              <div class="flex items-center gap-1 mt-1">
                <TypeBadge v-for="t in types" :key="t" :type="t" />
              </div>
            </div>
          </div>

          <!-- Tabs -->
          <div class="flex gap-2 mb-4 border-b border-border pb-2">
            <button
              v-for="tab in (['stats', 'equipment', 'skills'] as const)"
              :key="tab"
              class="px-4 py-1.5 text-sm rounded-t-lg transition-colors"
              :class="selectedTab === tab ? 'bg-primary text-white' : 'text-gray-500 hover:text-gray-700'"
              @click="selectedTab = tab"
            >
              {{ tab === 'stats' ? '属性' : tab === 'equipment' ? '装备' : '技能' }}
            </button>
          </div>

          <!-- Stats Tab -->
          <div v-if="selectedTab === 'stats'" class="space-y-4">
            <Card>
              <div class="flex items-center justify-between mb-3">
                <div class="flex items-center gap-2">
                  <span v-if="!editingName" class="text-base font-bold">{{ selected.name || `No.${selected.pmno}` }}</span>
                  <input
                    v-else
                    v-model="renameValue"
                    class="border border-border rounded px-2 py-0.5 text-sm"
                    :disabled="renameLoading"
                    @keyup.enter="handleRename"
                    @keyup.escape="editingName = false"
                  />
                  <button
                    v-if="!editingName"
                    class="text-xs text-primary hover:underline"
                    @click="openRename"
                  >
                    改名
                  </button>
                  <template v-else>
                    <button
                      class="text-xs text-green-500 hover:underline"
                      :disabled="renameLoading"
                      @click="handleRename"
                    >
                      确认
                    </button>
                    <button
                      class="text-xs text-gray-400 hover:underline"
                      @click="editingName = false"
                    >
                      取消
                    </button>
                  </template>
                </div>
                <div class="flex items-center gap-2">
                  <button
                    class="text-xs bg-green-500 text-white px-3 py-1 rounded hover:opacity-90 transition-opacity disabled:opacity-50"
                    :disabled="selected.level < 15"
                    @click="handleEvolve"
                  >
                    进化
                  </button>
                </div>
              </div>

              <!-- HP Bar -->
              <div class="mb-3">
                <div class="flex justify-between text-xs text-gray-500 mb-1">
                  <span>HP</span>
                  <span>{{ selected.hp ?? 0 }} / {{ selected.maxhp ?? selected.maxHp ?? 0 }}</span>
                </div>
                <div class="w-full h-3 bg-gray-200 rounded-full overflow-hidden">
                  <div
                    class="h-full rounded-full transition-all duration-300"
                    :class="hpClass(selected.hp ?? 0, selected.maxhp ?? selected.maxHp ?? 1)"
                    :style="{ width: hpPct + '%' }"
                  />
                </div>
              </div>

              <!-- EXP Bar -->
              <div class="mb-3">
                <div class="flex justify-between text-xs text-gray-500 mb-1">
                  <span>EXP</span>
                  <span>{{ selected.exp ?? selected.experience ?? 0 }} / {{ selected.nextexp ?? selected.nextExp ?? 0 }}</span>
                </div>
                <div class="w-full h-2 bg-gray-200 rounded-full overflow-hidden">
                  <div
                    class="h-full rounded-full bg-blue-400 transition-all duration-300"
                    :style="{ width: expPct + '%' }"
                  />
                </div>
              </div>

              <!-- Types and States -->
              <div class="flex flex-wrap items-center gap-2 mb-3">
                <span class="text-xs text-gray-500">属性:</span>
                <TypeBadge v-for="t in types" :key="t" :type="t" />
                <template v-if="selected.state">
                  <span class="text-xs text-gray-500 ml-2">状态:</span>
                  <span class="text-xs bg-yellow-100 text-yellow-700 px-2 py-0.5 rounded-full">{{ selected.state }}</span>
                </template>
                <span class="text-xs text-gray-400 ml-auto">{{ siteLabel(selected.site) }}</span>
              </div>

              <!-- Stats Radar -->
              <StatsRadar :stats="stats" />

              <!-- Base Stats -->
              <div class="grid grid-cols-3 gap-2 mt-4">
                <div v-for="(val, key) in stats" :key="key" class="text-center bg-gray-50 rounded-lg p-2">
                  <div class="text-xs text-gray-400">{{ key === 'hp' ? 'HP' : key === 'atk' ? '攻击' : key === 'def' ? '防御' : key === 'spatk' ? '特攻' : key === 'spdef' ? '特防' : '速度' }}</div>
                  <div class="text-sm font-bold">{{ val }}</div>
                </div>
              </div>
            </Card>
          </div>

          <!-- Equipment Tab -->
          <div v-if="selectedTab === 'equipment'" class="space-y-4">
            <Card title="装备栏">
              <div class="grid grid-cols-2 gap-3">
                <div
                  v-for="slot in equipmentSlots"
                  :key="slot.slot"
                  class="border border-border rounded-lg p-3 text-center min-h-20"
                >
                  <div class="text-xs text-gray-400 mb-1">{{ slot.name }}</div>
                  <template v-if="slot.item">
                    <div class="text-sm font-medium">{{ slot.item.name }}</div>
                    <div class="text-xs text-gray-400">{{ slot.item.description || '' }}</div>
                    <button
                      class="text-xs text-red-400 hover:text-red-500 mt-1"
                      @click="handleUnequip(slot.slot)"
                    >
                      卸下
                    </button>
                  </template>
                  <div v-else class="text-sm text-gray-300">空</div>
                </div>
              </div>
            </Card>
          </div>

          <!-- Skills Tab -->
          <div v-if="selectedTab === 'skills'" class="space-y-4">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm font-bold">技能栏</span>
              <button
                class="text-xs bg-primary text-white px-3 py-1 rounded hover:opacity-90"
                @click="openLearnModal"
              >
                学习技能
              </button>
            </div>

            <div class="grid grid-cols-2 gap-3">
              <div
                v-for="(skill, idx) in skillSlots"
                :key="idx"
                class="border border-border rounded-lg p-3 min-h-20"
              >
                <div class="text-xs text-gray-400 mb-1">技能槽 {{ idx + 1 }}</div>
                <template v-if="skill">
                  <div class="flex items-center justify-between">
                    <span class="text-sm font-bold">{{ skill.name }}</span>
                    <TypeBadge v-if="skill.type" :type="skill.type" />
                  </div>
                  <div class="flex items-center gap-3 mt-1 text-xs text-gray-500">
                    <span>威力: {{ skill.power ?? '-' }}</span>
                    <span>PP: {{ skill.pp ?? skill.currentPp ?? '-' }}/{{ skill.maxpp ?? skill.maxPp ?? '-' }}</span>
                    <span>{{ skill.category ?? '' }}</span>
                  </div>
                  <div class="flex gap-2 mt-2">
                    <button
                      class="text-xs text-blue-500 hover:underline"
                      @click="handleUseSkill(skill)"
                    >
                      使用
                    </button>
                    <button
                      class="text-xs text-red-400 hover:underline"
                      @click="handleForgetSkill(skill)"
                    >
                      遗忘
                    </button>
                  </div>
                </template>
                <div v-else class="text-sm text-gray-300">空</div>
              </div>
            </div>
          </div>
        </template>

        <div v-else class="text-center py-16">
          <p class="text-gray-400">请从左侧列表选择一只宝可梦</p>
        </div>
      </div>
    </div>

    <!-- Learn Modal -->
    <Modal :open="learnModalOpen" title="可学习技能" wide @close="learnModalOpen = false">
      <div v-if="learnSkillsLoading" class="text-center py-8">
        <div class="animate-spin w-6 h-6 border-2 border-primary border-t-transparent rounded-full mx-auto mb-2" />
        <p class="text-sm text-gray-400">加载中...</p>
      </div>
      <div v-else-if="!learnableSkills.length" class="text-center py-8">
        <p class="text-gray-400 text-sm">暂无可学习的技能</p>
      </div>
      <div v-else class="space-y-2 max-h-96 overflow-y-auto">
        <div
          v-for="skill in learnableSkills"
          :key="skill.id"
          class="flex items-center gap-3 p-3 rounded-lg border border-border hover:bg-gray-50 cursor-pointer transition-colors"
          @click="handleLearnSkill(skill)"
        >
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <span class="text-sm font-bold">{{ skill.name }}</span>
              <TypeBadge v-if="skill.type" :type="skill.type" />
            </div>
            <div class="flex items-center gap-3 mt-1 text-xs text-gray-500">
              <span>威力: {{ skill.power ?? '-' }}</span>
              <span>PP: {{ skill.pp ?? skill.maxPp ?? '-' }}</span>
              <span>{{ skill.category ?? '' }}</span>
            </div>
            <div v-if="skill.description" class="text-xs text-gray-400 mt-0.5">{{ skill.description }}</div>
          </div>
          <span class="text-xs text-primary font-medium shrink-0">学习</span>
        </div>
      </div>
    </Modal>
  </AppLayout>
</template>
