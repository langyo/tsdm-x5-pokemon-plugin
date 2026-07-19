import { computed, defineComponent } from 'vue'
import { usePokemonStore, useBattleStore } from '@/stores'
import { api } from '@/api/client'
import { spriteUrl, hpClass } from '@/utils/pokemon'
import Modal from '@/components/common/Modal'

export default defineComponent({
  name: 'BattleSwitchModal',
  props: {
    open: { type: Boolean, required: true },
    reason: { type: String as () => 'switch' | 'replace', required: true },
  },
  emits: ['close', 'action'],
  setup(props, { emit }) {
    const pokemonStore = usePokemonStore()
    const battleStore = useBattleStore()

    const currentPokemonId = computed(() => battleStore.scene?.my_pokemon?.id)

    const switchablePokemons = computed(() => {
      return pokemonStore.bagPokemons.filter((p: any) => p.id !== currentPokemonId.value)
    })

    async function doSwitch(pokemonId: number) {
      const action = props.reason === 'replace' ? 'replace_pokemon' : 'switch_pokemon'
      emit('action')
      try {
        const data: any = await api.post('battle', { action, pokemon_id: pokemonId })
        battleStore.setScene(data.scene ?? data)
        emit('close')
      } catch {
        // handled by parent
      }
    }

    const hpPct = (pokemon: any) => Math.max(0, Math.min(100, ((pokemon.hp ?? 0) / (pokemon.maxHp ?? pokemon.max_hp ?? 1)) * 100))

    return () => (
      <Modal
        open={props.open}
        title={props.reason === 'replace' ? '选择替换宝可梦' : '选择出场宝可梦'}
        onClose={() => emit('close')}
      >
        {!switchablePokemons.value.length ? (
          <div class="text-center py-8 text-gray-400 text-sm">
            没有可用的宝可梦
          </div>
        ) : (
          <div class="space-y-2 max-h-80 overflow-y-auto">
            {switchablePokemons.value.map((pokemon: any) => (
              <button
                key={pokemon.id}
                class="w-full flex items-center gap-3 p-3 rounded-lg border border-border bg-white hover:bg-gray-50 transition-colors cursor-pointer text-left"
                onClick={() => doSwitch(pokemon.id)}
              >
                {pokemon.pmno ? (
                  <img
                    src={spriteUrl(pokemon.pmno)}
                    alt={pokemon.name}
                    class="w-10 h-10 object-contain shrink-0"
                  />
                ) : (
                  <div class="w-10 h-10 shrink-0 bg-gray-100 rounded flex items-center justify-center text-xs text-gray-400">
                    无
                  </div>
                )}
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2">
                    <span class="text-sm font-bold truncate">{pokemon.name}</span>
                    <span class="text-xs text-gray-400">Lv.{pokemon.level}</span>
                  </div>
                  <div class="mt-1">
                    <div class="flex items-center gap-2">
                      <div class="flex-1 h-2 bg-gray-200 rounded-full overflow-hidden">
                        <div
                          class={['h-full rounded-full transition-all', hpClass(pokemon.hp ?? 0, pokemon.maxHp ?? pokemon.max_hp ?? 1)]}
                          style={{ width: `${hpPct(pokemon)}%` }}
                        />
                      </div>
                      <span class="text-xs text-gray-400 shrink-0">
                        {pokemon.hp ?? 0}/{pokemon.maxHp ?? pokemon.max_hp ?? 0}
                      </span>
                    </div>
                  </div>
                </div>
              </button>
            ))}
          </div>
        )}
      </Modal>
    )
  },
})
