<script setup lang="ts">
import { ref, computed } from 'vue'
import { useBattleStore, usePokemonStore, useUserStore } from '@/stores'
import { api } from '@/api/client'
import { spriteUrl } from '@/utils/pokemon'
import Card from '@/components/common/Card.vue'
import TypeBadge from '@/components/common/TypeBadge.vue'

const battleStore = useBattleStore()
const pokemonStore = usePokemonStore()
const userStore = useUserStore()

const status = computed(() => battleStore.scene?.status)
const scene = computed(() => battleStore.scene)

const continueLoading = ref(false)

const statusConfig = computed(() => {
  const s = status.value
  switch (s) {
    case 'Victory':
      return { title: '战斗胜利！', icon: '🎉', color: 'text-yellow-600', bg: 'bg-yellow-50' }
    case 'Defeat':
      return { title: '战斗失败...', icon: '💔', color: 'text-red-600', bg: 'bg-red-50' }
    case 'Fled':
      return { title: '逃跑成功', icon: '🏃', color: 'text-blue-600', bg: 'bg-blue-50' }
    case 'Captured':
      return { title: '捕捉成功！', icon: '✨', color: 'text-green-600', bg: 'bg-green-50' }
    default:
      return { title: '战斗结束', icon: '', color: 'text-gray-600', bg: 'bg-gray-50' }
  }
})

const rewards = computed(() => scene.value?.rewards ?? scene.value?.reward ?? null)

const capturedPokemon = computed(() => {
  return scene.value?.captured_pokemon ?? scene.value?.caught_pokemon ?? null
})

async function onContinue() {
  if (continueLoading.value) return
  continueLoading.value = true
  try {
    const data: any = await api.post('battle', { action: 'recover' })
    battleStore.setScene(data.scene ?? null)
    if (!data.scene) battleStore.clear()
    await Promise.all([
      pokemonStore.fetchList(),
      userStore.fetchProfile(),
      userStore.fetchInventoryStats(),
    ])
  } catch {
    battleStore.clear()
  } finally {
    continueLoading.value = false
  }
}
</script>

<template>
  <div class="space-y-4">
    <div class="text-center py-6 rounded-xl" :class="statusConfig.bg">
      <div class="text-4xl mb-2">{{ statusConfig.icon }}</div>
      <h2 class="text-xl font-bold" :class="statusConfig.color">{{ statusConfig.title }}</h2>
    </div>

    <Card v-if="status === 'Victory' && rewards">
      <div class="text-sm font-bold mb-2">获得奖励</div>
      <div class="space-y-1 text-sm">
        <div v-if="rewards.exp !== undefined" class="flex justify-between">
          <span class="text-gray-500">经验值</span>
          <span class="font-bold text-blue-600">+{{ rewards.exp }}</span>
        </div>
        <div v-if="rewards.money !== undefined" class="flex justify-between">
          <span class="text-gray-500">金币</span>
          <span class="font-bold text-yellow-600">+{{ rewards.money }}</span>
        </div>
        <div v-if="rewards.items?.length" class="mt-2 pt-2 border-t border-border">
          <div class="text-gray-500 mb-1">掉落道具</div>
          <div v-for="item in rewards.items" :key="item.id" class="text-xs text-gray-600">
            {{ item.name }} x{{ item.count ?? 1 }}
          </div>
        </div>
      </div>
    </Card>

    <Card v-if="status === 'Captured' && capturedPokemon">
      <div class="text-sm font-bold mb-3">捕捉到的宝可梦</div>
      <div class="flex items-center gap-4">
        <img
          v-if="capturedPokemon.pmno"
          :src="spriteUrl(capturedPokemon.pmno)"
          :alt="capturedPokemon.name"
          class="w-16 h-16 object-contain"
        />
        <div class="flex-1 min-w-0">
          <div class="flex items-center gap-2 mb-1">
            <span class="text-base font-bold">{{ capturedPokemon.name }}</span>
            <span class="text-xs text-gray-400">Lv.{{ capturedPokemon.level }}</span>
          </div>
          <div class="flex gap-1 mb-1">
            <TypeBadge v-if="capturedPokemon.type1" :type="capturedPokemon.type1" />
            <TypeBadge v-if="capturedPokemon.type2" :type="capturedPokemon.type2" />
          </div>
          <div v-if="capturedPokemon.hp !== undefined" class="text-xs text-gray-400">
            HP {{ capturedPokemon.hp }}/{{ capturedPokemon.maxHp ?? capturedPokemon.max_hp }}
          </div>
        </div>
      </div>
    </Card>

    <button
      class="w-full py-3 rounded-lg text-sm font-bold text-white transition-colors"
      :class="continueLoading ? 'bg-gray-400 cursor-not-allowed' : 'bg-primary hover:bg-primary/80 cursor-pointer'"
      :disabled="continueLoading"
      @click="onContinue"
    >
      <template v-if="continueLoading">处理中...</template>
      <template v-else-if="status === 'Victory'">继续冒险</template>
      <template v-else-if="status === 'Defeat'">返回治疗</template>
      <template v-else-if="status === 'Captured'">继续</template>
      <template v-else>继续</template>
    </button>
  </div>
</template>
