<script setup lang="ts">
import { RouterView, RouterLink, useRoute } from 'vue-router'

const route = useRoute()

const navItems = [
  { path: '/admin/config', name: '全局配置' },
  { path: '/admin/pokemon', name: '宠物数据' },
  { path: '/admin/items', name: '道具数据' },
  { path: '/admin/maps', name: '地图设定' },
  { path: '/admin/users', name: '用户数据' },
  { path: '/admin/evolution', name: '进化路线' },
  { path: '/admin/skills', name: '技能数据' },
  { path: '/admin/sql', name: 'SQL 控制台' },
]
</script>

<template>
  <div class="min-h-screen flex">
    <aside class="w-48 bg-gray-900 text-white flex-shrink-0">
      <div class="px-4 py-3 text-sm font-bold border-b border-gray-700">管理后台</div>
      <nav class="py-2">
        <RouterLink v-for="item in navItems" :key="item.path" :to="item.path"
          class="block px-4 py-2 text-sm hover:bg-gray-800"
          :class="{ 'bg-gray-800 text-primary': route.path === item.path }">
          {{ item.name }}
        </RouterLink>
      </nav>
    </aside>
    <main class="flex-1 p-6 bg-gray-50">
      <RouterView />
    </main>
  </div>
</template>
