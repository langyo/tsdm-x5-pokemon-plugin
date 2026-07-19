import { defineComponent, ref, onMounted } from 'vue'
import { api } from '@/api/client'
import './GlobalConfigPage.scss'

interface GlobalConfig {
  is_open: number
  is_enable_catch: number
  is_enable_pvp: number
  medical_price: number
  egg_price: number
  shiny_rate: number
  exp_rate: number
}

export default defineComponent({
  name: 'GlobalConfigPage',
  setup() {
    const config = ref<GlobalConfig>({
      is_open: 0,
      is_enable_catch: 1,
      is_enable_pvp: 0,
      medical_price: 100,
      egg_price: 500,
      shiny_rate: 1,
      exp_rate: 1,
    })

    const loading = ref(false)
    const saving = ref(false)
    const error = ref<string | null>(null)
    const saveMessage = ref('')

    const sql = ref('')
    const sqlResult = ref<string | null>(null)
    const sqlLoading = ref(false)
    const sqlError = ref<string | null>(null)

    async function loadConfig() {
      loading.value = true
      error.value = null
      try {
        const res = await api.admin('list::global_config')
        const data = res.data ?? res.config ?? res
        if (data) {
          if (Array.isArray(data) && data.length > 0) {
            const c = data[0]
            config.value = {
              is_open: Number(c.is_open ?? 0),
              is_enable_catch: Number(c.is_enable_catch ?? 1),
              is_enable_pvp: Number(c.is_enable_pvp ?? 0),
              medical_price: Number(c.medical_price ?? 100),
              egg_price: Number(c.egg_price ?? 500),
              shiny_rate: Number(c.shiny_rate ?? 1),
              exp_rate: Number(c.exp_rate ?? 1),
            }
          } else if (typeof data === 'object') {
            config.value = {
              is_open: Number(data.is_open ?? 0),
              is_enable_catch: Number(data.is_enable_catch ?? 1),
              is_enable_pvp: Number(data.is_enable_pvp ?? 0),
              medical_price: Number(data.medical_price ?? 100),
              egg_price: Number(data.egg_price ?? 500),
              shiny_rate: Number(data.shiny_rate ?? 1),
              exp_rate: Number(data.exp_rate ?? 1),
            }
          }
        }
      } catch (e: any) {
        error.value = e.message
      } finally {
        loading.value = false
      }
    }

    async function saveConfig() {
      saving.value = true
      saveMessage.value = ''
      try {
        await api.admin('set::global_config', { data: JSON.stringify(config.value) })
        saveMessage.value = '保存成功'
      } catch (e: any) {
        saveMessage.value = '保存失败: ' + e.message
      } finally {
        saving.value = false
      }
    }

    async function executeSql() {
      if (!sql.value.trim()) return
      sqlLoading.value = true
      sqlError.value = null
      sqlResult.value = null
      try {
        const res = await api.admin('run::sql_console', { sql: sql.value })
        sqlResult.value = JSON.stringify(res, null, 2)
      } catch (e: any) {
        sqlError.value = e.message
      } finally {
        sqlLoading.value = false
      }
    }

    onMounted(() => {
      loadConfig()
    })

    return () => (
      <div class="space-y-4">
        {loading.value ? (
          <div class="card flex items-center justify-center py-8">
            <span class="i-svg-spinners-3-dots-bounce text-xl text-primary" />
            <span class="ml-2 text-gray-500">加载配置中...</span>
          </div>
        ) : (
          <>
            <div class="card">
              <h2 class="text-lg font-bold mb-4">系统配置</h2>
              <div class="space-y-3">
                <div class="flex items-center justify-between py-2 border-b border-gray-100">
                  <span class="text-sm font-medium">启用插件</span>
                  <label class="relative inline-flex items-center cursor-pointer">
                    <input
                      type="checkbox"
                      checked={config.value.is_open === 1}
                      onChange={(e: any) => config.value.is_open = e.target.checked ? 1 : 0}
                      class="sr-only peer"
                    />
                    <div class="w-9 h-5 bg-gray-200 rounded-full peer-checked:bg-primary peer-focus:ring-2 peer-focus:ring-primary/30 transition-colors" />
                    <div class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full peer-checked:translate-x-4 transition-transform" />
                  </label>
                </div>
                <div class="flex items-center justify-between py-2 border-b border-gray-100">
                  <span class="text-sm font-medium">启用捕捉</span>
                  <label class="relative inline-flex items-center cursor-pointer">
                    <input
                      type="checkbox"
                      checked={config.value.is_enable_catch === 1}
                      onChange={(e: any) => config.value.is_enable_catch = e.target.checked ? 1 : 0}
                      class="sr-only peer"
                    />
                    <div class="w-9 h-5 bg-gray-200 rounded-full peer-checked:bg-primary peer-focus:ring-2 peer-focus:ring-primary/30 transition-colors" />
                    <div class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full peer-checked:translate-x-4 transition-transform" />
                  </label>
                </div>
                <div class="flex items-center justify-between py-2 border-b border-gray-100">
                  <span class="text-sm font-medium">启用 PVP</span>
                  <label class="relative inline-flex items-center cursor-pointer">
                    <input
                      type="checkbox"
                      checked={config.value.is_enable_pvp === 1}
                      onChange={(e: any) => config.value.is_enable_pvp = e.target.checked ? 1 : 0}
                      class="sr-only peer"
                    />
                    <div class="w-9 h-5 bg-gray-200 rounded-full peer-checked:bg-primary peer-focus:ring-2 peer-focus:ring-primary/30 transition-colors" />
                    <div class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full peer-checked:translate-x-4 transition-transform" />
                  </label>
                </div>
              </div>
            </div>

            <div class="card">
              <h2 class="text-lg font-bold mb-4">经济配置</h2>
              <div class="space-y-3">
                <div class="flex items-center justify-between py-2 border-b border-gray-100">
                  <label class="text-sm font-medium">治疗费用</label>
                  <input
                    type="number"
                    min="0"
                    value={config.value.medical_price}
                    onInput={(e: any) => config.value.medical_price = Number(e.target.value)}
                    class="input w-32 text-right"
                  />
                </div>
                <div class="flex items-center justify-between py-2 border-b border-gray-100">
                  <label class="text-sm font-medium">精灵蛋价格</label>
                  <input
                    type="number"
                    min="0"
                    value={config.value.egg_price}
                    onInput={(e: any) => config.value.egg_price = Number(e.target.value)}
                    class="input w-32 text-right"
                  />
                </div>
              </div>
            </div>

            <div class="card">
              <h2 class="text-lg font-bold mb-4">倍率配置</h2>
              <div class="space-y-3">
                <div class="flex items-center justify-between py-2 border-b border-gray-100">
                  <label class="text-sm font-medium">闪光倍率</label>
                  <input
                    type="number"
                    step="0.1"
                    min="0.1"
                    value={config.value.shiny_rate}
                    onInput={(e: any) => config.value.shiny_rate = Number(e.target.value)}
                    class="input w-32 text-right"
                  />
                </div>
                <div class="flex items-center justify-between py-2 border-b border-gray-100">
                  <label class="text-sm font-medium">经验倍率</label>
                  <input
                    type="number"
                    step="0.1"
                    min="0.1"
                    value={config.value.exp_rate}
                    onInput={(e: any) => config.value.exp_rate = Number(e.target.value)}
                    class="input w-32 text-right"
                  />
                </div>
              </div>
            </div>

            <div class="flex items-center gap-3">
              <button class="btn btn-primary" disabled={saving.value} onClick={saveConfig}>
                {saving.value ? (
                  <span class="i-svg-spinners-3-dots-bounce text-white" />
                ) : (
                  <span>保存配置</span>
                )}
              </button>
              {saveMessage.value && (
                <span class={['text-sm', saveMessage.value.includes('成功') ? 'text-green-600' : 'text-red-500'].join(' ')}>
                  {saveMessage.value}
                </span>
              )}
            </div>

            <div class="card">
              <h2 class="text-lg font-bold mb-4">SQL 控制台</h2>
              <textarea
                value={sql.value}
                onInput={(e: any) => sql.value = e.target.value}
                class="input w-full min-h-24 font-mono text-sm resize-y"
                placeholder="输入 SQL 语句..."
              />
              <div class="flex items-center gap-3 mt-3">
                <button class="btn-primary" disabled={sqlLoading.value || !sql.value.trim()} onClick={executeSql}>
                  {sqlLoading.value ? (
                    <span class="i-svg-spinners-3-dots-bounce text-white" />
                  ) : (
                    <span>执行</span>
                  )}
                </button>
              </div>
              {sqlError.value && (
                <div class="mt-3 p-3 bg-red-50 border border-red-200 rounded-lg text-red-600 text-sm font-mono">
                  {sqlError.value}
                </div>
              )}
              {sqlResult.value && (
                <pre class="mt-3 p-3 bg-gray-900 text-green-400 rounded-lg text-xs overflow-x-auto max-h-80"><code>{sqlResult.value}</code></pre>
              )}
              {!sqlError.value && !sqlResult.value && sqlLoading.value && (
                <div class="mt-3 text-sm text-gray-400">执行中...</div>
              )}
            </div>
          </>
        )}

        {error.value && (
          <div class="card bg-red-50 border-red-200">
            <p class="text-red-600 text-sm">{error.value}</p>
            <button class="btn-primary mt-2 text-sm" onClick={loadConfig}>重试</button>
          </div>
        )}
      </div>
    )
  },
})
