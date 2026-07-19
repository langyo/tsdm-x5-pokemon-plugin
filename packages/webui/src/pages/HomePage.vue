<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { api } from '@/api/client'
import AppLayout from '@/components/layout/AppLayout.vue'

const topics = ref<any[]>([])
const players = ref<any[]>([])
const onlineCount = ref(0)

onMounted(async () => {
  try {
    const [t, p] = await Promise.all([
      api.get('topics', { action: 'list', limit: '6' }),
      api.get('user', { action: 'online_players' }),
    ])
    topics.value = t.data ?? []
    players.value = p.players ?? []
    onlineCount.value = p.count ?? 0
  } catch {}
})
</script>

<template>
  <AppLayout>
    <div class="grid grid-cols-3 gap-4">
      <div class="col-span-2">
        <div class="card mb-4">
          <h2 class="text-lg font-bold mb-3">最新话题</h2>
          <div v-if="topics.length" class="space-y-2">
            <a v-for="t in topics" :key="t.tid" :href="t.url" target="_blank"
               class="block p-2 rounded hover:bg-gray-50 text-sm">{{ t.subject }}</a>
          </div>
          <p v-else class="text-gray-400 text-sm">暂无话题</p>
        </div>
      </div>
      <div class="card">
        <h2 class="text-lg font-bold mb-3">在线玩家 ({{ onlineCount }})</h2>
        <div class="flex flex-wrap gap-2">
          <span v-for="p in players" :key="p.uid" class="text-xs bg-primary/10 text-primary px-2 py-1 rounded">
            {{ p.username }}
          </span>
        </div>
      </div>
    </div>
  </AppLayout>
</template>
