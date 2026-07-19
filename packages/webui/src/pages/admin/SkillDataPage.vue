<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAdminStore } from '@/stores/admin'
import Modal from '@/components/common/Modal.vue'
import { api } from '@/api/client'
import { getTypeColor } from '@/utils/pokemon'

const SKILL_TYPES = ['普', '火', '水', '草', '电', '冰', '斗', '毒', '地', '飞', '超', '虫', '岩', '鬼', '龙', '恶', '钢', '妖']
const SKILL_CATEGORIES = ['physical', 'special', 'status']

const store = useAdminStore('skill')
const { items, loading, error, total, hasMore, reload, loadMore, save: storeSave, remove: storeRemove, applyFilters } = store

const editorOpen = ref(false)
const editorIsCreate = ref(false)
const form = ref<Record<string, any>>({})
const formError = ref<string | null>(null)
const saving = ref(false)
const filterOpen = ref(false)
const filterForm = ref({ name: '', type: '', minPower: '', maxPower: '' })

const pokemonSearch = ref('')
const pokemonResults = ref<any[]>([])
const selectedPokemon = ref<any[]>([])

function onTableScroll(e: Event) {
  const el = e.target as HTMLElement
  if (el.scrollHeight - el.scrollTop - el.clientHeight < 80) {
    loadMore()
  }
}

function parseAvailablePokemon(raw: any): any[] {
  if (!raw) return []
  if (Array.isArray(raw)) return raw
  if (typeof raw === 'string') {
    try {
      const parsed = JSON.parse(raw)
      return Array.isArray(parsed) ? parsed : []
    } catch {
      return raw.split(',').map(Number).filter((n: number) => !isNaN(n))
    }
  }
  return []
}

function openCreate() {
  editorIsCreate.value = true
  form.value = {
    name: '', txt: '', lv: 1, powr: 0, num: 10, type: '普', tn: '', category: 'physical',
  }
  selectedPokemon.value = []
  pokemonSearch.value = ''
  pokemonResults.value = []
  formError.value = null
  editorOpen.value = true
}

function openEdit(row: any) {
  editorIsCreate.value = false
  form.value = {
    id: row.id,
    name: row.name ?? '',
    txt: row.txt ?? '',
    lv: Number(row.lv ?? 1),
    powr: Number(row.powr ?? 0),
    num: Number(row.num ?? 10),
    type: row.type ?? '普',
    tn: row.tn ?? '',
    category: row.category ?? 'physical',
  }
  selectedPokemon.value = parseAvailablePokemon(row.available ?? row.pokemon ?? [])
  pokemonSearch.value = ''
  pokemonResults.value = []
  formError.value = null
  editorOpen.value = true
}

function closeEditor() {
  editorOpen.value = false
}

let searchTimer: ReturnType<typeof setTimeout> | null = null
async function searchPokemon() {
  if (searchTimer) clearTimeout(searchTimer)
  searchTimer = setTimeout(async () => {
    const q = pokemonSearch.value.trim()
    if (!q) {
      pokemonResults.value = []
      return
    }
    try {
      const res = await api.admin('filter::pokemon', { name: q })
      pokemonResults.value = res.data ?? res.list ?? res.items ?? []
    } catch { /* ignore */ }
  }, 300)
}

function addPokemon(pkm: any) {
  const id = pkm.id
  if (!selectedPokemon.value.find((p: any) => (typeof p === 'object' ? p.id === id : p === id))) {
    selectedPokemon.value.push({ id: pkm.id, name: pkm.name })
  }
  pokemonSearch.value = ''
  pokemonResults.value = []
}

function removePokemon(id: number) {
  selectedPokemon.value = selectedPokemon.value.filter((p: any) =>
    (typeof p === 'object' ? p.id !== id : p !== id)
  )
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
    data.available = selectedPokemon.value.map((p: any) => (typeof p === 'object' ? p.id : p))
    await storeSave(data)
    editorOpen.value = false
  } catch (e: any) {
    formError.value = e.message
  } finally {
    saving.value = false
  }
}

async function deleteRow(row: any) {
  if (!confirm(`确认删除技能 "${row.name}" (ID: ${row.id})？`)) return
  try {
    await storeRemove(row.id)
  } catch (e: any) {
    alert('删除失败: ' + e.message)
  }
}

function doApplyFilter() {
  const f: Record<string, any> = {}
  if (filterForm.value.name) f.name = filterForm.value.name
  if (filterForm.value.type) f.type = filterForm.value.type
  if (filterForm.value.minPower) f.minPower = Number(filterForm.value.minPower)
  if (filterForm.value.maxPower) f.maxPower = Number(filterForm.value.maxPower)
  applyFilters(f)
  filterOpen.value = false
}

