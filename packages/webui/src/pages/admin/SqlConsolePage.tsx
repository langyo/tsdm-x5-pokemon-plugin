import { defineComponent, ref, computed } from 'vue'
import { api } from '@/api/client'
import './SqlConsolePage.scss'

export default defineComponent({
  name: 'SqlConsolePage',
  setup() {
    const sql = ref('')
    const executing = ref(false)
    const error = ref<string | null>(null)
    const columns = ref<string[]>([])
    const results = ref<Record<string, any>[]>([])
    const queryTime = ref<number | null>(null)

    const history = ref<{ sql: string; time: number }[]>([])

    async function execute() {
      const trimmed = sql.value.trim()
      if (!trimmed) return

      executing.value = true
      error.value = null
      results.value = []
      columns.value = []
      queryTime.value = null

      const startTime = performance.now()
      try {
        const res = await api.admin('run::sql_console', { sql: trimmed })
        const endTime = performance.now()
        queryTime.value = Math.round(endTime - startTime)

        const data = res.data ?? res.rows ?? res.results ?? res

        if (Array.isArray(data) && data.length > 0) {
          columns.value = Object.keys(data[0])
          results.value = data
        } else if (typeof data === 'object' && data !== null) {
          if (data.columns && data.rows) {
            columns.value = data.columns
            results.value = data.rows
          } else if (data.message) {
            columns.value = ['Message']
            results.value = [{ Message: data.message }]
          } else {
            columns.value = Object.keys(data)
            results.value = [data]
          }
        } else if (typeof data === 'string') {
          columns.value = ['Result']
          results.value = [{ Result: data }]
        } else {
          columns.value = ['Result']
          results.value = [{ Result: String(res.success ?? 'OK') }]
        }

        history.value.unshift({ sql: trimmed, time: queryTime.value ?? 0 })
        if (history.value.length > 50) {
          history.value = history.value.slice(0, 50)
        }
      } catch (e: any) {
        error.value = e.message
      } finally {
        executing.value = false
      }
    }

    function runFromHistory(item: { sql: string }) {
      sql.value = item.sql
      execute()
    }

    function clear() {
      sql.value = ''
      error.value = null
      columns.value = []
      results.value = []
      queryTime.value = null
    }

    function clearHistory() {
      history.value = []
    }

    function onKeydown(e: KeyboardEvent) {
      if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
        e.preventDefault()
        execute()
      }
    }

    return () => (
      <div class="flex flex-col" style="height: calc(100vh - 6.5rem)">
        <div class="flex items-center justify-between mb-3">
          <h2 class="text-lg font-bold">SQL 控制台</h2>
          {queryTime.value !== null && (
            <span class="text-xs text-gray-400 font-mono">{queryTime.value}ms</span>
          )}
        </div>

        <div class="flex gap-3 flex-1 min-h-0">
          <div class="flex-1 flex flex-col min-h-0">
            <div class="card p-0 overflow-hidden mb-3">
              <textarea
                value={sql.value}
                onInput={(e: any) => sql.value = e.target.value}
                onKeydown={onKeydown}
                class="w-full min-h-40 p-4 font-mono text-sm bg-gray-900 text-green-400 resize-y outline-none border-none"
                placeholder="输入 SQL 语句..."
              />
            </div>

            <div class="flex items-center gap-2 mb-3">
              <button class="btn-primary text-sm" disabled={executing.value || !sql.value.trim()} onClick={execute}>
                {executing.value ? (
                  <span class="i-svg-spinners-3-dots-bounce text-white" />
                ) : (
                  <span>执行 (Ctrl+Enter)</span>
                )}
              </button>
              <button class="btn text-sm border border-gray-300 hover:bg-gray-100" onClick={clear}>清空</button>
            </div>

            {error.value && (
              <div class="card bg-red-50 border-red-200 mb-3">
                <p class="text-red-600 text-sm font-mono whitespace-pre-wrap">{error.value}</p>
              </div>
            )}

            {results.value.length > 0 ? (
              <div class="card flex-1 flex flex-col min-h-0 p-0 overflow-hidden">
                <div class="px-3 py-2 text-xs text-gray-400 border-b border-gray-100">
                  返回 {results.value.length} 条记录
                </div>
                <div class="overflow-auto flex-1">
                  <table class="w-full text-sm">
                    <thead class="sticky top-0 bg-gray-50 z-10">
                      <tr class="border-b border-gray-200">
                        {columns.value.map((col) => (
                          <th
                            key={col}
                            class="px-3 py-2 text-left text-xs font-mono font-bold whitespace-nowrap"
                          >
                            {col}
                          </th>
                        ))}
                      </tr>
                    </thead>
                    <tbody>
                      {results.value.map((row, i) => (
                        <tr key={i} class="border-b border-gray-100 hover:bg-gray-50">
                          {columns.value.map((col) => (
                            <td
                              key={col}
                              class="px-3 py-1.5 text-xs font-mono whitespace-nowrap max-w-80 truncate"
                            >
                              {row[col] ?? 'NULL'}
                            </td>
                          ))}
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>
            ) : (!executing.value && sql.value.trim() && !error.value) ? (
              <div class="card flex items-center justify-center py-8">
                <p class="text-gray-400 text-sm">点击执行或按 Ctrl+Enter</p>
              </div>
            ) : null}
          </div>

          <div class="w-64 flex-shrink-0">
            <div class="card p-0 overflow-hidden">
              <div class="px-3 py-2 flex items-center justify-between border-b border-gray-100">
                <h3 class="text-sm font-bold">查询历史</h3>
                {history.value.length > 0 && (
                  <button
                    class="text-xs text-gray-400 hover:text-red-500"
                    onClick={clearHistory}
                  >
                    清空
                  </button>
                )}
              </div>
              {history.value.length === 0 ? (
                <div class="px-3 py-4 text-xs text-gray-400 text-center">
                  暂无查询记录
                </div>
              ) : (
                <div class="overflow-y-auto max-h-full">
                  {history.value.map((item, i) => (
                    <div
                      key={i}
                      class="px-3 py-2 border-b border-gray-50 hover:bg-gray-50 cursor-pointer transition-colors"
                      onClick={() => runFromHistory(item)}
                    >
                      <p class="text-xs font-mono text-gray-700 truncate">{item.sql}</p>
                      <p class="text-[10px] text-gray-400 mt-0.5">{item.time}ms</p>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    )
  },
})
