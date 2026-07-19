<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAdminStore } from '@/stores/admin'
import Modal from '@/components/common/Modal.vue'
import { getTypeColor } from '@/utils/pokemon'

const POKEMON_TYPES = ['普', '火', '水', '草', '电', '冰', '斗', '毒', '地', '飞', '超', '虫', '岩', '鬼', '龙', '恶', '钢', '妖']

const store = useAdminStore('pokemon')
const { items, loading, error, total, hasMore, reload, loadMore, save: storeSave, remove: storeRemove, applyFilters } = store

const editorOpen = ref(false)
const editorIsCreate = ref(false)
const form = ref<Record<string, any>>({})
const formError = ref<string | null>(null)
const saving = ref(false)
const filterOpen = ref(false)
const filterForm = ref({ name: '', type: '', minlevel: '', maxlevel: '' })

function onTableScroll(e: Event) {
  const el = e.target as HTMLElement
  if (el.scrollHeight - el.scrollTop - el.clientHeight < 80) {
    loadMore()
  }
}

function openCreate() {
  editorIsCreate.value = true
  form.value = {
    id: 0, name: '', txt: '', xs: '', xs2: '',
    hp: 0, atk: 0, def: 0, spatk: 0, spdef: 0, sd: 0,
    mapid: 0, capture: 255, money: 0, birth: 0, birthodds: 0,
    pnclevel: 1, god: 0, strength: 0,
  }
  formError.value = null
  editorOpen.value = true
}

function openEdit(row: any) {
  editorIsCreate.value = false
  form.value = {
    id: row.id ?? 0,
    name: row.name ?? '',
    txt: row.txt ?? '',
    xs: row.xs ?? '',
    xs2: row.xs2 ?? '',
    hp: Number(row.hp ?? 0),
    atk: Number(row.atk ?? 0),
    def: Number(row.def ?? 0),
    spatk: Number(row.spatk ?? 0),
    spdef: Number(row.spdef ?? 0),
    sd: Number(row.sd ?? 0),
    mapid: Number(row.mapid ?? 0),
    capture: Number(row.capture ?? 255),
    money: Number(row.money ?? 0),
    birth: Number(row.birth ?? 0),
    birthodds: Number(row.birthodds ?? 0),
    pnclevel: Number(row.pnclevel ?? 1),
    god: Number(row.god ?? 0),
    strength: Number(row.strength ?? 0),
  }
  formError.value = null
  editorOpen.value = true
}

function closeEditor() {
  editorOpen.value = false
}

