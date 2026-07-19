import { defineComponent, ref, computed, onMounted } from 'vue'
import AppLayout from '@/components/layout/AppLayout'
import Card from '@/components/common/Card'
import { usePokemonStore } from '@/stores'
import { api } from '@/api/client'
import { spriteUrl, hpClass } from '@/utils/pokemon'

export default defineComponent({
  name: 'PokemonCenterPage',
  setup() {
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

    return () => (
      <AppLayout>
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-lg font-bold">宠物中心</h2>
          {injuredPokemons.value.length > 0 && (
            <button
              class={`px-4 py-2 rounded-lg text-sm font-bold text-white transition-colors ${healingAll.value ? 'bg-gray-400 cursor-not-allowed' : 'bg-red-500 hover:bg-red-600'}`}
              disabled={healingAll.value}
              onClick={healAll}
            >
              {healingAll.value ? '治疗中...' : '一键治疗'}
            </button>
          )}
        </div>

        {error.value && <div class="card text-center py-8 text-red-500 mb-4">{error.value}</div>}

        {loading.value ? (
          <div class="text-center py-12 text-gray-400">加载中...</div>
        ) : !injuredPokemons.value.length ? (
          <div class="card text-center py-12">
            <div class="text-4xl mb-3">&#10004;</div>
            <p class="text-gray-400">所有宠物都很健康！</p>
          </div>
        ) : (
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            {injuredPokemons.value.map((pokemon: any) => {
              const isHealing = healingId.value === pokemon.id
              const hpMax = pokemon.maxHp ?? pokemon.hp
              const hpWidth = hpMax > 0 ? Math.max(0, Math.min(100, (pokemon.hp / hpMax) * 100)) : 0
              return (
                <Card key={pokemon.id} class="flex items-center gap-4">
                  <img
                    src={spriteUrl(pokemon.pmno)}
                    alt={pokemon.name}
                    class="w-14 h-14 object-contain shrink-0"
                  />
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2">
                      <span class="font-bold text-sm truncate">{pokemon.name}</span>
                      <span class="text-xs text-gray-500">Lv.{pokemon.level}</span>
                      {(pokemon.state ?? 0) !== 0 && (
                        <span class="inline-block rounded-full px-2 py-0.5 text-xs font-bold text-white bg-purple-500">
                          {stateLabels[pokemon.state] ?? '异常'}
                        </span>
                      )}
                    </div>
                    <div class="w-full h-2 bg-gray-200 rounded-full mt-1.5 mb-1 overflow-hidden">
                      <div
                        class={`h-full rounded-full transition-all ${hpClass(pokemon.hp, hpMax)}`}
                        style={{ width: hpWidth + '%' }}
                      />
                    </div>
                    <div class="text-xs text-gray-400">
                      HP: {pokemon.hp} / {hpMax}
                      {pokemon.site === 1 && <span class="text-red-400 ml-2">战斗中</span>}
                    </div>
                  </div>
                  {pokemon.site === 1 ? (
                    <button
                      class={`shrink-0 px-3 py-1.5 rounded-lg text-sm font-bold text-white transition-colors ${isHealing ? 'bg-gray-400 cursor-not-allowed' : 'bg-orange-500 hover:bg-orange-600'}`}
                      disabled={isHealing}
                      onClick={() => healAndLeave(pokemon)}
                    >
                      {isHealing ? '...' : '脱战并治疗'}
                    </button>
                  ) : (
                    <button
                      class={`shrink-0 px-3 py-1.5 rounded-lg text-sm font-bold text-white transition-colors ${isHealing ? 'bg-gray-400 cursor-not-allowed' : 'bg-green-500 hover:bg-green-600'}`}
                      disabled={isHealing}
                      onClick={() => heal(pokemon)}
                    >
                      {isHealing ? '...' : '治疗'}
                    </button>
                  )}
                </Card>
              )
            })}
          </div>
        )}
      </AppLayout>
    )
  }
})
