<script setup lang="ts">
import { spriteUrl, hpClass } from '@/utils/pokemon'

defineProps<{ pokemons: any[] }>()
const emit = defineEmits<{ select: [id: number] }>()
</script>

<template>
  <div class="grid grid-cols-3 gap-1">
    <div
      v-for="pokemon in pokemons"
      :key="pokemon.id"
      class="relative bg-white rounded border border-border p-1 cursor-pointer hover:shadow-md transition-shadow"
      @click="emit('select', pokemon.id)"
    >
      <div class="flex items-center gap-1">
        <img
          :src="spriteUrl(pokemon.pmno)"
          :alt="String(pokemon.pmno)"
          class="w-8 h-8 object-contain"
        />
        <div class="flex-1 min-w-0">
          <div class="flex items-center justify-between">
            <span class="text-xs text-gray-500">Lv.{{ pokemon.level }}</span>
            <span class="text-xs text-gray-400">{{ pokemon.site || '' }}</span>
          </div>
          <div v-if="pokemon.hp !== undefined" class="w-full h-1.5 bg-gray-200 rounded-full mt-0.5 overflow-hidden">
            <div
              class="h-full rounded-full transition-all"
              :class="hpClass(pokemon.hp, pokemon.maxHp || pokemon.hp || 1)"
              :style="{ width: Math.max(0, Math.min(100, pokemon.maxHp ? (pokemon.hp / pokemon.maxHp) * 100 : 100)) + '%' }"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
