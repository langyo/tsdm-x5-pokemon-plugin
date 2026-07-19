import { defineComponent, ref, onMounted } from 'vue'
import AppLayout from '@/components/layout/AppLayout'
import Card from '@/components/common/Card'
import { useUserStore, usePokemonStore } from '@/stores'
import { api } from '@/api/client'
import { spriteUrl } from '@/utils/pokemon'

export default defineComponent({
  name: 'ShopPage',
  setup() {
    const userStore = useUserStore()
    const pokemonStore = usePokemonStore()

    const tabs = [
      { key: '1', label: '回复药', type: 1 },
      { key: '2', label: '精灵球', type: 2 },
      { key: '3', label: '进化石', type: 3 },
      { key: '4', label: '强化道具', type: 4 },
      { key: '5', label: '装备道具', type: 5 },
      { key: 'pets', label: '宠物', type: null },
    ]

    const activeTab = ref('1')
    const items = ref<any[]>([])
    const loading = ref(false)
    const error = ref('')
    const page = ref(1)
    const totalPages = ref(1)
    const buyingId = ref<number | null>(null)

    const currentTab = () => tabs.find((t) => t.key === activeTab.value)!

    async function fetchItems() {
      loading.value = true
      error.value = ''
      try {
        const tab = currentTab()
        if (tab.key === 'pets') {
          const data: any = await api.get('shop', { action: 'pets', page: page.value })
          items.value = data.pets ?? data.data ?? []
          totalPages.value = data.total_pages ?? data.totalPages ?? 1
        } else {
          const data: any = await api.get('shop', { action: 'list', type: tab.type, page: page.value })
          items.value = data.items ?? data.data ?? []
          totalPages.value = data.total_pages ?? data.totalPages ?? 1
        }
      } catch (e: any) {
        error.value = e.message || '加载失败'
        items.value = []
      } finally {
        loading.value = false
      }
    }

    function switchTab(key: string) {
      activeTab.value = key
      page.value = 1
      fetchItems()
    }

    async function buy(item: any) {
      buyingId.value = item.id ?? item.pet_id
      try {
        const tab = currentTab()
        if (tab.key === 'pets') {
          await api.post('shop', { action: 'buy_pet', pet_id: item.id ?? item.pet_id })
        } else {
          await api.post('shop', { action: 'buy', item_id: item.id })
        }
        await Promise.all([userStore.fetchProfile(), pokemonStore.fetchList()])
      } catch (e: any) {
        alert(e.message || '购买失败')
      } finally {
        buyingId.value = null
      }
    }

    function prevPage() {
      if (page.value > 1) {
        page.value--
        fetchItems()
      }
    }
    function nextPage() {
      if (page.value < totalPages.value) {
        page.value++
        fetchItems()
      }
    }

    onMounted(() => {
      fetchItems()
    })

    return () => (
      <AppLayout>
        <div class="mb-4 flex items-center gap-2">
          <span class="text-sm text-gray-500">余额:</span>
          <span class="font-bold text-primary">{userStore.money}</span>
        </div>

        <div class="flex gap-2 mb-4 flex-wrap">
          {tabs.map((tab) => (
            <button
              key={tab.key}
              class={`px-4 py-1.5 rounded-full text-sm transition-colors ${activeTab.value === tab.key ? 'bg-primary text-white' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}`}
              onClick={() => switchTab(tab.key)}
            >
              {tab.label}
            </button>
          ))}
        </div>

        {loading.value ? (
          <div class="text-center py-12 text-gray-400">加载中...</div>
        ) : error.value ? (
          <div class="card text-center py-12 text-red-500">{error.value}</div>
        ) : !items.value.length ? (
          <div class="card text-center py-12 text-gray-400">暂无物品</div>
        ) : (
          <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
            {items.value.map((item: any) => {
              const id = item.id ?? item.pet_id ?? item.pmno
              const isPets = currentTab().key === 'pets'
              const isBuying = buyingId.value === id
              return (
                <Card key={id} class="flex flex-col">
                  <div class="flex items-center gap-3 mb-2">
                    {isPets ? (
                      <img
                        src={spriteUrl(item.pmno ?? item.image ?? 0)}
                        alt={item.name}
                        class="w-12 h-12 object-contain"
                      />
                    ) : item.icon ? (
                      <div class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center text-lg">
                        {item.icon}
                      </div>
                    ) : (
                      <div class="w-10 h-10 rounded-lg bg-gray-100 flex items-center justify-center">
                        <span class="text-xs text-gray-400">道具</span>
                      </div>
                    )}
                    <div class="flex-1 min-w-0">
                      <div class="font-bold text-sm truncate">{item.name}</div>
                      <div class="text-primary font-bold text-sm">{item.price}</div>
                    </div>
                  </div>
                  {item.description && <div class="text-xs text-gray-400 mb-3 line-clamp-2">{item.description}</div>}
                  <button
                    class={`mt-auto w-full py-1.5 rounded-lg text-sm font-bold text-white transition-colors ${isBuying ? 'bg-gray-400 cursor-not-allowed' : 'bg-primary hover:bg-primary/80'}`}
                    disabled={isBuying}
                    onClick={() => buy(item)}
                  >
                    {isBuying ? '购买中...' : '购买'}
                  </button>
                </Card>
              )
            })}
          </div>
        )}

        {totalPages.value > 1 && !loading.value && !error.value && (
          <div class="flex items-center justify-center gap-4 mt-6">
            <button
              class={`px-3 py-1 rounded text-sm ${page.value <= 1 ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}`}
              disabled={page.value <= 1}
              onClick={prevPage}
            >
              上一页
            </button>
            <span class="text-sm text-gray-500">{page.value} / {totalPages.value}</span>
            <button
              class={`px-3 py-1 rounded text-sm ${page.value >= totalPages.value ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}`}
              disabled={page.value >= totalPages.value}
              onClick={nextPage}
            >
              下一页
            </button>
          </div>
        )}
      </AppLayout>
    )
  }
})
