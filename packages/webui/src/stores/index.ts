import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useAppStore = defineStore('app', () => {
  const loading = ref(false)
  const loadingMessage = ref('')

  function startLoading(msg = '加载中...') {
    loading.value = true
    loadingMessage.value = msg
  }
  function stopLoading() {
    loading.value = false
  }

  return { loading, loadingMessage, startLoading, stopLoading }
})

export const useUserStore = defineStore('user', () => {
  const profile = ref<any>(null)
  const inventoryStats = ref<any>(null)
  const profileLoaded = ref(false)
  const inventoryLoaded = ref(false)

  const money = computed(() => profile.value?.money ?? 0)
  const isInBattle = computed(() => (profile.value?.npcid ?? 0) > 0)

  async function fetchProfile() {
    const { api } = await import('@/api/client')
    const data: any = await api.get('user', { action: 'profile' })
    profile.value = data
    profileLoaded.value = true
  }

  async function fetchInventoryStats() {
    const { api } = await import('@/api/client')
    const data: any = await api.get('user', { action: 'inventory_stats' })
    inventoryStats.value = data
    inventoryLoaded.value = true
  }

  return { profile, inventoryStats, profileLoaded, inventoryLoaded, money, isInBattle, fetchProfile, fetchInventoryStats }
})

export const usePokemonStore = defineStore('pokemon', () => {
  const list = ref<any[]>([])
  const loading = ref(false)
  const loaded = ref(false)
  const error = ref<string | null>(null)

  const bagPokemons = computed(() => list.value.filter((p: any) => p.site === 1 || p.site === 2))
  const storagePokemons = computed(() => list.value.filter((p: any) => p.site === 3))
  const firstPokemon = computed(() => list.value.find((p: any) => p.site === 1))

  function getById(id: number) { return list.value.find((p: any) => p.id === id) }

  async function fetchList() {
    loading.value = true
    try {
      const { api } = await import('@/api/client')
      const data: any = await api.get('pokemon', { action: 'list' })
      list.value = data.pokemons ?? data.data ?? []
      loaded.value = true
    } catch (e: any) {
      error.value = e.message
    } finally {
      loading.value = false
    }
  }

  return { list, loading, loaded, error, bagPokemons, storagePokemons, firstPokemon, getById, fetchList }
})

export const useBattleStore = defineStore('battle', () => {
  const scene = ref<any>(null)
  const loading = ref(false)
  const isActive = computed(() => scene.value?.status === 'Active')

  function setScene(s: any) { scene.value = s }
  function clear() { scene.value = null }

  return { scene, loading, isActive, setScene, clear }
})
