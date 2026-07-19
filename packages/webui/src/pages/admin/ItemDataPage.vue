<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAdminStore } from '@/stores/admin'
import Modal from '@/components/common/Modal.vue'

const ITEM_TYPES = ['drug', 'ball', 'evolution', 'enhance', 'equip']

const store = useAdminStore('item')
const { items, loading, error, total, hasMore, reload, loadMore, save: storeSave, remove: storeRemove, applyFilters } = store

const editorOpen = ref(false)
const editorIsCreate = ref(false)
const form = ref<Record<string, any>>({})
const formError = ref<string | null>(null)
const saving = ref(false)
const filterOpen = ref(false)
const filterForm = ref({ name: '', type: '', minPrice: '', maxPrice: '' })

function onTableScroll(e: Event) {
  const el = e.target as HTMLElement
  if (el.scrollHeight - el.scrollTop - el.clientHeight < 80) {
    loadMore()
  }
}

function openCreate() {
  editorIsCreate.value = true
  form.value = {
    name: '', tpname: '', intro: '', shop: 0, money: 0, type: 'drug',
    lvask: 0, xsask: '', addhp: 0, addexp: 0, addlv: 0, addgood: 0,
    ballid: 0, upitem: 0, captmax: 255, captmin: 0, zbtype: 0,
    hp: 0, atk: 0, def: 0, spatk: 0, spdef: 0, sd: 0,
    e_atk: 0, e_def: 0, e_spatk: 0, e_spdef: 0, e_speed: 0,
  }
  formError.value = null
  editorOpen.value = true
}

