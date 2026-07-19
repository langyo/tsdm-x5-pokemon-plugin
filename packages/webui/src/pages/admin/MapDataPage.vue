<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAdminStore } from '@/stores/admin'
import Modal from '@/components/common/Modal.vue'

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
</script>

<template>
  <div class="flex flex-col" style="height: calc(100vh - 6.5rem)">
    <div class="flex items-center justify-between mb-3">
      <h2 class="text-lg font-bold">地图设定管理</h2>
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
              <th class="px-2 py-2 text-left w-24">名称</th>
              <th class="px-2 py-2 text-center w-12">开放</th>
              <th class="px-2 py-2 text-center w-20">等级范围</th>
              <th class="px-2 py-2 text-left w-16">区域</th>
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
              <td class="px-2 py-2 font-medium truncate">{{ row.name }}</td>
              <td class="px-2 py-2 text-center">
                <span class="inline-block rounded-full w-2 h-2" :class="row.kg == 1 ? 'bg-green-500' : 'bg-gray-300'" />
                <span class="ml-1 text-xs text-gray-500">{{ row.kg == 1 ? '是' : '否' }}</span>
              </td>
              <td class="px-2 py-2 text-center text-xs">
                <span class="font-mono">{{ row.minlevel ?? '-' }} - {{ row.maxlevel ?? '-' }}</span>
              </td>
              <td class="px-2 py-2 text-xs truncate">{{ row.region || '-' }}</td>
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
              <span class="text-xs text-gray-500">区域</span>
              <input v-model="filterForm.region" type="text" class="input w-24 text-sm" placeholder="region" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">最低等级</span>
              <input v-model="filterForm.minlevel" type="number" class="input w-20 text-sm" placeholder="0" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">最高等级</span>
              <input v-model="filterForm.maxlevel" type="number" class="input w-20 text-sm" placeholder="100" />
            </label>
            <button class="btn-primary text-sm" @click="doApplyFilter">应用</button>
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="resetFilter">重置</button>
          </div>
        </div>
      </div>
    </Teleport>

    <Modal :open="editorOpen" :title="editorIsCreate ? '新增地图' : '编辑地图'" wide @close="closeEditor">
      <div class="grid grid-cols-3 gap-3 max-h-100 overflow-y-auto">
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">名称 *</span>
          <input v-model="form.name" type="text" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">区域</span>
          <input v-model="form.region" type="text" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">开放 (kg)</span>
          <label class="relative inline-flex items-center cursor-pointer mt-1">
            <input v-model="form.kg" type="checkbox" :true-value="1" :false-value="0" class="sr-only peer" />
            <div class="w-9 h-5 bg-gray-200 rounded-full peer-checked:bg-primary peer-focus:ring-2 peer-focus:ring-primary/30 transition-colors" />
            <div class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full peer-checked:translate-x-4 transition-transform" />
          </label>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">最低等级</span>
          <input v-model.number="form.minlevel" type="number" min="1" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">最高等级</span>
          <input v-model.number="form.maxlevel" type="number" min="1" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">X 坐标</span>
          <input v-model.number="form.pos_x" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">Y 坐标</span>
          <input v-model.number="form.pos_y" type="number" class="input text-sm" />
        </label>
      </div>

      <div class="mt-4 pt-4 border-t border-gray-100">
        <h4 class="text-sm font-bold mb-2">野生精灵</h4>
        <div class="flex items-center gap-2 mb-3">
          <input
            v-model="wildPokemonInput"
            type="number"
            class="input text-sm w-32"
            placeholder="精灵ID"
            @keyup.enter="addWildPokemon"
          />
          <button class="px-3 py-1.5 text-xs rounded bg-primary/10 text-primary hover:bg-primary/20 transition-colors" @click="addWildPokemon">
            添加
          </button>
        </div>
        <div v-if="!Array.isArray(form.wild) || form.wild.length === 0" class="text-xs text-gray-400 py-2">
          暂无野生精灵
        </div>
        <div v-else class="flex flex-wrap gap-2">
          <div
            v-for="id in form.wild"
            :key="id"
            class="flex items-center gap-1 px-2 py-1 bg-gray-100 rounded text-xs"
          >
            <span class="font-mono">{{ id }}</span>
            <button class="text-red-400 hover:text-red-600 leading-none" @click="removeWildPokemon(id)">&#x2715;</button>
          </div>
        </div>
      </div>

      <div class="mt-4 pt-4 border-t border-gray-100">
        <h4 class="text-sm font-bold mb-2">Boss 配置 (expn)</h4>
        <textarea
          v-model="form.expn"
          class="input text-sm min-h-24 w-full font-mono resize-y"
          placeholder='{"boss_id": 1, "exp": 1000}'
        />
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
