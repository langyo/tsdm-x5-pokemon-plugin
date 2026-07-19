import { ref, computed, defineComponent } from 'vue'
import { useBattleStore, usePokemonStore, useUserStore } from '@/stores'
import { api } from '@/api/client'
import { spriteUrl } from '@/utils/pokemon'
import Card from '@/components/common/Card'
import TypeBadge from '@/components/common/TypeBadge'

export default defineComponent({
  name: 'BattleResultSection',
  setup() {
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

    return () => {
      const cfg = statusConfig.value

      let btnText: string
      if (continueLoading.value) {
        btnText = '处理中...'
      } else if (status.value === 'Victory') {
        btnText = '继续冒险'
      } else if (status.value === 'Defeat') {
        btnText = '返回治疗'
      } else if (status.value === 'Captured') {
        btnText = '继续'
      } else {
        btnText = '继续'
      }

      return (
        <div class="space-y-4">
          <div class={['text-center py-6 rounded-xl', cfg.bg]}>
            <div class="text-4xl mb-2">{cfg.icon}</div>
            <h2 class={['text-xl font-bold', cfg.color]}>{cfg.title}</h2>
          </div>

          {status.value === 'Victory' && rewards.value && (
            <Card>
              <div class="text-sm font-bold mb-2">获得奖励</div>
              <div class="space-y-1 text-sm">
                {rewards.value.exp !== undefined && (
                  <div class="flex justify-between">
                    <span class="text-gray-500">经验值</span>
                    <span class="font-bold text-blue-600">+{rewards.value.exp}</span>
                  </div>
                )}
                {rewards.value.money !== undefined && (
                  <div class="flex justify-between">
                    <span class="text-gray-500">金币</span>
                    <span class="font-bold text-yellow-600">+{rewards.value.money}</span>
                  </div>
                )}
                {rewards.value.items?.length > 0 && (
                  <div class="mt-2 pt-2 border-t border-border">
                    <div class="text-gray-500 mb-1">掉落道具</div>
                    {rewards.value.items.map((item: any) => (
                      <div key={item.id} class="text-xs text-gray-600">
                        {item.name} x{item.count ?? 1}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </Card>
          )}

          {status.value === 'Captured' && capturedPokemon.value && (
            <Card>
              <div class="text-sm font-bold mb-3">捕捉到的宝可梦</div>
              <div class="flex items-center gap-4">
                {capturedPokemon.value.pmno ? (
                  <img
                    src={spriteUrl(capturedPokemon.value.pmno)}
                    alt={capturedPokemon.value.name}
                    class="w-16 h-16 object-contain"
                  />
                ) : null}
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="text-base font-bold">{capturedPokemon.value.name}</span>
                    <span class="text-xs text-gray-400">Lv.{capturedPokemon.value.level}</span>
                  </div>
                  <div class="flex gap-1 mb-1">
                    {capturedPokemon.value.type1 && <TypeBadge type={capturedPokemon.value.type1} />}
                    {capturedPokemon.value.type2 && <TypeBadge type={capturedPokemon.value.type2} />}
                  </div>
                  {capturedPokemon.value.hp !== undefined && (
                    <div class="text-xs text-gray-400">
                      HP {capturedPokemon.value.hp}/{capturedPokemon.value.maxHp ?? capturedPokemon.value.max_hp}
                    </div>
                  )}
                </div>
              </div>
            </Card>
          )}

          <button
            class={['w-full py-3 rounded-lg text-sm font-bold text-white transition-colors', continueLoading.value ? 'bg-gray-400 cursor-not-allowed' : 'bg-primary hover:bg-primary/80 cursor-pointer']}
            disabled={continueLoading.value}
            onClick={onContinue}
          >
            {btnText}
          </button>
        </div>
      )
    }
  },
})
