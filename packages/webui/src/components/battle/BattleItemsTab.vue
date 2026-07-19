<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useBattleStore } from '@/stores'
import { api } from '@/api/client'
import Modal from '@/components/common/Modal.vue'

const props = defineProps<{
  waiting: boolean
}>()

const emit = defineEmits<{
  action: []
}>()

const battleStore = useBattleStore()

const items = ref<any[]>([])
const loading = ref(false)
const error = ref('')

const ppSkillModalOpen = ref(false)
const selectedItem = ref<any>(null)

const mySkills = computed(() => {
  return battleStore.scene?.my_pokemon?.skills ?? []
})

const usableItems = computed(() => {
  return items.value.filter((i: any) => i.type !== 2)
})

async function fetchItems() {
  loading.value = true
  error.value = ''
  try {
    const data: any = await api.get('battle', { action: 'get_battle_items' })
    items.value = data.items ?? data.data ?? []
  } catch (e: any) {
    error.value = e.message || '获取道具失败'
  } finally {
    loading.value = false
  }
}

function isPPItem(item: any): boolean {
  const name = (item.name ?? '').toLowerCase()
  return name.includes('pp') || item.category === 'pp' || item.effect_type === 'pp_restore'
}

async function useItem(item: any) {
  if (props.waiting) return
  if (isPPItem(item)) {
    selectedItem.value = item
    ppSkillModalOpen.value = true
    return
  }
  await doUseItem(item.id, undefined)
}

async function restorePP(skillId: number) {
  await doUseItem(selectedItem.value.id, skillId)
  ppSkillModalOpen.value = false
  selectedItem.value = null
}

async function doUseItem(itemId: number, skillId?: number) {
  emit('action')
  try {
    const body: Record<string, any> = { action: 'use_item', item_id: itemId }
    if (skillId !== undefined) body.skill_id = skillId
    const data: any = await api.post('battle', body)
    battleStore.setScene(data.scene ?? data)
  } catch {
    // handled by parent
  }
}

onMounted(fetchItems)
</script>

<template>
  <div>
    <div v-if="loading" class="text-center py-8 text-gray-400 text-sm">加载道具中...</div>

    <div v-else-if="error" class="text-center py-8 text-red-500 text-sm">{{ error }}</div>

    <div v-else-if="!usableItems.length" class="text-center py-8 text-gray-400 text-sm">
      没有可用的道具
    </div>

    <div v-else class="grid grid-cols-2 gap-2">
      <button
        v-for="item in usableItems"
        :key="item.id"
        class="flex flex-col items-center gap-1 p-3 rounded-lg border border-border bg-white hover:bg-gray-50 transition-colors cursor-pointer text-center"
        :class="waiting ? 'opacity-50 cursor-not-allowed' : ''"
        :disabled="waiting"
        @click="useItem(item)"
      >
        <span class="text-sm font-bold truncate w-full">{{ item.name }}</span>
        <span class="text-xs text-gray-500">
          <span v-if="item.count !== undefined">x{{ item.count }}</span>
          <span v-if="item.description" class="ml-1">{{ item.description }}</span>
        </span>
        <span v-if="isPPItem(item)" class="text-xs text-blue-500 mt-0.5">PP恢复</span>
      </button>
    </div>

    <Modal
      :open="ppSkillModalOpen"
      title="选择恢复的技能"
      @close="ppSkillModalOpen = false; selectedItem = null"
    >
      <div v-if="!mySkills.length" class="text-center py-8 text-gray-400 text-sm">
        当前宝可梦没有技能
      </div>
      <div v-else class="space-y-2">
        <button
          v-for="skill in mySkills"
          :key="skill.id"
          class="w-full flex items-center justify-between p-3 rounded-lg border border-border bg-white hover:bg-gray-50 transition-colors cursor-pointer text-left"
          @click="restorePP(skill.id)"
        >
          <div>
            <div class="text-sm font-bold">{{ skill.name }}</div>
            <div class="text-xs text-gray-400">
              PP {{ skill.pp ?? skill.current_pp ?? 0 }}/{{ skill.max_pp ?? skill.pp_max ?? 0 }}
            </div>
          </div>
          <span class="text-xs text-blue-500">恢复</span>
        </button>
      </div>
    </Modal>
  </div>
</template>
