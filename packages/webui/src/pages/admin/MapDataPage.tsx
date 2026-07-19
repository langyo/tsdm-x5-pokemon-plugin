import { defineComponent, ref, onMounted, Teleport } from 'vue'
import { useAdminStore } from '@/stores/admin'
import Modal from '@/components/common/Modal'
import './MapDataPage.scss'

export default defineComponent({
  name: 'MapDataPage',
  setup() {
    const store = useAdminStore('map')
    const { items, loading, error, total, hasMore, reload, loadMore, save: storeSave, remove: storeRemove, applyFilters } = store

    const editorOpen = ref(false)
    const editorIsCreate = ref(false)
    const form = ref<Record<string, any>>({})
    const formError = ref<string | null>(null)
    const saving = ref(false)
    const filterOpen = ref(false)
    const filterForm = ref({ name: '', region: '', minlevel: '', maxlevel: '' })
    const wildPokemonInput = ref('')

    function onTableScroll(e: Event) {
      const el = e.target as HTMLElement
      if (el.scrollHeight - el.scrollTop - el.clientHeight < 80) {
        loadMore()
      }
    }

    function openCreate() {
      editorIsCreate.value = true
      form.value = {
        name: '', kg: 0, minlevel: 1, maxlevel: 100, region: '',
        pos_x: 0, pos_y: 0, wild: [], expn: '{}',
      }
      formError.value = null
      editorOpen.value = true
    }

    function openEdit(row: any) {
      editorIsCreate.value = false
      form.value = {
        id: row.id,
        name: row.name ?? '',
        kg: Number(row.kg ?? 0),
        minlevel: Number(row.minlevel ?? 1),
        maxlevel: Number(row.maxlevel ?? 100),
        region: row.region ?? '',
        pos_x: Number(row.pos_x ?? 0),
        pos_y: Number(row.pos_y ?? 0),
        wild: parseWildPokemon(row.wild),
        expn: row.expn ?? '{}',
      }
      formError.value = null
      editorOpen.value = true
    }

    function closeEditor() {
      editorOpen.value = false
    }

    function parseWildPokemon(raw: any): any[] {
      if (!raw) return []
      if (Array.isArray(raw)) return raw
      if (typeof raw === 'string') {
        try { return JSON.parse(raw) } catch { return raw.split(',').map(Number).filter((n: number) => !isNaN(n)) }
      }
      return []
    }

    function addWildPokemon() {
      const id = Number(wildPokemonInput.value.trim())
      if (!id || isNaN(id)) return
      if (!Array.isArray(form.value.wild)) form.value.wild = []
      if (!form.value.wild.includes(id)) {
        form.value.wild.push(id)
      }
      wildPokemonInput.value = ''
    }

    function removeWildPokemon(id: number) {
      if (!Array.isArray(form.value.wild)) return
      form.value.wild = form.value.wild.filter((n: number) => n !== id)
    }

    async function submitForm() {
      if (!form.value.name) {
        formError.value = '名称不能为空'
        return
      }
      saving.value = true
      formError.value = null
      try {
        const data = { ...form.value }
        if (Array.isArray(data.wild)) {
          data.wild = JSON.stringify(data.wild)
        }
        await storeSave(data)
        editorOpen.value = false
      } catch (e: any) {
        formError.value = e.message
      } finally {
        saving.value = false
      }
    }

    async function deleteRow(row: any) {
      if (!confirm(`确认删除 "${row.name}" (ID: ${row.id})？`)) return
      try {
        await storeRemove(row.id)
      } catch (e: any) {
        alert('删除失败: ' + e.message)
      }
    }

    function doApplyFilter() {
      const f: Record<string, any> = {}
      if (filterForm.value.name) f.name = filterForm.value.name
      if (filterForm.value.region) f.region = filterForm.value.region
      if (filterForm.value.minlevel) f.minlevel = Number(filterForm.value.minlevel)
      if (filterForm.value.maxlevel) f.maxlevel = Number(filterForm.value.maxlevel)
      applyFilters(f)
      filterOpen.value = false
    }

    function resetFilter() {
      applyFilters({})
      filterOpen.value = false
      filterForm.value = { name: '', region: '', minlevel: '', maxlevel: '' }
    }

    onMounted(() => {
      reload()
    })

    return () => (
      <div class="flex flex-col" style="height: calc(100vh - 6.5rem)">
        <div class="flex items-center justify-between mb-3">
          <h2 class="text-lg font-bold">地图设定管理</h2>
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
                  <th class="px-2 py-2 text-left w-10">ID</th>
                  <th class="px-2 py-2 text-left w-24">名称</th>
                  <th class="px-2 py-2 text-center w-12">开放</th>
                  <th class="px-2 py-2 text-center w-20">等级范围</th>
                  <th class="px-2 py-2 text-left w-16">区域</th>
                  <th class="px-2 py-2 text-center w-18">操作</th>
                </tr>
              </thead>
              <tbody>
                {loading.value && items.value.length === 0 ? (
                  <tr>
                    <td colspan="6" class="text-center text-gray-400 py-8">
                      <span class="i-svg-spinners-3-dots-bounce text-lg" />
                    </td>
                  </tr>
                ) : !loading.value && items.value.length === 0 ? (
                  <tr>
                    <td colspan="6" class="text-center text-gray-400 py-8">暂无数据</td>
                  </tr>
                ) : (
                  items.value.map((row: any) => (
                    <tr
                      key={row.id}
                      class="border-b border-gray-100 hover:bg-gray-50 cursor-pointer transition-colors"
                      onClick={() => openEdit(row)}
                    >
                      <td class="px-2 py-2 font-mono text-xs">{row.id}</td>
                      <td class="px-2 py-2 font-medium truncate">{row.name}</td>
                      <td class="px-2 py-2 text-center">
                        <span class={['inline-block rounded-full w-2 h-2', row.kg == 1 ? 'bg-green-500' : 'bg-gray-300'].join(' ')} />
                        <span class="ml-1 text-xs text-gray-500">{row.kg == 1 ? '是' : '否'}</span>
                      </td>
                      <td class="px-2 py-2 text-center text-xs">
                        <span class="font-mono">{row.minlevel ?? '-'} - {row.maxlevel ?? '-'}</span>
                      </td>
                      <td class="px-2 py-2 text-xs truncate">{row.region || '-'}</td>
                      <td class="px-2 py-2 text-center" onClick={(e: Event) => e.stopPropagation()}>
                        <div class="flex items-center justify-center gap-1">
                          <button class="px-2 py-1 text-xs rounded bg-primary/10 text-primary hover:bg-primary/20 transition-colors" onClick={() => openEdit(row)}>
                            编辑
                          </button>
                          <button class="px-2 py-1 text-xs rounded bg-red-50 text-red-500 hover:bg-red-100 transition-colors" onClick={() => deleteRow(row)}>
                            删除
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
              {loading.value && items.value.length > 0 && (
                <tfoot>
                  <tr>
                    <td colspan="6" class="text-center text-gray-400 py-2">
                      <span class="i-svg-spinners-3-dots-bounce text-sm" />
                    </td>
                  </tr>
                </tfoot>
              )}
            </table>
          </div>
        </div>

        <div class="fixed bottom-0 left-48 right-0 bg-white border-t border-gray-200 px-4 py-2 flex items-center justify-center gap-3 z-20 shadow-lg">
          <button class="btn-primary text-sm" onClick={openCreate}>新增</button>
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
                    <span class="text-xs text-gray-500">名称</span>
                    <input type="text" value={filterForm.value.name} onInput={(e: any) => filterForm.value.name = e.target.value} class="input w-32 text-sm" placeholder="关键词" />
                  </label>
                  <label class="flex flex-col gap-1">
                    <span class="text-xs text-gray-500">区域</span>
                    <input type="text" value={filterForm.value.region} onInput={(e: any) => filterForm.value.region = e.target.value} class="input w-24 text-sm" placeholder="region" />
                  </label>
                  <label class="flex flex-col gap-1">
                    <span class="text-xs text-gray-500">最低等级</span>
                    <input type="number" value={filterForm.value.minlevel} onInput={(e: any) => filterForm.value.minlevel = e.target.value} class="input w-20 text-sm" placeholder="0" />
                  </label>
                  <label class="flex flex-col gap-1">
                    <span class="text-xs text-gray-500">最高等级</span>
                    <input type="number" value={filterForm.value.maxlevel} onInput={(e: any) => filterForm.value.maxlevel = e.target.value} class="input w-20 text-sm" placeholder="100" />
                  </label>
                  <button class="btn-primary text-sm" onClick={doApplyFilter}>应用</button>
                  <button class="btn text-sm border border-gray-300 hover:bg-gray-100" onClick={resetFilter}>重置</button>
                </div>
              </div>
            </div>
          </Teleport>
        )}

        <Modal open={editorOpen.value} title={editorIsCreate.value ? '新增地图' : '编辑地图'} wide onClose={closeEditor}>
          <div class="grid grid-cols-3 gap-3 max-h-100 overflow-y-auto">
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">名称 *</span>
              <input type="text" value={form.value.name} onInput={(e: any) => form.value.name = e.target.value} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">区域</span>
              <input type="text" value={form.value.region} onInput={(e: any) => form.value.region = e.target.value} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">开放 (kg)</span>
              <label class="relative inline-flex items-center cursor-pointer mt-1">
                <input
                  type="checkbox"
                  checked={form.value.kg === 1}
                  onChange={(e: any) => form.value.kg = e.target.checked ? 1 : 0}
                  class="sr-only peer"
                />
                <div class="w-9 h-5 bg-gray-200 rounded-full peer-checked:bg-primary peer-focus:ring-2 peer-focus:ring-primary/30 transition-colors" />
                <div class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full peer-checked:translate-x-4 transition-transform" />
              </label>
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">最低等级</span>
              <input type="number" min="1" value={form.value.minlevel} onInput={(e: any) => form.value.minlevel = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">最高等级</span>
              <input type="number" min="1" value={form.value.maxlevel} onInput={(e: any) => form.value.maxlevel = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">X 坐标</span>
              <input type="number" value={form.value.pos_x} onInput={(e: any) => form.value.pos_x = Number(e.target.value)} class="input text-sm" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">Y 坐标</span>
              <input type="number" value={form.value.pos_y} onInput={(e: any) => form.value.pos_y = Number(e.target.value)} class="input text-sm" />
            </label>
          </div>

          <div class="mt-4 pt-4 border-t border-gray-100">
            <h4 class="text-sm font-bold mb-2">野生精灵</h4>
            <div class="flex items-center gap-2 mb-3">
              <input
                type="number"
                value={wildPokemonInput.value}
                onInput={(e: any) => wildPokemonInput.value = e.target.value}
                onKeyup={(e: KeyboardEvent) => { if (e.key === 'Enter') addWildPokemon() }}
                class="input text-sm w-32"
                placeholder="精灵ID"
              />
              <button class="px-3 py-1.5 text-xs rounded bg-primary/10 text-primary hover:bg-primary/20 transition-colors" onClick={addWildPokemon}>
                添加
              </button>
            </div>
            {!Array.isArray(form.value.wild) || form.value.wild.length === 0 ? (
              <div class="text-xs text-gray-400 py-2">暂无野生精灵</div>
            ) : (
              <div class="flex flex-wrap gap-2">
                {form.value.wild.map((id: number) => (
                  <div key={id} class="flex items-center gap-1 px-2 py-1 bg-gray-100 rounded text-xs">
                    <span class="font-mono">{id}</span>
                    <button class="text-red-400 hover:text-red-600 leading-none" onClick={() => removeWildPokemon(id)}>&#x2715;</button>
                  </div>
                ))}
              </div>
            )}
          </div>

          <div class="mt-4 pt-4 border-t border-gray-100">
            <h4 class="text-sm font-bold mb-2">Boss 配置 (expn)</h4>
            <textarea
              value={form.value.expn}
              onInput={(e: any) => form.value.expn = e.target.value}
              class="input text-sm min-h-24 w-full font-mono resize-y"
              placeholder='{"boss_id": 1, "exp": 1000}'
            />
          </div>

          {formError.value && (
            <div class="mt-3 text-red-500 text-sm">{formError.value}</div>
          )}

          <div class="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-gray-100">
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" onClick={closeEditor}>取消</button>
            <button class="btn-primary text-sm" disabled={saving.value} onClick={submitForm}>
              {saving.value ? (
                <span class="i-svg-spinners-3-dots-bounce text-white" />
              ) : (
                <span>保存</span>
              )}
            </button>
          </div>
        </Modal>
      </div>
    )
  },
})
