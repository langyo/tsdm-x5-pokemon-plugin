<script setup lang="ts">
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <header class="bg-white border-b px-6 py-3 flex items-center gap-4">
      <h1 class="text-lg font-bold text-primary">TSDM Pokemon</h1>
      <nav class="flex gap-3 ml-8">
        <RouterLink to="/my-pokemon" class="text-sm text-gray-600 hover:text-primary">个人中心</RouterLink>
        <RouterLink to="/shop" class="text-sm text-gray-600 hover:text-primary">商店</RouterLink>
        <RouterLink to="/pokemon-center" class="text-sm text-gray-600 hover:text-primary">宠物中心</RouterLink>
        <RouterLink to="/adventure" class="text-sm text-gray-600 hover:text-primary">野外冒险</RouterLink>
        <RouterLink to="/inventory" class="text-sm text-gray-600 hover:text-primary">背包</RouterLink>
        <RouterLink to="/storage" class="text-sm text-gray-600 hover:text-primary">仓库</RouterLink>
      </nav>
    </header>
    <main class="flex-1 p-4">
      <slot />
    </main>
    <footer class="bg-gray-100 text-center text-xs text-gray-400 py-3">
      TSDM Pokemon Plugin — Vue 3
    </footer>
  </div>
</template>