function resetFilter() {
  applyFilters({})
  filterOpen.value = false
  filterForm.value = { name: '', type: '', minPower: '', maxPower: '' }
}

onMounted(() => {
  reload()
})
</script>

<template>
  <div class="flex flex-col" style="height: calc(100vh - 6.5rem)">
    <div class="flex items-center justify-between mb-3">
      <h2 class="text-lg font-bold">技能数据管理</h2>
      <span class="text-xs text-gray-400">共 {{ total }} 条</span>
    </div>

    <div v-if="error" class="card bg-red-50 border-red-200 mb-3">
      <p class="text-red-600 text-sm">{{ error }}</p>
    </div>

    <div class="card flex-1 flex flex-col min-h-0 p-0 overflow-hidden">
      <div class="overflow-y-auto flex-1" @scroll="onTableScroll">
        <table class="w-full text-sm table-fixed">
          <thead class="sticky top-0 bg-gray-50 z-10">
            <tr class="border-b border-gray-200">
              <th class="px-2 py-2 text-left w-10">ID</th>
              <th class="px-2 py-2 text-left w-28">名称</th>
              <th class="px-2 py-2 text-center w-12">属性</th>
              <th class="px-2 py-2 text-right w-12">威力</th>
              <th class="px-2 py-2 text-right w-10">PP</th>
              <th class="px-2 py-2 text-center w-16">类别</th>
              <th class="px-2 py-2 text-left">可学习精灵</th>
              <th class="px-2 py-2 text-center w-18">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="loading && items.length === 0">
              <td colspan="8" class="text-center text-gray-400 py-8">
                <span class="i-svg-spinners-3-dots-bounce text-lg" />
              </td>
            </tr>
            <tr v-else-if="!loading && items.length === 0">
              <td colspan="8" class="text-center text-gray-400 py-8">暂无数据</td>
            </tr>
            <tr
              v-for="row in items"
              :key="row.id"
              class="border-b border-gray-100 hover:bg-gray-50 cursor-pointer transition-colors"
              @click="openEdit(row)"
            >
              <td class="px-2 py-2 font-mono text-xs">{{ row.id }}</td>
              <td class="px-2 py-2 font-medium truncate">{{ row.name }}</td>
              <td class="px-2 py-2 text-center">
                <span class="inline-block rounded px-1.5 py-0.5 text-[10px] font-bold text-white" :style="{ backgroundColor: getTypeColor(row.type) }">
                  {{ row.type }}
                </span>
              </td>
              <td class="px-2 py-2 text-right text-xs font-mono">{{ row.powr ?? 0 }}</td>
              <td class="px-2 py-2 text-right text-xs font-mono">{{ row.num ?? 0 }}</td>
              <td class="px-2 py-2 text-center">
                <span class="inline-block rounded px-1 py-0.5 text-[10px] font-bold"
                  :class="{
                    'bg-red-100 text-red-600': row.category === 'physical',
                    'bg-blue-100 text-blue-600': row.category === 'special',
                    'bg-gray-100 text-gray-600': row.category === 'status',
                  }"
                >
                  {{ row.category === 'physical' ? '物理' : row.category === 'special' ? '特殊' : '变化' }}
                </span>
              </td>
              <td class="px-2 py-2 text-xs text-gray-500">
                <span v-if="row.available && row.available.length > 0" class="truncate block">
                  {{ Array.isArray(row.available) ? row.available.map((p: any) => typeof p === 'object' ? p.name ?? p.id : p).slice(0, 3).join(', ') + (row.available.length > 3 ? '...' : '') : row.available }}
                </span>
                <span v-else class="text-gray-300">-</span>
              </td>
              <td class="px-2 py-2 text-center" @click.stop>
                <div class="flex items-center justify-center gap-1">
                  <button class="px-2 py-1 text-xs rounded bg-primary/10 text-primary hover:bg-primary/20 transition-colors" @click="openEdit(row)">
                    编辑
                  </button>
                  <button class="px-2 py-1 text-xs rounded bg-red-50 text-red-500 hover:bg-red-100 transition-colors" @click="deleteRow(row)">
                    删除
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
          <tfoot v-if="loading && items.length > 0">
            <tr>
              <td colspan="8" class="text-center text-gray-400 py-2">
                <span class="i-svg-spinners-3-dots-bounce text-sm" />
              </td>
            </tr>
          </tfoot>
        </table>
      </div>
    </div>

    <div class="fixed bottom-0 left-48 right-0 bg-white border-t border-gray-200 px-4 py-2 flex items-center justify-center gap-3 z-20 shadow-lg">
      <button class="btn-primary text-sm" @click="openCreate">新增</button>
      <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="reload()">刷新</button>
      <button class="btn text-sm border border-gray-300 hover:bg-gray-100" :class="{ 'bg-primary/10 border-primary': filterOpen }" @click="filterOpen = !filterOpen">筛选</button>
    </div>

    <!-- Filter Panel -->
    <Teleport to="body">
      <div v-if="filterOpen" class="fixed inset-0 z-30">
        <div class="absolute inset-0 bg-black/30" @click="filterOpen = false" />
        <div class="absolute bottom-14 left-48 right-0 bg-white border-t shadow-xl p-4">
          <div class="flex flex-wrap items-end gap-3">
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">名称</span>
              <input v-model="filterForm.name" type="text" class="input w-32 text-sm" placeholder="关键词" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">属性</span>
              <select v-model="filterForm.type" class="input w-20 text-sm">
                <option value="">全部</option>
                <option v-for="t in SKILL_TYPES" :key="t" :value="t">{{ t }}</option>
              </select>
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">最低威力</span>
              <input v-model="filterForm.minPower" type="number" class="input w-20 text-sm" placeholder="0" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">最高威力</span>
              <input v-model="filterForm.maxPower" type="number" class="input w-24 text-sm" placeholder="999" />
            </label>
            <button class="btn-primary text-sm" @click="doApplyFilter">应用</button>
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="resetFilter">重置</button>
          </div>
        </div>
      </div>
    </Teleport>

    <!-- Editor Modal -->
    <Modal :open="editorOpen" :title="editorIsCreate ? '新增技能' : '编辑技能'" wide @close="closeEditor">
      <div class="grid grid-cols-2 gap-3 max-h-100 overflow-y-auto">
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">名称 *</span>
          <input v-model="form.name" type="text" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">属性名 (tn)</span>
          <input v-model="form.tn" type="text" class="input text-sm" placeholder="如: 火之牙" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">类型</span>
          <select v-model="form.type" class="input text-sm">
            <option v-for="t in SKILL_TYPES" :key="t" :value="t">{{ t }}</option>
          </select>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">类别</span>
          <select v-model="form.category" class="input text-sm">
            <option value="physical">物理</option>
            <option value="special">特殊</option>
            <option value="status">变化</option>
          </select>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">等级要求</span>
          <input v-model.number="form.lv" type="number" min="1" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">威力</span>
          <input v-model.number="form.powr" type="number" min="0" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">PP</span>
          <input v-model.number="form.num" type="number" min="0" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1 col-span-2">
          <span class="text-xs text-gray-500">描述</span>
          <textarea v-model="form.txt" class="input text-sm min-h-16 resize-y" placeholder="技能描述..." />
        </label>
      </div>

      <div class="mt-4 pt-4 border-t border-gray-100">
        <h4 class="text-sm font-bold mb-2">可学习精灵</h4>
        <div class="flex items-center gap-2 mb-3">
          <div class="relative flex-1">
            <input
              v-model="pokemonSearch"
              type="text"
              class="input text-sm w-full"
              placeholder="搜索精灵名称..."
              @input="searchPokemon"
            />
            <div v-if="pokemonResults.length > 0" class="absolute top-full left-0 right-0 bg-white border border-gray-200 rounded-lg shadow-lg max-h-40 overflow-y-auto z-50">
              <div
                v-for="r in pokemonResults"
                :key="r.id"
                class="px-3 py-1.5 text-sm hover:bg-primary/10 cursor-pointer"
                @click="addPokemon(r)"
              >
                <span class="font-mono text-xs text-gray-400 mr-2">#{{ r.id }}</span>
                {{ r.name }}
              </div>
            </div>
          </div>
        </div>
        <div v-if="selectedPokemon.length === 0" class="text-xs text-gray-400 py-2">暂无可学习精灵</div>
        <div v-else class="flex flex-wrap gap-2">
          <div
            v-for="p in selectedPokemon"
            :key="typeof p === 'object' ? p.id : p"
            class="flex items-center gap-1 px-2 py-1 bg-gray-100 rounded text-xs"
          >
            <span class="font-mono text-gray-400">#{{ typeof p === 'object' ? p.id : p }}</span>
            <span v-if="typeof p === 'object' && p.name">{{ p.name }}</span>
            <button class="text-red-400 hover:text-red-600 leading-none" @click="removePokemon(typeof p === 'object' ? p.id : p)">&#x2715;</button>
          </div>
        </div>
      </div>

      <div v-if="formError" class="mt-3 text-red-500 text-sm">{{ formError }}</div>

      <div class="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-gray-100">
        <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="closeEditor">取消</button>
        <button class="btn-primary text-sm" :disabled="saving" @click="submitForm">
          <span v-if="saving" class="i-svg-spinners-3-dots-bounce text-white" />
          <span v-else>保存</span>
        </button>
      </div>
    </Modal>
  </div>
</template>
