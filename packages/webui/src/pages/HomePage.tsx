import { defineComponent, onMounted, ref } from 'vue'
import { api } from '@/api/client'
import { useUserStore } from '@/stores'
import AppLayout from '@/components/layout/AppLayout'
import Card from '@/components/common/Card'
import LoadingOverlay from '@/components/common/LoadingOverlay'

export default defineComponent({
  name: 'HomePage',
  setup() {
    const userStore = useUserStore()

    const topics = ref<any[]>([])
    const players = ref<any[]>([])
    const onlineCount = ref(0)
    const loading = ref(true)
    const error = ref<string | null>(null)

    function reload() {
      location.reload()
    }

    onMounted(async () => {
      try {
        const [t, p] = await Promise.all([
          api.get<any>('topics', { action: 'list', limit: '6' }),
          api.get<any>('user', { action: 'online_players' }),
        ])
        topics.value = t.data ?? t.topics ?? []
        players.value = p.players ?? []
        onlineCount.value = p.count ?? p.players?.length ?? 0
      } catch (e: any) {
        error.value = e.message || '加载失败'
      } finally {
        loading.value = false
      }
    })

    return () => (
      <AppLayout>
        {loading.value && <LoadingOverlay message="加载首页..." />}

        {error.value ? (
          <div class="text-center py-16">
            <p class="text-red-500 mb-4">{error.value}</p>
            <button
              class="bg-primary text-white px-4 py-2 rounded-lg text-sm"
              onClick={reload}
            >
              重试
            </button>
          </div>
        ) : !loading.value ? (
          <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
            <div class="lg:col-span-2 space-y-4">
              <Card title="游戏公告">
                {userStore.profile?.announcement ? (
                  <div class="text-sm text-gray-700 leading-relaxed" innerHTML={userStore.profile.announcement} />
                ) : (
                  <p class="text-gray-400 text-sm">暂无公告</p>
                )}
              </Card>

              <Card title="最新话题">
                {topics.value.length ? (
                  <div class="space-y-1">
                    {topics.value.map((t: any) => (
                      <a
                        key={t.tid}
                        href={t.url || `forum.php?mod=viewthread&tid=${t.tid}`}
                        target="_blank"
                        class="block p-2 rounded hover:bg-gray-50 text-sm text-gray-700 hover:text-primary transition-colors"
                      >
                        {t.subject}
                      </a>
                    ))}
                  </div>
                ) : (
                  <p class="text-gray-400 text-sm">暂无话题</p>
                )}
              </Card>
            </div>

            <div class="space-y-4">
              <Card title={`在线玩家 (${onlineCount.value})`}>
                {players.value.length ? (
                  <div class="flex flex-wrap gap-2">
                    {players.value.map((p: any) => (
                      <span
                        key={p.uid}
                        class="text-xs bg-primary/10 text-primary px-2 py-1 rounded"
                      >
                        {p.username}
                      </span>
                    ))}
                  </div>
                ) : (
                  <p class="text-gray-400 text-sm">暂无在线玩家</p>
                )}
              </Card>

              <Card title="训练师信息">
                {userStore.profile ? (
                  <div class="space-y-2 text-sm">
                    <div class="flex justify-between">
                      <span class="text-gray-500">用户名</span>
                      <span class="font-medium">{userStore.profile.username}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-500">金币</span>
                      <span class="font-medium text-amber-500">{userStore.money}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-500">状态</span>
                      <span class={`font-medium ${userStore.isInBattle ? 'text-red-500' : 'text-green-500'}`}>
                        {userStore.isInBattle ? '战斗中' : '空闲'}
                      </span>
                    </div>
                  </div>
                ) : (
                  <p class="text-gray-400 text-sm">未登录</p>
                )}
              </Card>
            </div>
          </div>
        ) : null}
      </AppLayout>
    )
  }
})
