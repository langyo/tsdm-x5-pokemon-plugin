import { defineComponent, ref, onMounted, Teleport } from 'vue'
import { useAdminStore } from '@/stores/admin'
import Modal from '@/components/common/Modal'
import { api } from '@/api/client'
import { getTypeColor } from '@/utils/pokemon'
import './UserDataPage.scss'

export default defineComponent({
  name: 'UserDataPage',
  setup() {
    const store = useAdminStore('user')
    const { items, loading, error, total, hasMore, reload, loadMore, applyFilters } = store

    const filterOpen = ref(false)
    const filterForm = ref({ uid: '', username: '' })

    const detailOpen = ref(false)
    const detailUser = ref<any>(null)
    const detailTab = ref<'pokemon' | 'items'>('pokemon')

    const userPokemon = ref<any[]>([])
    const userItems = ref<any[]>([])
    const detailLoading = ref(false)
    const detailError = ref<string | null>(null)

    const grantPkmOpen = ref(false)
    const grantPkmSearch = ref('')
    const grantPkmResults = ref<any[]>([])
    const grantPkmForm = ref({ speciesId: 0, name: '', level: 1, hp: 0 })
    const grantPkmSaving = ref(false)
    const grantPkmError = ref<string | null>(null)

    const grantItemOpen = ref(false)
    const grantItemSearch = ref('')
    const grantItemResults = ref<any[]>([])
    const grantItemForm = ref({ itemId: 0, quantity: 1 })
    const grantItemSaving = ref(false)
    const grantItemError = ref<string | null>(null)

    const editPkmOpen = ref(false)
    const editPkmForm = ref<Record<string, any>>({})
    const editPkmSaving = ref(false)
    const editPkmError = ref<string | null>(null)

    const editItemOpen = ref(false)
    const editItemForm = ref<Record<string, any>>({})
    const editItemSaving = ref(false)
    const editItemError = ref<string | null>(null)

    const deleteConfirm = ref<{ type: 'pokemon' | 'item'; data: any } | null>(null)

    function onTableScroll(e: Event) {
      const el = e.target as HTMLElement
      if (el.scrollHeight - el.scrollTop - el.clientHeight < 80) {
        loadMore()
      }
    }

    function doApplyFilter() {
      const f: Record<string, any> = {}
      if (filterForm.value.uid) f.uid = filterForm.value.uid
      if (filterForm.value.username) f.username = filterForm.value.username
      applyFilters(f)
      filterOpen.value = false
    }

    function resetFilter() {
      applyFilters({})
      filterOpen.value = false
      filterForm.value = { uid: '', username: '' }
    }

    async function openDetail(row: any) {
      detailUser.value = row
      detailTab.value = 'pokemon'
      detailOpen.value = true
      await loadUserData()
    }

    function closeDetail() {
      detailOpen.value = false
      detailUser.value = null
    }

    async function loadUserData() {
      if (!detailUser.value) return
      detailLoading.value = true
      detailError.value = null
      try {
        const uid = detailUser.value.uid ?? detailUser.value.id
        const [pkmRes, itemRes] = await Promise.all([
          api.admin('list::user_pokemon', { uid: String(uid) }),
          api.admin('list::user_item', { uid: String(uid) }),
        ])
        userPokemon.value = pkmRes.data ?? pkmRes.list ?? pkmRes.items ?? []
        userItems.value = itemRes.data ?? itemRes.list ?? itemRes.items ?? []
      } catch (e: any) {
        detailError.value = e.message
      } finally {
        detailLoading.value = false
      }
    }

    function openGrantPkm() {
      grantPkmForm.value = { speciesId: 0, name: '', level: 1, hp: 0 }
      grantPkmSearch.value = ''
      grantPkmResults.value = []
      grantPkmError.value = null
      grantPkmOpen.value = true
    }

    function closeGrantPkm() {
      grantPkmOpen.value = false
    }

    let grantPkmTimer: ReturnType<typeof setTimeout> | null = null
    async function searchPokemonForGrant() {
      if (grantPkmTimer) clearTimeout(grantPkmTimer)
      grantPkmTimer = setTimeout(async () => {
        const q = grantPkmSearch.value.trim()
        if (!q) {
          grantPkmResults.value = []
          return
        }
        try {
          const res = await api.admin('filter::pokemon', { name: q })
          grantPkmResults.value = res.data ?? res.list ?? res.items ?? []
        } catch { /* ignore */ }
      }, 300)
    }

    function selectPokemonForGrant(pkm: any) {
      grantPkmForm.value.speciesId = pkm.id
      grantPkmSearch.value = pkm.name
      grantPkmResults.value = []
    }

    async function submitGrantPkm() {
      if (!grantPkmForm.value.speciesId) {
        grantPkmError.value = '请选择宠物'
        return
      }
      grantPkmSaving.value = true
      grantPkmError.value = null
      try {
        await api.admin('insert::user_pokemon', {
          uid: String(detailUser.value.uid ?? detailUser.value.id),
          data: JSON.stringify(grantPkmForm.value),
        })
        grantPkmOpen.value = false
        await loadUserData()
      } catch (e: any) {
        grantPkmError.value = e.message
      } finally {
        grantPkmSaving.value = false
      }
    }

    function openGrantItem() {
      grantItemForm.value = { itemId: 0, quantity: 1 }
      grantItemSearch.value = ''
      grantItemResults.value = []
      grantItemError.value = null
      grantItemOpen.value = true
    }

    function closeGrantItem() {
      grantItemOpen.value = false
    }

    let grantItemTimer: ReturnType<typeof setTimeout> | null = null
    async function searchItemForGrant() {
      if (grantItemTimer) clearTimeout(grantItemTimer)
      grantItemTimer = setTimeout(async () => {
        const q = grantItemSearch.value.trim()
        if (!q) {
          grantItemResults.value = []
          return
        }
        try {
          const res = await api.admin('filter::item', { name: q })
          grantItemResults.value = res.data ?? res.list ?? res.items ?? []
        } catch { /* ignore */ }
      }, 300)
    }

    function selectItemForGrant(item: any) {
      grantItemForm.value.itemId = item.id
      grantItemSearch.value = item.name
      grantItemResults.value = []
    }

    async function submitGrantItem() {
      if (!grantItemForm.value.itemId) {
        grantItemError.value = '请选择道具'
        return
      }
      grantItemSaving.value = true
      grantItemError.value = null
      try {
        await api.admin('insert::user_item', {
          uid: String(detailUser.value.uid ?? detailUser.value.id),
          data: JSON.stringify(grantItemForm.value),
        })
        grantItemOpen.value = false
        await loadUserData()
      } catch (e: any) {
        grantItemError.value = e.message
      } finally {
        grantItemSaving.value = false
      }
    }

    function openEditPkm(row: any) {
      editPkmForm.value = {
        id: row.id ?? 0,
        species: row.species ?? row.pid ?? 0,
        name: row.name ?? '',
        level: Number(row.level ?? 1),
        hp: Number(row.hp ?? 0),
        atk: Number(row.atk ?? 0),
        def: Number(row.def ?? 0),
        spatk: Number(row.spatk ?? 0),
        spdef: Number(row.spdef ?? 0),
        sd: Number(row.sd ?? 0),
        experience: Number(row.experience ?? row.exp ?? 0),
      }
      editPkmError.value = null
      editPkmOpen.value = true
    }

    function closeEditPkm() {
      editPkmOpen.value = false
    }

    async function submitEditPkm() {
      editPkmSaving.value = true
      editPkmError.value = null
      try {
        await api.admin('set::user_pokemon', {
          uid: String(detailUser.value.uid ?? detailUser.value.id),
          data: JSON.stringify(editPkmForm.value),
        })
        editPkmOpen.value = false
        await loadUserData()
      } catch (e: any) {
        editPkmError.value = e.message
      } finally {
        editPkmSaving.value = false
      }
    }

    function openEditItem(row: any) {
      editItemForm.value = {
        id: row.id ?? 0,
        itemId: row.itemId ?? row.item_id ?? row.pid ?? 0,
        name: row.name ?? '',
        quantity: Number(row.quantity ?? row.num ?? 1),
      }
      editItemError.value = null
      editItemOpen.value = true
    }

    function closeEditItem() {
      editItemOpen.value = false
    }

    async function submitEditItem() {
      editItemSaving.value = true
      editItemError.value = null
      try {
        await api.admin('set::user_item', {
          uid: String(detailUser.value.uid ?? detailUser.value.id),
          data: JSON.stringify(editItemForm.value),
        })
        editItemOpen.value = false
        await loadUserData()
      } catch (e: any) {
        editItemError.value = e.message
      } finally {
        editItemSaving.value = false
      }
    }

    function confirmDelete(type: 'pokemon' | 'item', data: any) {
      deleteConfirm.value = { type, data }
    }

    async function executeDelete() {
      if (!deleteConfirm.value) return
      const { type, data } = deleteConfirm.value
      try {
        if (type === 'pokemon') {
          await api.admin('delete::user_pokemon', {
            uid: String(detailUser.value.uid ?? detailUser.value.id),
            id: String(data.id),
          })
        } else {
          await api.admin('delete::user_item', {
            uid: String(detailUser.value.uid ?? detailUser.value.id),
            id: String(data.id),
          })
        }
        deleteConfirm.value = null
        await loadUserData()
      } catch (e: any) {
        alert('删除失败: ' + e.message)
      }
    }

    onMounted(() => {
      reload()
    })

    return () => (
      <div class="flex flex-col" style="height: calc(100vh - 6.5rem)">
        <div class="flex items-center justify-between mb-3">
          <h2 class="text-lg font-bold">用户数据管理</h2>
          <span class="text-xs text-gray-400">共 {total.value} 条</span>
        </div>

        {error.value && (
          <div class="card bg-red-50 border-red-200 mb-3">
            <p class="text-red-600 text-sm">{error.value}</p>
          </div>
        )}

        <div class="card flex-1 flex flex-col min-h-0 p-0 overflow-hidden">
          <div class="overflow-y-auto flex-1" onScroll={onTableScroll}>
            <table class="w-full text-sm table-fixed">
              <thead class="sticky top-0 bg-gray-50 z-10">
                <tr class="border-b border-gray-200">
                  <th class="px-2 py-2 text-left w-16">UID</th>
                  <th class="px-2 py-2 text-left w-28">用户名</th>
                  <th class="px-2 py-2 text-right w-16">胜场</th>
                  <th class="px-2 py-2 text-right w-16">败场</th>
                  <th class="px-2 py-2 text-right w-20">金币</th>
                  <th class="px-2 py-2 text-right w-20">经验</th>
                  <th class="px-2 py-2 text-center w-18">操作</th>
                </tr>
              </thead>
              <tbody>
                {loading.value && items.value.length === 0 ? (
                  <tr>
                    <td colspan="7" class="text-center text-gray-400 py-8">
                      <span class="i-svg-spinners-3-dots-bounce text-lg" />
                    </td>
                  </tr>
                ) : !loading.value && items.value.length === 0 ? (
                  <tr>
                    <td colspan="7" class="text-center text-gray-400 py-8">暂无数据</td>
                  </tr>
                ) : (
                  items.value.map((row: any) => (
                    <tr
                      key={row.uid ?? row.id}
                      class="border-b border-gray-100 hover:bg-gray-50 cursor-pointer transition-colors"
                      onClick={() => openDetail(row)}
                    >
                      <td class="px-2 py-2 font-mono text-xs">{row.uid ?? row.id}</td>
                      <td class="px-2 py-2 font-medium truncate">{row.username}</td>
                      <td class="px-2 py-2 text-right text-xs text-green-600">{row.wins ?? 0}</td>
                      <td class="px-2 py-2 text-right text-xs text-red-500">{row.losses ?? 0}</td>
                      <td class="px-2 py-2 text-right text-xs font-mono">{row.money ?? 0}</td>
                      <td class="px-2 py-2 text-right text-xs font-mono">{row.exp ?? row.experience ?? 0}</td>
                      <td class="px-2 py-2 text-center" onClick={(e: Event) => e.stopPropagation()}>
                        <button class="px-2 py-1 text-xs rounded bg-primary/10 text-primary hover:bg-primary/20 transition-colors" onClick={() => openDetail(row)}>
                          详情
                        </button>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
              {loading.value && items.value.length > 0 && (
                <tfoot>
                  <tr>
                    <td colspan="7" class="text-center text-gray-400 py-2">
                      <span class="i-svg-spinners-3-dots-bounce text-sm" />
                    </td>
                  </tr>
                </tfoot>
              )}
            </table>
          </div>
        </div>

        <div class="fixed bottom-0 left-48 right-0 bg-white border-t border-gray-200 px-4 py-2 flex items-center justify-center gap-3 z-20 shadow-lg">
          <button class="btn text-sm border border-gray-300 hover:bg-gray-100" onClick={() => reload()}>刷新</button>
          <button class={['btn text-sm border border-gray-300 hover:bg-gray-100', filterOpen.value ? 'bg-primary/10 border-primary' : ''].join(' ')} onClick={() => filterOpen.value = !filterOpen.value}>筛选</button>
        </div>

        {filterOpen.value && (
          <Teleport to="body">
            <div class="fixed inset-0 z-30">
              <div class="absolute inset-0 bg-black/30" onClick={() => filterOpen.value = false} />
              <div class="absolute bottom-14 left-48 right-0 bg-white border-t shadow-xl p-4">
                <div class="flex flex-wrap items-end gap-3">
                  <label class="flex flex-col gap-1">
                    <span class="text-xs text-gray-500">UID</span>
                    <input type="text" value={filterForm.value.uid} onInput={(e: any) => filterForm.value.uid = e.target.value} class="input w-32 text-sm" placeholder="用户UID" />
                  </label>
                  <label class="flex flex-col gap-1">
                    <span class="text-xs text-gray-500">用户名</span>
                    <input type="text" value={filterForm.value.username} onInput={(e: any) => filterForm.value.username = e.target.value} class="input w-40 text-sm" placeholder="用户名关键词" />
                  </label>
                  <button class="btn-primary text-sm" onClick={doApplyFilter}>应用</button>
                  <button class="btn text-sm border border-gray-300 hover:bg-gray-100" onClick={resetFilter}>重置</button>
                </div>
              </div>
            </div>
          </Teleport>
        )}

        <Modal open={detailOpen.value} title={`用户详情 - ${detailUser.value?.username ?? ''}`} wide onClose={closeDetail}>
          {detailError.value && (
            <div class="text-red-500 text-sm mb-3">{detailError.value}</div>
          )}

          <div class="flex gap-4 mb-4">
            <button
              class={['px-3 py-1.5 text-sm rounded transition-colors', detailTab.value === 'pokemon' ? 'bg-primary text-white' : 'bg-gray-100 hover:bg-gray-200'].join(' ')}
              onClick={() => detailTab.value = 'pokemon'}
            >
              宠物 ({userPokemon.value.length})
            </button>
            <button
              class={['px-3 py-1.5 text-sm rounded transition-colors', detailTab.value === 'items' ? 'bg-primary text-white' : 'bg-gray-100 hover:bg-gray-200'].join(' ')}
              onClick={() => detailTab.value = 'items'}
            >
              道具 ({userItems.value.length})
            </button>
          </div>

          {detailLoading.value ? (
            <div class="text-center text-gray-400 py-4">
              <span class="i-svg-spinners-3-dots-bounce text-lg" />
            </div>
          ) : detailTab.value === 'pokemon' ? (
            <>
              <div class="flex items-center justify-between mb-3">
                <h4 class="text-sm font-bold">宠物列表</h4>
                <button class="btn-primary text-xs" onClick={openGrantPkm}>+ 发放宠物</button>
              </div>
              {userPokemon.value.length === 0 ? (
                <div class="text-center text-gray-400 py-4 text-sm">暂无宠物</div>
              ) : (
                <table class="w-full text-sm">
                  <thead>
                    <tr class="border-b border-gray-200">
                      <th class="px-2 py-1.5 text-left text-xs">ID</th>
                      <th class="px-2 py-1.5 text-left text-xs">物种</th>
                      <th class="px-2 py-1.5 text-left text-xs">名称</th>
                      <th class="px-2 py-1.5 text-right text-xs">等级</th>
                      <th class="px-2 py-1.5 text-right text-xs">HP</th>
                      <th class="px-2 py-1.5 text-center text-xs w-28">操作</th>
                    </tr>
                  </thead>
                  <tbody>
                    {userPokemon.value.map((pkm: any) => (
                      <tr key={pkm.id} class="border-b border-gray-100">
                        <td class="px-2 py-1.5 font-mono text-xs">{pkm.id}</td>
                        <td class="px-2 py-1.5 text-xs">{pkm.species ?? pkm.pid}</td>
                        <td class="px-2 py-1.5 text-xs font-medium">{pkm.name}</td>
                        <td class="px-2 py-1.5 text-xs text-right">Lv.{pkm.level}</td>
                        <td class="px-2 py-1.5 text-xs text-right">{pkm.hp}/{pkm.maxhp ?? pkm.max_hp}</td>
                        <td class="px-2 py-1.5 text-center">
                          <div class="flex items-center justify-center gap-1">
                            <button class="px-1.5 py-0.5 text-[10px] rounded bg-primary/10 text-primary hover:bg-primary/20" onClick={() => openEditPkm(pkm)}>编辑</button>
                            <button class="px-1.5 py-0.5 text-[10px] rounded bg-red-50 text-red-500 hover:bg-red-100" onClick={() => confirmDelete('pokemon', pkm)}>删除</button>
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              )}
            </>
          ) : (
            <>
              <div class="flex items-center justify-between mb-3">
                <h4 class="text-sm font-bold">道具列表</h4>
                <button class="btn-primary text-xs" onClick={openGrantItem}>+ 发放道具</button>
              </div>
              {userItems.value.length === 0 ? (
                <div class="text-center text-gray-400 py-4 text-sm">暂无道具</div>
              ) : (
                <table class="w-full text-sm">
                  <thead>
                    <tr class="border-b border-gray-200">
                      <th class="px-2 py-1.5 text-left text-xs">ID</th>
                      <th class="px-2 py-1.5 text-left text-xs">名称</th>
                      <th class="px-2 py-1.5 text-right text-xs">数量</th>
                      <th class="px-2 py-1.5 text-center text-xs w-28">操作</th>
                    </tr>
                  </thead>
                  <tbody>
                    {userItems.value.map((item: any) => (
                      <tr key={item.id} class="border-b border-gray-100">
                        <td class="px-2 py-1.5 font-mono text-xs">{item.id}</td>
                        <td class="px-2 py-1.5 text-xs font-medium">{item.name}</td>
                        <td class="px-2 py-1.5 text-xs text-right">x{item.quantity ?? item.num}</td>
                        <td class="px-2 py-1.5 text-center">
                          <div class="flex items-center justify-center gap-1">
                            <button class="px-1.5 py-0.5 text-[10px] rounded bg-primary/10 text-primary hover:bg-primary/20" onClick={() => openEditItem(item)}>编辑</button>
                            <button class="px-1.5 py-0.5 text-[10px] rounded bg-red-50 text-red-500 hover:bg-red-100" onClick={() => confirmDelete('item', item)}>删除</button>
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              )}
            </>
          )}
        </Modal>

        <Modal open={grantPkmOpen.value} title="发放宠物" onClose={closeGrantPkm}>
          <div class="space-y-3">
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">搜索宠物物种</span>
              <div class="relative">
                <input
                  type="text"
                  value={grantPkmSearch.value}
                  onInput={(e: any) => { grantPkmSearch.value = e.target.value; searchPokemonForGrant() }}
                  class="input text-sm w-full"
                  placeholder="输入名称搜索..."
                />
                {grantPkmResults.value.length > 0 && (
                  <div class="absolute top-full left-0 right-0 bg-white border border-gray-200 rounded-lg shadow-lg max-h-40 overflow-y-auto z-50">
                    {grantPkmResults.value.map((r: any) => (
                      <div
                        key={r.id}
                        class="px-3 py-1.5 text-sm hover:bg-primary/10 cursor-pointer"
                        onClick={() => selectPokemonForGrant(r)}
                      >
                        <span class="font-mono text-xs text-gray-400 mr-2">#{r.id}</span>
                        {r.name}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">名称</span>
              <input type="text" value={grantPkmForm.value.name} onInput={(e: any) => grantPkmForm.value.name = e.target.value} class="input text-sm" placeholder="精灵昵称" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">等级</span>
              <input type="number" min="1" max="100" value={grantPkmForm.value.level} onInput={(e: any) => grantPkmForm.value.level = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">HP</span>
              <input type="number" min="0" value={grantPkmForm.value.hp} onInput={(e: any) => grantPkmForm.value.hp = Number(e.target.value)} class="input text-sm" />
            </label>
          </div>

          {grantPkmError.value && (
            <div class="mt-3 text-red-500 text-sm">{grantPkmError.value}</div>
          )}

          <div class="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-gray-100">
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" onClick={closeGrantPkm}>取消</button>
            <button class="btn-primary text-sm" disabled={grantPkmSaving.value} onClick={submitGrantPkm}>
              {grantPkmSaving.value ? (
                <span class="i-svg-spinners-3-dots-bounce text-white" />
              ) : (
                <span>发放</span>
              )}
            </button>
          </div>
        </Modal>

        <Modal open={grantItemOpen.value} title="发放道具" onClose={closeGrantItem}>
          <div class="space-y-3">
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">搜索道具</span>
              <div class="relative">
                <input
                  type="text"
                  value={grantItemSearch.value}
                  onInput={(e: any) => { grantItemSearch.value = e.target.value; searchItemForGrant() }}
                  class="input text-sm w-full"
                  placeholder="输入名称搜索..."
                />
                {grantItemResults.value.length > 0 && (
                  <div class="absolute top-full left-0 right-0 bg-white border border-gray-200 rounded-lg shadow-lg max-h-40 overflow-y-auto z-50">
                    {grantItemResults.value.map((r: any) => (
                      <div
                        key={r.id}
                        class="px-3 py-1.5 text-sm hover:bg-primary/10 cursor-pointer"
                        onClick={() => selectItemForGrant(r)}
                      >
                        <span class="font-mono text-xs text-gray-400 mr-2">#{r.id}</span>
                        {r.name}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">数量</span>
              <input type="number" min="1" value={grantItemForm.value.quantity} onInput={(e: any) => grantItemForm.value.quantity = Number(e.target.value)} class="input text-sm" />
            </label>
          </div>

          {grantItemError.value && (
            <div class="mt-3 text-red-500 text-sm">{grantItemError.value}</div>
          )}

          <div class="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-gray-100">
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" onClick={closeGrantItem}>取消</button>
            <button class="btn-primary text-sm" disabled={grantItemSaving.value} onClick={submitGrantItem}>
              {grantItemSaving.value ? (
                <span class="i-svg-spinners-3-dots-bounce text-white" />
              ) : (
                <span>发放</span>
              )}
            </button>
          </div>
        </Modal>

        <Modal open={editPkmOpen.value} title="编辑宠物" onClose={closeEditPkm}>
          <div class="grid grid-cols-2 gap-3">
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">物种ID</span>
              <input type="number" value={editPkmForm.value.species} onInput={(e: any) => editPkmForm.value.species = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">名称</span>
              <input type="text" value={editPkmForm.value.name} onInput={(e: any) => editPkmForm.value.name = e.target.value} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">等级</span>
              <input type="number" min="1" max="100" value={editPkmForm.value.level} onInput={(e: any) => editPkmForm.value.level = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">HP</span>
              <input type="number" value={editPkmForm.value.hp} onInput={(e: any) => editPkmForm.value.hp = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">ATK</span>
              <input type="number" value={editPkmForm.value.atk} onInput={(e: any) => editPkmForm.value.atk = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">DEF</span>
              <input type="number" value={editPkmForm.value.def} onInput={(e: any) => editPkmForm.value.def = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">SPATK</span>
              <input type="number" value={editPkmForm.value.spatk} onInput={(e: any) => editPkmForm.value.spatk = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">SPDEF</span>
              <input type="number" value={editPkmForm.value.spdef} onInput={(e: any) => editPkmForm.value.spdef = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">SPEED</span>
              <input type="number" value={editPkmForm.value.sd} onInput={(e: any) => editPkmForm.value.sd = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">经验</span>
              <input type="number" value={editPkmForm.value.experience} onInput={(e: any) => editPkmForm.value.experience = Number(e.target.value)} class="input text-sm" />
            </label>
          </div>

          {editPkmError.value && (
            <div class="mt-3 text-red-500 text-sm">{editPkmError.value}</div>
          )}

          <div class="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-gray-100">
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" onClick={closeEditPkm}>取消</button>
            <button class="btn-primary text-sm" disabled={editPkmSaving.value} onClick={submitEditPkm}>
              {editPkmSaving.value ? (
                <span class="i-svg-spinners-3-dots-bounce text-white" />
              ) : (
                <span>保存</span>
              )}
            </button>
          </div>
        </Modal>

        <Modal open={editItemOpen.value} title="编辑道具" onClose={closeEditItem}>
          <div class="space-y-3">
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">道具ID</span>
              <input type="number" value={editItemForm.value.itemId} onInput={(e: any) => editItemForm.value.itemId = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">名称</span>
              <input type="text" value={editItemForm.value.name} onInput={(e: any) => editItemForm.value.name = e.target.value} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">数量</span>
              <input type="number" min="0" value={editItemForm.value.quantity} onInput={(e: any) => editItemForm.value.quantity = Number(e.target.value)} class="input text-sm" />
            </label>
          </div>

          {editItemError.value && (
            <div class="mt-3 text-red-500 text-sm">{editItemError.value}</div>
          )}

          <div class="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-gray-100">
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" onClick={closeEditItem}>取消</button>
            <button class="btn-primary text-sm" disabled={editItemSaving.value} onClick={submitEditItem}>
              {editItemSaving.value ? (
                <span class="i-svg-spinners-3-dots-bounce text-white" />
              ) : (
                <span>保存</span>
              )}
            </button>
          </div>
        </Modal>

        {deleteConfirm.value && (
          <Teleport to="body">
            <div class="fixed inset-0 z-50 flex items-center justify-center">
              <div class="absolute inset-0 bg-black/50" onClick={() => deleteConfirm.value = null} />
              <div class="relative bg-white rounded-xl shadow-lg border border-border p-6 w-full max-w-sm z-10 mx-4">
                <h3 class="text-base font-bold mb-2">确认删除</h3>
                <p class="text-sm text-gray-600 mb-1">
                  确认删除此条记录？
                </p>
                {deleteConfirm.value.type === 'pokemon' ? (
                  <p class="text-sm text-gray-500 mb-4">
                    宠物: {deleteConfirm.value.data.name} (ID: {deleteConfirm.value.data.id})
                  </p>
                ) : (
                  <p class="text-sm text-gray-500 mb-4">
                    道具: {deleteConfirm.value.data.name} (ID: {deleteConfirm.value.data.id})
                  </p>
                )}
                <div class="flex items-center justify-end gap-3">
                  <button class="btn text-sm border border-gray-300 hover:bg-gray-100" onClick={() => deleteConfirm.value = null}>取消</button>
                  <button class="px-4 py-1.5 text-sm rounded bg-red-500 text-white hover:bg-red-600" onClick={executeDelete}>删除</button>
                </div>
              </div>
            </div>
          </Teleport>
        )}
      </div>
    )
  },
})
