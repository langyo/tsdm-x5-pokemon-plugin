import { ref, computed, reactive } from 'vue'
import { api } from '@/api/client'

export function useAdminStore(entity: string) {
  const items = ref<any[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const total = ref(0)
  const filters = ref<Record<string, any>>({})

  const pageSize = 50
  const _page = ref(0)
  const hasMore = computed(() => items.value.length < total.value)

  async function reload() {
    loading.value = true
    error.value = null
    try {
      const [countRes, listRes] = await Promise.all([
        api.admin(`count::${entity}`, filters.value),
        api.admin(`list::${entity}`, { ...filters.value, offset: 0, limit: pageSize }),
      ])
      total.value = countRes.count ?? countRes.total ?? 0
      items.value = listRes.data ?? listRes.list ?? listRes.items ?? []
      _page.value = 0
    } catch (e: any) {
      error.value = e.message
    } finally {
      loading.value = false
    }
  }

  async function loadMore() {
    if (loading.value || !hasMore.value) return
    loading.value = true
    error.value = null
    try {
      const nextPage = _page.value + 1
      const offset = nextPage * pageSize
      const res = await api.admin(`list::${entity}`, { ...filters.value, offset, limit: pageSize })
      const data = res.data ?? res.list ?? res.items ?? []
      items.value.push(...data)
      _page.value = nextPage
    } catch (e: any) {
      error.value = e.message
    } finally {
      loading.value = false
    }
  }

  async function save(data: Record<string, any>) {
    const action = data.id ? `set::${entity}` : `insert::${entity}`
    await api.admin(action, { data: JSON.stringify(data) })
    await reload()
  }

  async function remove(id: number | string) {
    await api.admin(`delete::${entity}`, { id: String(id) })
    await reload()
  }

  function applyFilters(f: Record<string, any>) {
    filters.value = { ...f }
    reload()
  }

  return { items, loading, error, total, hasMore, reload, loadMore, save, remove, applyFilters, filters }
}

export const useAdminPokemonStore = () => useAdminStore('pokemon')
export const useAdminItemStore = () => useAdminStore('item')
export const useAdminMapStore = () => useAdminStore('map')