function openEdit(row: any) {
  editorIsCreate.value = false
  form.value = {
    id: row.id,
    name: row.name ?? '',
    tpname: row.tpname ?? '',
    intro: row.intro ?? '',
    shop: Number(row.shop ?? 0),
    money: Number(row.money ?? 0),
    type: row.type ?? 'drug',
    lvask: Number(row.lvask ?? 0),
    xsask: row.xsask ?? '',
    addhp: Number(row.addhp ?? 0),
    addexp: Number(row.addexp ?? 0),
    addlv: Number(row.addlv ?? 0),
    addgood: Number(row.addgood ?? 0),
    ballid: Number(row.ballid ?? 0),
    upitem: Number(row.upitem ?? 0),
    captmax: Number(row.captmax ?? 255),
    captmin: Number(row.captmin ?? 0),
    zbtype: Number(row.zbtype ?? 0),
    hp: Number(row.hp ?? 0),
    atk: Number(row.atk ?? 0),
    def: Number(row.def ?? 0),
    spatk: Number(row.spatk ?? 0),
    spdef: Number(row.spdef ?? 0),
    sd: Number(row.sd ?? 0),
    e_atk: Number(row.e_atk ?? row.atk ?? 0),
    e_def: Number(row.e_def ?? row.def ?? 0),
    e_spatk: Number(row.e_spatk ?? row.spatk ?? 0),
    e_spdef: Number(row.e_spdef ?? row.spdef ?? 0),
    e_speed: Number(row.e_speed ?? row.speed ?? row.sd ?? 0),
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
  if (filterForm.value.minPrice) f.minPrice = Number(filterForm.value.minPrice)
  if (filterForm.value.maxPrice) f.maxPrice = Number(filterForm.value.maxPrice)
  applyFilters(f)
  filterOpen.value = false
}

function resetFilter() {
  applyFilters({})
  filterOpen.value = false
  filterForm.value = { name: '', type: '', minPrice: '', maxPrice: '' }
}

onMounted(() => {
  reload()
})
</script>

<template>
  <div class="flex flex-col" style="height: calc(100vh - 6.5rem)">
    <div class="flex items-center justify-between mb-3">
      <h2 class="text-lg font-bold">道具数据管理</h2>
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
              <th class="px-2 py-2 text-left w-28">名称</th>
              <th class="px-2 py-2 text-center w-16">类型</th>
              <th class="px-2 py-2 text-right w-16">价格</th>
              <th class="px-2 py-2 text-left">描述</th>
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
                <span class="inline-block rounded px-1.5 py-0.5 text-[10px] bg-gray-100 text-gray-600">{{ row.type }}</span>
              </td>
              <td class="px-2 py-2 text-right text-xs">{{ row.money }}</td>
              <td class="px-2 py-2 text-xs text-gray-500 truncate">{{ row.intro }}</td>
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
              <span class="text-xs text-gray-500">类型</span>
              <select v-model="filterForm.type" class="input w-24 text-sm">
                <option value="">全部</option>
                <option v-for="t in ITEM_TYPES" :key="t" :value="t">{{ t }}</option>
              </select>
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">最低价格</span>
              <input v-model="filterForm.minPrice" type="number" class="input w-24 text-sm" placeholder="0" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">最高价格</span>
              <input v-model="filterForm.maxPrice" type="number" class="input w-24 text-sm" placeholder="99999" />
            </label>
            <button class="btn-primary text-sm" @click="doApplyFilter">应用</button>
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="resetFilter">重置</button>
          </div>
        </div>
      </div>
    </Teleport>

    <Modal :open="editorOpen" :title="editorIsCreate ? '新增道具' : '编辑道具'" wide @close="closeEditor">
      <div class="grid grid-cols-3 gap-3 max-h-100 overflow-y-auto">
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">名称 *</span>
          <input v-model="form.name" type="text" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">显示名称</span>
          <input v-model="form.tpname" type="text" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">类型</span>
          <select v-model="form.type" class="input text-sm">
            <option v-for="t in ITEM_TYPES" :key="t" :value="t">{{ t }}</option>
          </select>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">商店</span>
          <input v-model.number="form.shop" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">价格</span>
          <input v-model.number="form.money" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">装备类型 (zbtype)</span>
          <input v-model.number="form.zbtype" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">等级要求</span>
          <input v-model.number="form.lvask" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">属性要求</span>
          <input v-model="form.xsask" type="text" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">加血</span>
          <input v-model.number="form.addhp" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">加经验</span>
          <input v-model.number="form.addexp" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">加等级</span>
          <input v-model.number="form.addlv" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">加好感度</span>
          <input v-model.number="form.addgood" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">球ID</span>
          <input v-model.number="form.ballid" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">升级道具</span>
          <input v-model.number="form.upitem" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">最大捕捉</span>
          <input v-model.number="form.captmax" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">最小捕捉</span>
          <input v-model.number="form.captmin" type="number" class="input text-sm" />
        </label>
      </div>

      <div class="mt-4 pt-4 border-t border-gray-100">
        <h4 class="text-sm font-bold mb-3">装备属性</h4>
        <div class="grid grid-cols-5 gap-3">
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
        </div>
      </div>

      <div class="mt-3">
        <h4 class="text-sm font-bold mb-3">战斗属性加成</h4>
        <div class="grid grid-cols-5 gap-3">
          <label class="flex flex-col gap-1">
            <span class="text-xs text-gray-500">e_atk</span>
            <input v-model.number="form.e_atk" type="number" class="input text-sm" />
          </label>
          <label class="flex flex-col gap-1">
            <span class="text-xs text-gray-500">e_def</span>
            <input v-model.number="form.e_def" type="number" class="input text-sm" />
          </label>
          <label class="flex flex-col gap-1">
            <span class="text-xs text-gray-500">e_spatk</span>
            <input v-model.number="form.e_spatk" type="number" class="input text-sm" />
          </label>
          <label class="flex flex-col gap-1">
            <span class="text-xs text-gray-500">e_spdef</span>
            <input v-model.number="form.e_spdef" type="number" class="input text-sm" />
          </label>
          <label class="flex flex-col gap-1">
            <span class="text-xs text-gray-500">e_speed</span>
            <input v-model.number="form.e_speed" type="number" class="input text-sm" />
          </label>
        </div>
      </div>

      <div class="mt-3">
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">描述</span>
          <textarea v-model="form.intro" class="input text-sm min-h-16 resize-y" />
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
