<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAdminStore } from '@/stores/admin'
import Modal from '@/components/common/Modal.vue'
import { api } from '@/api/client'

const EVOLUTION_METHODS = ['level', 'item', 'intimacy', 'trade', 'stone']

const store = useAdminStore('evolution')
const { items, loading, error, total, hasMore, reload, loadMore, save: storeSave, remove: storeRemove, applyFilters } = store

const editorOpen = ref(false)
const editorIsCreate = ref(false)
const form = ref<Record<string, any>>({})
const formError = ref<string | null>(null)
const saving = ref(false)
const filterOpen = ref(false)
const filterForm = ref({ fromName: '', toName: '', method: '' })

const fromSearch = ref('')
const fromResults = ref<any[]>([])
const toSearch = ref('')
const toResults = ref<any[]>([])

function onTableScroll(e: Event) {
  const el = e.target as HTMLElement
  if (el.scrollHeight - el.scrollTop - el.clientHeight < 80) {
    loadMore()
  }
}

function openCreate() {
  editorIsCreate.value = true
  form.value = { from_id: 0, to_id: 0, method: 'level', condition: '' }
  fromSearch.value = ''
  fromResults.value = []
  toSearch.value = ''
  toResults.value = []
  formError.value = null
  editorOpen.value = true
}

function openEdit(row: any) {
  editorIsCreate.value = false
  form.value = {
    id: row.id,
    from_id: Number(row.from_id ?? 0),
    to_id: Number(row.to_id ?? 0),
    method: row.method ?? 'level',
    condition: row.condition ?? '',
  }
  fromSearch.value = row.from_name ?? row.fromName ?? ''
  fromResults.value = []
  toSearch.value = row.to_name ?? row.toName ?? ''
  toResults.value = []
  formError.value = null
  editorOpen.value = true
}

function closeEditor() {
  editorOpen.value = false
}

let fromTimer: ReturnType<typeof setTimeout> | null = null
async function searchFromPokemon() {
  if (fromTimer) clearTimeout(fromTimer)
  fromTimer = setTimeout(async () => {
    const q = fromSearch.value.trim()
    if (!q) {
      fromResults.value = []
      return
    }
    try {
      const res = await api.admin('filter::pokemon', { name: q })
      fromResults.value = res.data ?? res.list ?? res.items ?? []
    } catch { /* ignore */ }
  }, 300)
}

function selectFromPokemon(pkm: any) {
  form.value.from_id = pkm.id
  fromSearch.value = pkm.name
  fromResults.value = []
}

let toTimer: ReturnType<typeof setTimeout> | null = null
async function searchToPokemon() {
  if (toTimer) clearTimeout(toTimer)
  toTimer = setTimeout(async () => {
    const q = toSearch.value.trim()
    if (!q) {
      toResults.value = []
      return
    }
    try {
      const res = await api.admin('filter::pokemon', { name: q })
      toResults.value = res.data ?? res.list ?? res.items ?? []
    } catch { /* ignore */ }
  }, 300)
}

function selectToPokemon(pkm: any) {
  form.value.to_id = pkm.id
  toSearch.value = pkm.name
  toResults.value = []
}

async function submitForm() {
  if (!form.value.from_id || !form.value.to_id) {
    formError.value = '请选择进化前后的宠物'
    return
  }
  saving.value = true
  formError.value = null
  try {
    await storeSave({ ...form.value })
    editorOpen.value = false
  } catch (e: any) {
    formError.value = e.message
  } finally {
    saving.value = false
  }
}

async function deleteRow(row: any) {
  const label = `${row.from_name ?? row.from_id} -> ${row.to_name ?? row.to_id}`
  if (!confirm(`确认删除进化路线 "${label}" (ID: ${row.id})？`)) return
  try {
    await storeRemove(row.id)
  } catch (e: any) {
    alert('删除失败: ' + e.message)
  }
}

function doApplyFilter() {
  const f: Record<string, any> = {}
  if (filterForm.value.fromName) f.fromName = filterForm.value.fromName
  if (filterForm.value.toName) f.toName = filterForm.value.toName
  if (filterForm.value.method) f.method = filterForm.value.method
  applyFilters(f)
  filterOpen.value = false
}

function resetFilter() {
  applyFilters({})
  filterOpen.value = false
  filterForm.value = { fromName: '', toName: '', method: '' }
}

onMounted(() => {
  reload()
})
</script>

