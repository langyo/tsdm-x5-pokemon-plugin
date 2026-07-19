import { defineComponent, ref, computed, onMounted } from 'vue'
import { api } from '@/api/client'
import { usePokemonStore } from '@/stores'
import AppLayout from '@/components/layout/AppLayout'
import Card from '@/components/common/Card'
import PopupMenu from '@/components/common/PopupMenu'
import { spriteUrl, hpClass } from '@/utils/pokemon'

export default defineComponent({
  name: 'StoragePage',
  setup() {
    const pokemonStore = usePokemonStore()

    const loading = ref(true)
    const error = ref<string | null>(null)

    const contextMenuOpen = ref(false)
    const contextMenuX = ref(0)
    const contextMenuY = ref(0)
    const contextTarget = ref<any>(null)

    const multiSelectMode = ref(false)
    const selectedIds = ref<Set<number>>(new Set())
    const releasingAll = ref(false)

    const bagSlots = computed(() => {
      const pokemons = pokemonStore.bagPokemons
      const slots: (any | null)[] = []
      for (let i = 0; i < 6; i++) {
        slots.push(pokemons[i] ?? null)
      }
      return slots
    })

    const storagePokemons = computed(() => pokemonStore.storagePokemons)

    const hasBagPokemon = computed(() => pokemonStore.bagPokemons.length > 0)
    const hasStoragePokemon = computed(() => pokemonStore.storagePokemons.length > 0)
    const allSelected = computed(() =>
      storagePokemons.value.length > 0 && selectedIds.value.size === storagePokemons.value.length
    )

    async function fetchPokemons() {
      loading.value = true
      error.value = null
      try {
        await pokemonStore.fetchList()
      } catch (e: any) {
        error.value = e.message || '加载失败'
      } finally {
        loading.value = false
      }
    }

    function openContextMenu(event: MouseEvent, pokemon: any) {
      contextTarget.value = pokemon
      contextMenuX.value = event.clientX
      contextMenuY.value = event.clientY
      contextMenuOpen.value = true
    }

    function closeContextMenu() {
      contextMenuOpen.value = false
      contextTarget.value = null
    }

    async function handleSetFirst() {
      if (!contextTarget.value) return
      closeContextMenu()
      try {
        await api.post('pokemon', { action: 'set_first', pokemon_id: contextTarget.value.id })
        await fetchPokemons()
      } catch (e: any) {
        alert(e.message || '设置失败')
      }
    }

    async function handleMoveToStorage(pokemon?: any) {
      const target = pokemon || contextTarget.value
      if (!target) return
      closeContextMenu()
      try {
        await api.post('pokemon', { action: 'move_pokemon', pokemon_id: target.id, site: 3 })
        await fetchPokemons()
      } catch (e: any) {
        alert(e.message || '移动失败')
      }
    }

    async function handleMoveToBag(pokemon?: any) {
      const target = pokemon || contextTarget.value
      if (!target) return
      closeContextMenu()
      const targetSite = pokemonStore.bagPokemons.length >= 6 ? null : 2
      if (targetSite === null) {
        alert('背包已满（最多6只），请先将背包中的宝可梦放入仓库')
        return
      }
      try {
        await api.post('pokemon', { action: 'move_pokemon', pokemon_id: target.id, site: targetSite })
        await fetchPokemons()
      } catch (e: any) {
        alert(e.message || '移动失败')
      }
    }

    async function handleRelease(pokemon?: any) {
      const target = pokemon || contextTarget.value
      if (!target) return
      closeContextMenu()
      if (!confirm(`确定要放生「${target.name || 'No.' + target.pmno}」吗？此操作不可撤销。`)) return
      try {
        await api.post('pokemon', { action: 'release', pokemon_id: target.id })
        await fetchPokemons()
      } catch (e: any) {
        alert(e.message || '放生失败')
      }
    }

    function toggleMultiSelect() {
      multiSelectMode.value = !multiSelectMode.value
      selectedIds.value = new Set()
    }

    function toggleSelect(pokemonId: number) {
      const next = new Set(selectedIds.value)
      if (next.has(pokemonId)) {
        next.delete(pokemonId)
      } else {
        next.add(pokemonId)
      }
      selectedIds.value = next
    }

    function toggleSelectAll() {
      if (allSelected.value) {
        selectedIds.value = new Set()
      } else {
        selectedIds.value = new Set(storagePokemons.value.map((p: any) => p.id))
      }
    }

    async function batchRelease() {
      if (selectedIds.value.size === 0) return
      if (!confirm(`确定要放生选中的 ${selectedIds.value.size} 只宝可梦吗？此操作不可撤销。`)) return
      releasingAll.value = true
      let failed = 0
      for (const id of selectedIds.value) {
        try {
          await api.post('pokemon', { action: 'release', pokemon_id: id })
        } catch {
          failed++
        }
      }
      releasingAll.value = false
      multiSelectMode.value = false
      selectedIds.value = new Set()
      await fetchPokemons()
      if (failed > 0) {
        alert(`放生完成，${failed} 只失败`)
      }
    }

    function hpPct(pokemon: any): number {
      const hp = pokemon.hp ?? 0
      const maxHp = pokemon.maxhp ?? pokemon.maxHp ?? 1
      return maxHp > 0 ? Math.min(100, (hp / maxHp) * 100) : 0
    }

    function battleStateLabel(pokemon: any): string {
      if (pokemon.state && pokemon.state !== 0) {
        const labels: Record<number, string> = { 1: '中毒', 2: '麻痹', 3: '烧伤', 4: '冰冻', 5: '睡眠' }
        return labels[pokemon.state] ?? '异常'
      }
      return ''
    }

    function isBag(pokemon: any): boolean {
      return pokemon.site === 1 || pokemon.site === 2
    }

    onMounted(() => {
      fetchPokemons()
    })

    return () => (
      <AppLayout>
        {/* Loading */}
        {loading.value ? (
          <div class="text-center py-16">
            <div class="animate-spin w-8 h-8 border-3 border-primary border-t-transparent rounded-full mx-auto mb-3" />
            <p class="text-gray-500 text-sm">加载宝可梦...</p>
          </div>
        ) : error.value ? (
          /* Error */
          <div class="text-center py-16">
            <p class="text-red-500 mb-4">{error.value}</p>
            <button
              class="bg-primary text-white px-4 py-2 rounded-lg text-sm"
              onClick={fetchPokemons}
            >
              重试
            </button>
          </div>
        ) : (
          <div class="space-y-6">
            {/* Bag Panel */}
            <Card title="随身背包">
              {!hasBagPokemon.value ? (
                <div class="text-center py-6 text-gray-400 text-sm">背包中没有宝可梦</div>
              ) : (
                <div class="grid grid-cols-3 md:grid-cols-6 gap-2">
                  {bagSlots.value.map((pokemon: any, idx: number) => (
                    <div
                      key={idx}
                      class={`relative rounded-lg border min-h-[100px] cursor-pointer transition-shadow hover:shadow-md ${pokemon ? 'border-border bg-white' : 'border-dashed border-gray-200 bg-gray-50'}`}
                      onClick={() => pokemon && openContextMenu(event as MouseEvent, pokemon)}
                    >
                      {pokemon ? (
                        <div class="flex flex-col items-center p-2">
                          <img
                            src={spriteUrl(pokemon.pmno)}
                            alt={pokemon.name}
                            class="w-10 h-10 object-contain"
                          />
                          <span class="text-xs font-medium mt-1 truncate w-full text-center">{pokemon.name || `No.${pokemon.pmno}`}</span>
                          <span class="text-xs text-gray-400">Lv.{pokemon.level}</span>
                          <div class="w-full h-1.5 bg-gray-200 rounded-full mt-1 overflow-hidden">
                            <div
                              class={`h-full rounded-full ${hpClass(pokemon.hp ?? 0, pokemon.maxhp ?? pokemon.maxHp ?? 1)}`}
                              style={{ width: hpPct(pokemon) + '%' }}
                            />
                          </div>
                          <div class="flex items-center gap-1 mt-0.5">
                            {pokemon.site === 1 && (
                              <span class="text-xs px-1 rounded bg-red-100 text-red-500 font-bold">首位</span>
                            )}
                            {pokemon.site === 2 && (
                              <span class="text-xs px-1 rounded bg-blue-100 text-blue-500">同行</span>
                            )}
                            {battleStateLabel(pokemon) && (
                              <span class="text-xs px-1 rounded bg-purple-100 text-purple-500">{battleStateLabel(pokemon)}</span>
                            )}
                          </div>
                        </div>
                      ) : (
                        <div class="flex items-center justify-center h-full text-gray-300 text-xs">空位</div>
                      )}
                    </div>
                  ))}
                </div>
              )}
            </Card>

            {/* Storage Panel */}
            <Card>
              <div class="flex items-center justify-between mb-3">
                <span class="text-base font-bold">宠物仓库</span>
                <div class="flex gap-2">
                  {hasStoragePokemon.value && !multiSelectMode.value && (
                    <button
                      class="text-xs bg-gray-100 text-gray-600 px-3 py-1 rounded hover:bg-gray-200 transition-colors"
                      onClick={toggleMultiSelect}
                    >
                      批量操作
                    </button>
                  )}
                  {multiSelectMode.value && (
                    <>
                      <button
                        class={`text-xs px-3 py-1 rounded transition-colors ${allSelected.value ? 'bg-gray-200 text-gray-600 hover:bg-gray-300' : 'bg-primary/10 text-primary hover:bg-primary/20'}`}
                        onClick={toggleSelectAll}
                      >
                        {allSelected.value ? '取消全选' : '全选'}
                      </button>
                      <button
                        class="text-xs bg-red-500 text-white px-3 py-1 rounded hover:bg-red-600 transition-colors disabled:opacity-50"
                        disabled={selectedIds.value.size === 0 || releasingAll.value}
                        onClick={batchRelease}
                      >
                        {releasingAll.value ? '放生中...' : `放生 (${selectedIds.value.size})`}
                      </button>
                      <button
                        class="text-xs bg-gray-100 text-gray-600 px-3 py-1 rounded hover:bg-gray-200 transition-colors"
                        onClick={toggleMultiSelect}
                      >
                        取消
                      </button>
                    </>
                  )}
                </div>
              </div>

              {!hasStoragePokemon.value ? (
                <div class="text-center py-8 text-gray-400 text-sm">仓库中没有宝可梦</div>
              ) : (
                <div class="grid grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-2">
                  {storagePokemons.value.map((pokemon: any) => {
                    const isSelected = multiSelectMode.value && selectedIds.value.has(pokemon.id)
                    return (
                      <div
                        key={pokemon.id}
                        class={`relative rounded-lg border border-border bg-white min-h-[100px] cursor-pointer transition-shadow hover:shadow-md ${isSelected ? 'ring-2 ring-primary' : ''}`}
                        onClick={() => multiSelectMode.value ? toggleSelect(pokemon.id) : openContextMenu(event as MouseEvent, pokemon)}
                      >
                        {multiSelectMode.value && (
                          <div class="absolute top-1 left-1 z-10">
                            <div
                              class={`w-4 h-4 rounded border-2 flex items-center justify-center ${isSelected ? 'bg-primary border-primary' : 'border-gray-300 bg-white'}`}
                            >
                              {isSelected && <span class="text-white text-xs font-bold">&#10003;</span>}
                            </div>
                          </div>
                        )}
                        <div class="flex flex-col items-center p-2">
                          <img
                            src={spriteUrl(pokemon.pmno)}
                            alt={pokemon.name}
                            class="w-10 h-10 object-contain"
                          />
                          <span class="text-xs font-medium mt-1 truncate w-full text-center">{pokemon.name || `No.${pokemon.pmno}`}</span>
                          <span class="text-xs text-gray-400">Lv.{pokemon.level}</span>
                          <div class="w-full h-1.5 bg-gray-200 rounded-full mt-1 overflow-hidden">
                            <div
                              class={`h-full rounded-full ${hpClass(pokemon.hp ?? 0, pokemon.maxhp ?? pokemon.maxHp ?? 1)}`}
                              style={{ width: hpPct(pokemon) + '%' }}
                            />
                          </div>
                          {battleStateLabel(pokemon) && (
                            <span class="text-xs mt-0.5 px-1 rounded bg-purple-100 text-purple-500">{battleStateLabel(pokemon)}</span>
                          )}
                        </div>
                      </div>
                    )
                  })}
                </div>
              )}
            </Card>
          </div>
        )}

        {/* Context Menu */}
        <PopupMenu open={contextMenuOpen.value} x={contextMenuX.value} y={contextMenuY.value} onClose={closeContextMenu}>
          {contextTarget.value && isBag(contextTarget.value) && (
            <button
              class="w-full text-left px-3 py-2 text-sm hover:bg-gray-50 transition-colors"
              onClick={handleSetFirst}
            >
              设为第一
            </button>
          )}
          {contextTarget.value && isBag(contextTarget.value) && (
            <button
              class="w-full text-left px-3 py-2 text-sm hover:bg-gray-50 transition-colors"
              onClick={() => handleMoveToStorage()}
            >
              放入仓库
            </button>
          )}
          {contextTarget.value && !isBag(contextTarget.value) && (
            <button
              class="w-full text-left px-3 py-2 text-sm hover:bg-gray-50 transition-colors"
              onClick={() => handleMoveToBag()}
            >
              放回背包
            </button>
          )}
          <button
            class="w-full text-left px-3 py-2 text-sm text-red-500 hover:bg-red-50 transition-colors"
            onClick={() => handleRelease()}
          >
            放生
          </button>
        </PopupMenu>
      </AppLayout>
    )
  }
})
