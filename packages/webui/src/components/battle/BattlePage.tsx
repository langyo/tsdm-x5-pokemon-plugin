import { ref, computed, onMounted, defineComponent } from 'vue'
import { useBattleStore, usePokemonStore } from '@/stores'
import { api } from '@/api/client'
import { battleSpriteUrl, hpClass } from '@/utils/pokemon'
import TypeBadge from '@/components/common/TypeBadge'
import Card from '@/components/common/Card'
import BattleSkillsTab from '@/components/battle/BattleSkillsTab'
import BattleItemsTab from '@/components/battle/BattleItemsTab'
import BattleResultSection from '@/components/battle/BattleResultSection'
import BattleSwitchModal from '@/components/battle/BattleSwitchModal'

const TABS = [
  { key: 'skills' as const, label: '技能' },
  { key: 'items' as const, label: '道具' },
  { key: 'capture' as const, label: '捕捉' },
  { key: 'flee' as const, label: '逃跑' },
]

export default defineComponent({
  name: 'BattlePage',
  setup() {
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

    const items = ref<any[]>([])
    const itemsLoading = ref(false)

    const captureItems = computed(() => {
      return items.value.filter((i: any) => i.type === 2)
    })

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

    return () => {
      if (!scene.value) {
        return (
          <div class="text-center py-12 text-gray-400">
            <p>没有活跃的战斗</p>
          </div>
        )
      }

      return (
        <div class="flex flex-col h-full max-w-2xl mx-auto">
          {error.value && (
            <div class="bg-red-50 border border-red-200 text-red-600 rounded-lg px-4 py-2 mb-3 text-sm">
              {error.value}
              <button class="ml-2 underline cursor-pointer" onClick={() => error.value = ''}>关闭</button>
            </div>
          )}

          {isActive.value ? (
            <>
              {/* Opponent Section */}
              <div class="flex items-center gap-4 p-4 bg-gray-50 rounded-xl mb-3">
                {opponent.value?.pmno ? (
                  <img
                    src={battleSpriteUrl(opponent.value.pmno)}
                    alt={opponent.value.name}
                    class="w-20 h-20 object-contain shrink-0"
                  />
                ) : (
                  <div class="w-20 h-20 shrink-0 bg-gray-200 rounded-xl flex items-center justify-center text-gray-400 text-xs">
                    未知
                  </div>
                )}
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="text-base font-bold truncate">{opponent.value?.name ?? '???'}</span>
                    <span class="text-xs text-gray-400 shrink-0">Lv.{opponent.value?.level ?? '?'}</span>
                    {opponent.value?.sex !== undefined && opponent.value?.sex > 0 && (
                      <span class="text-xs shrink-0">
                        {opponent.value.sex === 1 ? '♂' : '♀'}
                      </span>
                    )}
                  </div>
                  <div class="flex gap-1 mb-2">
                    {opponent.value?.type1 && <TypeBadge type={opponent.value.type1} />}
                    {opponent.value?.type2 && <TypeBadge type={opponent.value.type2} />}
                  </div>
                  <div class="flex items-center gap-2">
                    <div class="flex-1 h-3 bg-gray-300 rounded-full overflow-hidden">
                      <div
                        class={['h-full rounded-full transition-all duration-500', hpClass(opponent.value?.hp ?? 0, opponent.value?.maxHp ?? opponent.value?.max_hp ?? 1)]}
                        style={{ width: `${opponentHpPct.value}%` }}
                      />
                    </div>
                    <span class="text-xs text-gray-500 shrink-0">
                      {opponent.value?.hp ?? 0}/{opponent.value?.maxHp ?? opponent.value?.max_hp ?? 0}
                    </span>
                  </div>
                </div>
              </div>

              {/* Turn / Map Info */}
              <div class="flex items-center justify-between mb-3 text-xs text-gray-400">
                <span>{scene.value.map_name ?? scene.value.map ?? '未知'}</span>
                <span>回合 {turn.value}</span>
              </div>

              {/* My Pokemon Section */}
              <div class="flex items-center gap-4 p-4 bg-green-50 rounded-xl mb-4">
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="text-base font-bold truncate">{myPokemon.value?.name ?? '???'}</span>
                    <span class="text-xs text-gray-400 shrink-0">Lv.{myPokemon.value?.level ?? '?'}</span>
                    {myPokemon.value?.sex !== undefined && myPokemon.value?.sex > 0 && (
                      <span class="text-xs shrink-0">
                        {myPokemon.value.sex === 1 ? '♂' : '♀'}
                      </span>
                    )}
                  </div>
                  <div class="flex gap-1 mb-2">
                    {myPokemon.value?.type1 && <TypeBadge type={myPokemon.value.type1} />}
                    {myPokemon.value?.type2 && <TypeBadge type={myPokemon.value.type2} />}
                  </div>
                  <div class="flex items-center gap-2">
                    <div class="flex-1 h-3 bg-gray-300 rounded-full overflow-hidden">
                      <div
                        class={['h-full rounded-full transition-all duration-500', hpClass(myPokemon.value?.hp ?? 0, myPokemon.value?.maxHp ?? myPokemon.value?.max_hp ?? 1)]}
                        style={{ width: `${myHpPct.value}%` }}
                      />
                    </div>
                    <span class="text-xs text-gray-500 shrink-0">
                      {myPokemon.value?.hp ?? 0}/{myPokemon.value?.maxHp ?? myPokemon.value?.max_hp ?? 0}
                    </span>
                  </div>
                  {myPokemon.value?.exp !== undefined && myPokemon.value?.maxExp !== undefined && (
                    <div class="mt-1">
                      <div class="flex items-center gap-2">
                        <div class="flex-1 h-1.5 bg-gray-300 rounded-full overflow-hidden">
                          <div
                            class="h-full bg-blue-400 rounded-full transition-all"
                            style={{ width: `${Math.max(0, Math.min(100, ((myPokemon.value.exp) / (myPokemon.value.maxExp || 1)) * 100))}%` }}
                          />
                        </div>
                        <span class="text-xs text-blue-400 shrink-0">EXP</span>
                      </div>
                    </div>
                  )}
                </div>
                {myPokemon.value?.pmno ? (
                  <img
                    src={battleSpriteUrl(myPokemon.value.pmno)}
                    alt={myPokemon.value.name}
                    class="w-20 h-20 object-contain shrink-0 scale-x-[-1]"
                  />
                ) : (
                  <div class="w-20 h-20 shrink-0 bg-gray-200 rounded-xl flex items-center justify-center text-gray-400 text-xs">
                    未知
                  </div>
                )}
              </div>

              {/* Battle Actions Section */}
              <Card class="flex-1 flex flex-col min-h-0">
                {/* Switch Pokemon Quick Button */}
                <div class="flex items-center justify-between mb-3">
                  <span class="text-sm font-bold">战斗操作</span>
                  <button
                    class="text-xs text-blue-500 hover:text-blue-700 cursor-pointer"
                    onClick={() => openSwitch('switch')}
                  >
                    更换宝可梦
                  </button>
                </div>

                {/* Tab Buttons */}
                <div class="flex gap-1 mb-3 border-b border-border">
                  {TABS.map(tab => (
                    <button
                      key={tab.key}
                      class={[
                        'flex-1 py-2 text-sm font-medium transition-colors cursor-pointer',
                        activeTab.value === tab.key
                          ? 'text-primary border-b-2 border-primary -mb-0.5'
                          : 'text-gray-500 hover:text-gray-700',
                      ]}
                      onClick={() => activeTab.value = tab.key}
                    >
                      {tab.label}
                    </button>
                  ))}
                </div>

                {/* Tab Content */}
                <div class="flex-1 overflow-y-auto">
                  {activeTab.value === 'skills' && (
                    <BattleSkillsTab
                      waiting={waiting.value}
                      onAction={onAction}
                    />
                  )}

                  {activeTab.value === 'items' && (
                    <BattleItemsTab
                      waiting={waiting.value}
                      onAction={onAction}
                    />
                  )}

                  {activeTab.value === 'capture' && (
                    <div>
                      {itemsLoading.value ? (
                        <div class="text-center py-8 text-gray-400 text-sm">
                          加载精灵球...
                        </div>
                      ) : !captureItems.value.length ? (
                        <div class="text-center py-8 text-gray-400 text-sm">
                          没有可用的精灵球
                        </div>
                      ) : (
                        <div class="grid grid-cols-2 gap-2">
                          {captureItems.value.map((ball: any) => (
                            <button
                              key={ball.id}
                              class={['flex flex-col items-center gap-1 p-3 rounded-lg border border-border bg-white hover:bg-gray-50 transition-colors cursor-pointer text-center', waiting.value ? 'opacity-50 cursor-not-allowed' : '']}
                              disabled={waiting.value}
                              onClick={() => onCapture(ball.id)}
                            >
                              <span class="text-sm font-bold truncate w-full">{ball.name}</span>
                              <span class="text-xs text-gray-400">x{ball.count ?? 0}</span>
                            </button>
                          ))}
                        </div>
                      )}
                    </div>
                  )}

                  {activeTab.value === 'flee' && (
                    <div>
                      <div class="text-center py-4">
                        <p class="text-sm text-gray-500 mb-4">确定要逃跑吗？</p>
                        <button
                          class={['px-6 py-2 rounded-lg text-sm font-bold text-white transition-colors cursor-pointer', waiting.value ? 'bg-gray-400 cursor-not-allowed' : 'bg-red-500 hover:bg-red-600']}
                          disabled={waiting.value}
                          onClick={onFlee}
                        >
                          {waiting.value ? '逃跑中...' : '逃跑'}
                        </button>
                      </div>
                    </div>
                  )}
                </div>
              </Card>

              {/* Battle Log */}
              {logs.value.length > 0 && (
                <Card class="mt-3">
                  <div class="text-xs font-bold mb-2 text-gray-500">战斗日志</div>
                  <div class="space-y-1 max-h-32 overflow-y-auto">
                    {logs.value.map((log: any, i: number) => (
                      <div key={i} class="text-xs text-gray-600 py-0.5">
                        {typeof log === 'string' ? log : (log.text ?? log.message ?? JSON.stringify(log))}
                      </div>
                    ))}
                  </div>
                </Card>
              )}

              {/* Switch Modal */}
              <BattleSwitchModal
                open={switchModalOpen.value}
                reason={switchReason.value}
                onClose={() => switchModalOpen.value = false}
                onAction={onAction}
              />
            </>
          ) : (
            <div>
              <BattleResultSection />
            </div>
          )}
        </div>
      )
    }
  },
})