<template>
  <div class="flex flex-col" style="height: calc(100vh - 6.5rem)">
    <div class="flex items-center justify-between mb-3">
      <h2 class="text-lg font-bold">进化路线管理</h2>
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
              <th class="px-2 py-2 text-left w-12">ID</th>
              <th class="px-2 py-2 text-left w-32">进化前</th>
              <th class="px-2 py-2 text-left w-32">进化后</th>
              <th class="px-2 py-2 text-center w-20">方式</th>
              <th class="px-2 py-2 text-left">条件</th>
              <th class="px-2 py-2 text-center w-18">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="loading && items.length === 0">
              <td colspan="6" class="text-center text-gray-400 py-8">
                <span class="i-svg-spinners-3-dots-bounce text-lg" />
              </td>
            </tr>
            <tr v-else-if="!loading && items.length === 0">
              <td colspan="6" class="text-center text-gray-400 py-8">暂无数据</td>
            </tr>
            <tr
              v-for="row in items"
              :key="row.id"
              class="border-b border-gray-100 hover:bg-gray-50 cursor-pointer transition-colors"
              @click="openEdit(row)"
            >
              <td class="px-2 py-2 font-mono text-xs">{{ row.id }}</td>
              <td class="px-2 py-2">
                <span class="font-mono text-xs text-gray-400 mr-1">#{{ row.from_id }}</span>
                <span class="font-medium text-sm">{{ row.from_name ?? row.fromName }}</span>
              </td>
              <td class="px-2 py-2">
                <span class="font-mono text-xs text-gray-400 mr-1">#{{ row.to_id }}</span>
                <span class="font-medium text-sm">{{ row.to_name ?? row.toName }}</span>
              </td>
              <td class="px-2 py-2 text-center">
                <span class="inline-block rounded px-1.5 py-0.5 text-[10px] font-bold text-white"
                  :class="{
                    'bg-blue-500': row.method === 'level',
                    'bg-purple-500': row.method === 'item',
                    'bg-pink-500': row.method === 'intimacy',
                    'bg-orange-500': row.method === 'trade',
                    'bg-teal-500': row.method === 'stone',
                  }"
                >
                  {{ row.method }}
                </span>
              </td>
              <td class="px-2 py-2 text-xs text-gray-500 truncate">{{ row.condition || '-' }}</td>
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
              <td colspan="6" class="text-center text-gray-400 py-2">
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
              <span class="text-xs text-gray-500">进化前名称</span>
              <input v-model="filterForm.fromName" type="text" class="input w-32 text-sm" placeholder="关键词" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">进化后名称</span>
              <input v-model="filterForm.toName" type="text" class="input w-32 text-sm" placeholder="关键词" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">方式</span>
              <select v-model="filterForm.method" class="input w-24 text-sm">
                <option value="">全部</option>
                <option v-for="m in EVOLUTION_METHODS" :key="m" :value="m">{{ m }}</option>
              </select>
            </label>
            <button class="btn-primary text-sm" @click="doApplyFilter">应用</button>
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="resetFilter">重置</button>
          </div>
        </div>
      </div>
    </Teleport>

    <!-- Editor Modal -->
    <Modal :open="editorOpen" :title="editorIsCreate ? '新增进化路线' : '编辑进化路线'" @close="closeEditor">
      <div class="space-y-3">
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">进化前宠物 *</span>
          <div class="relative">
            <input
              v-model="fromSearch"
              type="text"
              class="input text-sm w-full"
              placeholder="搜索宠物名称..."
              @input="searchFromPokemon"
            />
            <div v-if="fromResults.length > 0" class="absolute top-full left-0 right-0 bg-white border border-gray-200 rounded-lg shadow-lg max-h-40 overflow-y-auto z-50">
              <div
                v-for="r in fromResults"
                :key="r.id"
                class="px-3 py-1.5 text-sm hover:bg-primary/10 cursor-pointer"
                @click="selectFromPokemon(r)"
              >
                <span class="font-mono text-xs text-gray-400 mr-2">#{{ r.id }}</span>
                {{ r.name }}
              </div>
            </div>
          </div>
          <span v-if="form.from_id" class="text-xs text-green-600 mt-0.5">已选择: #{{ form.from_id }}</span>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">进化后宠物 *</span>
          <div class="relative">
            <input
              v-model="toSearch"
              type="text"
              class="input text-sm w-full"
              placeholder="搜索宠物名称..."
              @input="searchToPokemon"
            />
            <div v-if="toResults.length > 0" class="absolute top-full left-0 right-0 bg-white border border-gray-200 rounded-lg shadow-lg max-h-40 overflow-y-auto z-50">
              <div
                v-for="r in toResults"
                :key="r.id"
                class="px-3 py-1.5 text-sm hover:bg-primary/10 cursor-pointer"
                @click="selectToPokemon(r)"
              >
                <span class="font-mono text-xs text-gray-400 mr-2">#{{ r.id }}</span>
                {{ r.name }}
              </div>
            </div>
          </div>
          <span v-if="form.to_id" class="text-xs text-green-600 mt-0.5">已选择: #{{ form.to_id }}</span>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">进化方式</span>
          <select v-model="form.method" class="input text-sm">
            <option v-for="m in EVOLUTION_METHODS" :key="m" :value="m">{{ m }}</option>
          </select>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">条件值</span>
          <input v-model="form.condition" type="text" class="input text-sm" placeholder="如: 16 (等级), 道具ID, 好感度值" />
        </label>
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
