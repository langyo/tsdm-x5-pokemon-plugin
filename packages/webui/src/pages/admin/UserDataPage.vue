<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useAdminStore } from '@/stores/admin'
import Modal from '@/components/common/Modal.vue'
import { api } from '@/api/client'
import { getTypeColor } from '@/utils/pokemon'

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

// Grant Pokemon
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

// Grant Item
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

// Edit Pokemon
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

// Edit Item
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

// Delete
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
</script>

<template>
  <div class="flex flex-col" style="height: calc(100vh - 6.5rem)">
    <div class="flex items-center justify-between mb-3">
      <h2 class="text-lg font-bold">用户数据管理</h2>
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
            <tr v-if="loading && items.length === 0">
              <td colspan="7" class="text-center text-gray-400 py-8">
                <span class="i-svg-spinners-3-dots-bounce text-lg" />
              </td>
            </tr>
            <tr v-else-if="!loading && items.length === 0">
              <td colspan="7" class="text-center text-gray-400 py-8">暂无数据</td>
            </tr>
            <tr
              v-for="row in items"
              :key="row.uid ?? row.id"
              class="border-b border-gray-100 hover:bg-gray-50 cursor-pointer transition-colors"
              @click="openDetail(row)"
            >
              <td class="px-2 py-2 font-mono text-xs">{{ row.uid ?? row.id }}</td>
              <td class="px-2 py-2 font-medium truncate">{{ row.username }}</td>
              <td class="px-2 py-2 text-right text-xs text-green-600">{{ row.wins ?? 0 }}</td>
              <td class="px-2 py-2 text-right text-xs text-red-500">{{ row.losses ?? 0 }}</td>
              <td class="px-2 py-2 text-right text-xs font-mono">{{ row.money ?? 0 }}</td>
              <td class="px-2 py-2 text-right text-xs font-mono">{{ row.exp ?? row.experience ?? 0 }}</td>
              <td class="px-2 py-2 text-center" @click.stop>
                <button class="px-2 py-1 text-xs rounded bg-primary/10 text-primary hover:bg-primary/20 transition-colors" @click="openDetail(row)">
                  详情
                </button>
              </td>
            </tr>
          </tbody>
          <tfoot v-if="loading && items.length > 0">
            <tr>
              <td colspan="7" class="text-center text-gray-400 py-2">
                <span class="i-svg-spinners-3-dots-bounce text-sm" />
              </td>
            </tr>
          </tfoot>
        </table>
      </div>
    </div>

    <div class="fixed bottom-0 left-48 right-0 bg-white border-t border-gray-200 px-4 py-2 flex items-center justify-center gap-3 z-20 shadow-lg">
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
              <span class="text-xs text-gray-500">UID</span>
              <input v-model="filterForm.uid" type="text" class="input w-32 text-sm" placeholder="用户UID" />
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs text-gray-500">用户名</span>
              <input v-model="filterForm.username" type="text" class="input w-40 text-sm" placeholder="用户名关键词" />
            </label>
            <button class="btn-primary text-sm" @click="doApplyFilter">应用</button>
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="resetFilter">重置</button>
          </div>
        </div>
      </div>
    </Teleport>

    <!-- Detail Modal -->
    <Modal :open="detailOpen" :title="`用户详情 - ${detailUser?.username ?? ''}`" wide @close="closeDetail">
      <div v-if="detailError" class="text-red-500 text-sm mb-3">{{ detailError }}</div>

      <div class="flex gap-4 mb-4">
        <button
          class="px-3 py-1.5 text-sm rounded transition-colors"
          :class="detailTab === 'pokemon' ? 'bg-primary text-white' : 'bg-gray-100 hover:bg-gray-200'"
          @click="detailTab = 'pokemon'"
        >
          宠物 ({{ userPokemon.length }})
        </button>
        <button
          class="px-3 py-1.5 text-sm rounded transition-colors"
          :class="detailTab === 'items' ? 'bg-primary text-white' : 'bg-gray-100 hover:bg-gray-200'"
          @click="detailTab = 'items'"
        >
          道具 ({{ userItems.length }})
        </button>
      </div>

      <div v-if="detailLoading" class="text-center text-gray-400 py-4">
        <span class="i-svg-spinners-3-dots-bounce text-lg" />
      </div>

      <!-- Pokemon Tab -->
      <template v-else-if="detailTab === 'pokemon'">
        <div class="flex items-center justify-between mb-3">
          <h4 class="text-sm font-bold">宠物列表</h4>
          <button class="btn-primary text-xs" @click="openGrantPkm">+ 发放宠物</button>
        </div>
        <div v-if="userPokemon.length === 0" class="text-center text-gray-400 py-4 text-sm">暂无宠物</div>
        <table v-else class="w-full text-sm">
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
            <tr v-for="pkm in userPokemon" :key="pkm.id" class="border-b border-gray-100">
              <td class="px-2 py-1.5 font-mono text-xs">{{ pkm.id }}</td>
              <td class="px-2 py-1.5 text-xs">{{ pkm.species ?? pkm.pid }}</td>
              <td class="px-2 py-1.5 text-xs font-medium">{{ pkm.name }}</td>
              <td class="px-2 py-1.5 text-xs text-right">Lv.{{ pkm.level }}</td>
              <td class="px-2 py-1.5 text-xs text-right">{{ pkm.hp }}/{{ pkm.maxhp ?? pkm.max_hp }}</td>
              <td class="px-2 py-1.5 text-center">
                <div class="flex items-center justify-center gap-1">
                  <button class="px-1.5 py-0.5 text-[10px] rounded bg-primary/10 text-primary hover:bg-primary/20" @click="openEditPkm(pkm)">编辑</button>
                  <button class="px-1.5 py-0.5 text-[10px] rounded bg-red-50 text-red-500 hover:bg-red-100" @click="confirmDelete('pokemon', pkm)">删除</button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </template>

      <!-- Items Tab -->
      <template v-else>
        <div class="flex items-center justify-between mb-3">
          <h4 class="text-sm font-bold">道具列表</h4>
          <button class="btn-primary text-xs" @click="openGrantItem">+ 发放道具</button>
        </div>
        <div v-if="userItems.length === 0" class="text-center text-gray-400 py-4 text-sm">暂无道具</div>
        <table v-else class="w-full text-sm">
          <thead>
            <tr class="border-b border-gray-200">
              <th class="px-2 py-1.5 text-left text-xs">ID</th>
              <th class="px-2 py-1.5 text-left text-xs">名称</th>
              <th class="px-2 py-1.5 text-right text-xs">数量</th>
              <th class="px-2 py-1.5 text-center text-xs w-28">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="item in userItems" :key="item.id" class="border-b border-gray-100">
              <td class="px-2 py-1.5 font-mono text-xs">{{ item.id }}</td>
              <td class="px-2 py-1.5 text-xs font-medium">{{ item.name }}</td>
              <td class="px-2 py-1.5 text-xs text-right">x{{ item.quantity ?? item.num }}</td>
              <td class="px-2 py-1.5 text-center">
                <div class="flex items-center justify-center gap-1">
                  <button class="px-1.5 py-0.5 text-[10px] rounded bg-primary/10 text-primary hover:bg-primary/20" @click="openEditItem(item)">编辑</button>
                  <button class="px-1.5 py-0.5 text-[10px] rounded bg-red-50 text-red-500 hover:bg-red-100" @click="confirmDelete('item', item)">删除</button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </template>
    </Modal>

    <!-- Grant Pokemon Modal -->
    <Modal :open="grantPkmOpen" title="发放宠物" @close="closeGrantPkm">
      <div class="space-y-3">
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">搜索宠物物种</span>
          <div class="relative">
            <input
              v-model="grantPkmSearch"
              type="text"
              class="input text-sm w-full"
              placeholder="输入名称搜索..."
              @input="searchPokemonForGrant"
            />
            <div v-if="grantPkmResults.length > 0" class="absolute top-full left-0 right-0 bg-white border border-gray-200 rounded-lg shadow-lg max-h-40 overflow-y-auto z-50">
              <div
                v-for="r in grantPkmResults"
                :key="r.id"
                class="px-3 py-1.5 text-sm hover:bg-primary/10 cursor-pointer"
                @click="selectPokemonForGrant(r)"
              >
                <span class="font-mono text-xs text-gray-400 mr-2">#{{ r.id }}</span>
                {{ r.name }}
              </div>
            </div>
          </div>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">名称</span>
          <input v-model="grantPkmForm.name" type="text" class="input text-sm" placeholder="精灵昵称" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">等级</span>
          <input v-model.number="grantPkmForm.level" type="number" min="1" max="100" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">HP</span>
          <input v-model.number="grantPkmForm.hp" type="number" min="0" class="input text-sm" />
        </label>
      </div>

      <div v-if="grantPkmError" class="mt-3 text-red-500 text-sm">{{ grantPkmError }}</div>

      <div class="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-gray-100">
        <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="closeGrantPkm">取消</button>
        <button class="btn-primary text-sm" :disabled="grantPkmSaving" @click="submitGrantPkm">
          <span v-if="grantPkmSaving" class="i-svg-spinners-3-dots-bounce text-white" />
          <span v-else>发放</span>
        </button>
      </div>
    </Modal>

    <!-- Grant Item Modal -->
    <Modal :open="grantItemOpen" title="发放道具" @close="closeGrantItem">
      <div class="space-y-3">
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">搜索道具</span>
          <div class="relative">
            <input
              v-model="grantItemSearch"
              type="text"
              class="input text-sm w-full"
              placeholder="输入名称搜索..."
              @input="searchItemForGrant"
            />
            <div v-if="grantItemResults.length > 0" class="absolute top-full left-0 right-0 bg-white border border-gray-200 rounded-lg shadow-lg max-h-40 overflow-y-auto z-50">
              <div
                v-for="r in grantItemResults"
                :key="r.id"
                class="px-3 py-1.5 text-sm hover:bg-primary/10 cursor-pointer"
                @click="selectItemForGrant(r)"
              >
                <span class="font-mono text-xs text-gray-400 mr-2">#{{ r.id }}</span>
                {{ r.name }}
              </div>
            </div>
          </div>
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">数量</span>
          <input v-model.number="grantItemForm.quantity" type="number" min="1" class="input text-sm" />
        </label>
      </div>

      <div v-if="grantItemError" class="mt-3 text-red-500 text-sm">{{ grantItemError }}</div>

      <div class="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-gray-100">
        <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="closeGrantItem">取消</button>
        <button class="btn-primary text-sm" :disabled="grantItemSaving" @click="submitGrantItem">
          <span v-if="grantItemSaving" class="i-svg-spinners-3-dots-bounce text-white" />
          <span v-else>发放</span>
        </button>
      </div>
    </Modal>

    <!-- Edit Pokemon Modal -->
    <Modal :open="editPkmOpen" title="编辑宠物" @close="closeEditPkm">
      <div class="grid grid-cols-2 gap-3">
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">物种ID</span>
          <input v-model.number="editPkmForm.species" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">名称</span>
          <input v-model="editPkmForm.name" type="text" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">等级</span>
          <input v-model.number="editPkmForm.level" type="number" min="1" max="100" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">HP</span>
          <input v-model.number="editPkmForm.hp" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">ATK</span>
          <input v-model.number="editPkmForm.atk" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">DEF</span>
          <input v-model.number="editPkmForm.def" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">SPATK</span>
          <input v-model.number="editPkmForm.spatk" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">SPDEF</span>
          <input v-model.number="editPkmForm.spdef" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">SPEED</span>
          <input v-model.number="editPkmForm.sd" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">经验</span>
          <input v-model.number="editPkmForm.experience" type="number" class="input text-sm" />
        </label>
      </div>

      <div v-if="editPkmError" class="mt-3 text-red-500 text-sm">{{ editPkmError }}</div>

      <div class="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-gray-100">
        <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="closeEditPkm">取消</button>
        <button class="btn-primary text-sm" :disabled="editPkmSaving" @click="submitEditPkm">
          <span v-if="editPkmSaving" class="i-svg-spinners-3-dots-bounce text-white" />
          <span v-else>保存</span>
        </button>
      </div>
    </Modal>

    <!-- Edit Item Modal -->
    <Modal :open="editItemOpen" title="编辑道具" @close="closeEditItem">
      <div class="space-y-3">
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">道具ID</span>
          <input v-model.number="editItemForm.itemId" type="number" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">名称</span>
          <input v-model="editItemForm.name" type="text" class="input text-sm" />
        </label>
        <label class="flex flex-col gap-1">
          <span class="text-xs text-gray-500">数量</span>
          <input v-model.number="editItemForm.quantity" type="number" min="0" class="input text-sm" />
        </label>
      </div>

      <div v-if="editItemError" class="mt-3 text-red-500 text-sm">{{ editItemError }}</div>

      <div class="flex items-center justify-end gap-3 mt-4 pt-4 border-t border-gray-100">
        <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="closeEditItem">取消</button>
        <button class="btn-primary text-sm" :disabled="editItemSaving" @click="submitEditItem">
          <span v-if="editItemSaving" class="i-svg-spinners-3-dots-bounce text-white" />
          <span v-else>保存</span>
        </button>
      </div>
    </Modal>

    <!-- Delete Confirmation Dialog -->
    <Teleport to="body">
      <div v-if="deleteConfirm" class="fixed inset-0 z-50 flex items-center justify-center">
        <div class="absolute inset-0 bg-black/50" @click="deleteConfirm = null" />
        <div class="relative bg-white rounded-xl shadow-lg border border-border p-6 w-full max-w-sm z-10 mx-4">
          <h3 class="text-base font-bold mb-2">确认删除</h3>
          <p class="text-sm text-gray-600 mb-1">
            确认删除此条记录？
          </p>
          <p v-if="deleteConfirm.type === 'pokemon'" class="text-sm text-gray-500 mb-4">
            宠物: {{ deleteConfirm.data.name }} (ID: {{ deleteConfirm.data.id }})
          </p>
          <p v-else class="text-sm text-gray-500 mb-4">
            道具: {{ deleteConfirm.data.name }} (ID: {{ deleteConfirm.data.id }})
          </p>
          <div class="flex items-center justify-end gap-3">
            <button class="btn text-sm border border-gray-300 hover:bg-gray-100" @click="deleteConfirm = null">取消</button>
            <button class="px-4 py-1.5 text-sm rounded bg-red-500 text-white hover:bg-red-600" @click="executeDelete">删除</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>