async function submitForm() {
  if (!form.value.name) {
    formError.value = '名称不能为空'
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
  if (filterForm.value.type) f.type = filterForm.value.type
  if (filterForm.value.minlevel) f.minlevel = Number(filterForm.value.minlevel)
  if (filterForm.value.maxlevel) f.maxlevel = Number(filterForm.value.maxlevel)
  applyFilters(f)
  filterOpen.value = false
}

function resetFilter() {
  applyFilters({})
  filterOpen.value = false
  filterForm.value = { name: '', type: '', minlevel: '', maxlevel: '' }
}

onMounted(() => {
  reload()
})
</script>

<template>
  <div class="flex flex-col" style="height: calc(100vh - 6.5rem)">
    <div class="flex items-center justify-between mb-3">
      <h2 class="text-lg font-bold">宠物数据管理</h2>
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
              <th class="px-1 py-2 text-center w-6">T1</th>
              <th class="px-1 py-2 text-center w-6">T2</th>
              <th class="px-1 py-2 text-right w-8">HP</th>
              <th class="px-1 py-2 text-right w-8">ATK</th>
              <th class="px-1 py-2 text-right w-8">DEF</th>
              <th class="px-1 py-2 text-right w-10">SA</th>
              <th class="px-1 py-2 text-right w-10">SD</th>
              <th class="px-1 py-2 text-right w-8">SPD</th>
              <th class="px-1 py-2 text-right w-10">捕捉</th>
              <th class="px-2 py-2 text-center w-18">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="loading && items.length === 0">
              <td colspan="12" class="text-center text-gray-400 py-8">
                <span class="i-svg-spinners-3-dots-bounce text-lg" />
              </td>
            </tr>
            <tr v-else-if="!loading && items.length === 0">
              <td colspan="12" class="text-center text-gray-400 py-8">暂无数据</td>
            </tr>
            <tr
              v-for="row in items"
              :key="row.id"
              class="border-b border-gray-100 hover:bg-gray-50 cursor-pointer transition-colors"
              @click="openEdit(row)"
            >
              <td class="px-2 py-2 font-mono text-xs">{{ row.id }}</td>
              <td class="px-2 py-2 font-medium truncate">{{ row.name }}</td>
              <td class="px-1 py-2 text-center">
                <span v-if="row.xs" class="inline-block rounded px-1 py-0.5 text-[10px] font-bold text-white" :style="{ backgroundColor: getTypeColor(row.xs) }">
                  {{ row.xs }}
                </span>
              </td>
              <td class="px-1 py-2 text-center">
                <span v-if="row.xs2" class="inline-block rounded px-1 py-0.5 text-[10px] font-bold text-white" :style="{ backgroundColor: getTypeColor(row.xs2) }">
                  {{ row.xs2 }}
                </span>
              </td>
              <td class="px-1 py-2 text-right text-xs">{{ row.hp }}</td>
              <td class="px-1 py-2 text-right text-xs">{{ row.atk }}</td>
              <td class="px-1 py-2 text-right text-xs">{{ row.def }}</td>
              <td class="px-1 py-2 text-right text-xs">{{ row.spatk }}</td>
              <td class="px-1 py-2 text-right text-xs">{{ row.spdef }}</td>
              <td class="px-1 py-2 text-right text-xs">{{ row.sd }}</td>
              <td class="px-1 py-2 text-right text-xs">{{ row.capture }}</td>
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
              <td colspan="12" class="text-center text-gray-400 py-2">
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
              <span class="text-xs text-gray-500">属性</span>
              <select v-model="filterForm.type" class="input w-20 text-sm">
                <option value="">全部</option>
                <option v-for="t in POKEMON_TYPES" :key="t" :value="t">{{ t }}</option>
              </select>
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

    <Modal :open="editorOpen" :title="editorIsCreate ? '新增宠物' : '编辑宠物'" wide @close="closeEditor">
      <div class="grid grid-cols-2 gap-3 max-h-100 overflow-y-auto">
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">ID</span>
          <input v-model.number="form.id" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">名称 *</span>
          <input v-model="form.name" type="text" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">属性1</span>
          <select v-model="form.xs" class="input text-sm">
            <option value="">无</option>
            <option v-for="t in POKEMON_TYPES" :key="t" :value="t">{{ t }}</option>
          </select>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">属性2</span>
          <select v-model="form.xs2" class="input text-sm">
            <option value="">无</option>
            <option v-for="t in POKEMON_TYPES" :key="t" :value="t">{{ t }}</option>
          </select>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">HP</span>
          <input v-model.number="form.hp" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">ATK</span>
          <input v-model.number="form.atk" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">DEF</span>
          <input v-model.number="form.def" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">SPATK</span>
          <input v-model.number="form.spatk" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">SPDEF</span>
          <input v-model.number="form.spdef" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">SPEED</span>
          <input v-model.number="form.sd" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">捕捉率</span>
          <input v-model.number="form.capture" type="number" min="0" max="255" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">栖息地 (mapid)</span>
          <input v-model.number="form.mapid" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">价格</span>
          <input v-model.number="form.money" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">生育</span>
          <input v-model.number="form.birth" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">生育几率</span>
          <input v-model.number="form.birthodds" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">PNC等级</span>
          <input v-model.number="form.pnclevel" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">神兽标识</span>
          <input v-model.number="form.god" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">强度</span>
          <input v-model.number="form.strength" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1 col-span-2">
          <span class="text-xs text-gray-500">描述</span>
          <textarea v-model="form.txt" class="input text-sm min-h-16 resize-y" />
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
