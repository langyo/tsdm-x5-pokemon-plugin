import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', name: 'home', component: () => import('@/pages/HomePage.vue') },
    { path: '/welcome', name: 'welcome', component: () => import('@/pages/WelcomePage.vue') },
    { path: '/my-pokemon', name: 'myPokemon', component: () => import('@/pages/MyPokemonPage.vue') },
    { path: '/shop', name: 'shop', component: () => import('@/pages/ShopPage.vue') },
    { path: '/pokemon-center', name: 'pc', component: () => import('@/pages/PokemonCenterPage.vue') },
    { path: '/adventure', name: 'adventure', component: () => import('@/pages/AdventurePage.vue') },
    { path: '/inventory', name: 'inventory', component: () => import('@/pages/InventoryPage.vue') },
    { path: '/storage', name: 'storage', component: () => import('@/pages/StoragePage.vue') },
    { path: '/admin', name: 'admin', component: () => import('@/pages/admin/AdminLayout.vue'), children: [
      { path: '', redirect: { name: 'adminConfig' } },
      { path: 'config', name: 'adminConfig', component: () => import('@/pages/admin/GlobalConfigPage.vue') },
      { path: 'pokemon', name: 'adminPokemon', component: () => import('@/pages/admin/PokemonDataPage.vue') },
      { path: 'items', name: 'adminItems', component: () => import('@/pages/admin/ItemDataPage.vue') },
      { path: 'maps', name: 'adminMaps', component: () => import('@/pages/admin/MapDataPage.vue') },
      { path: 'users', name: 'adminUsers', component: () => import('@/pages/admin/UserDataPage.vue') },
      { path: 'evolution', name: 'adminEvolution', component: () => import('@/pages/admin/EvolutionDataPage.vue') },
      { path: 'skills', name: 'adminSkills', component: () => import('@/pages/admin/SkillDataPage.vue') },
      { path: 'sql', name: 'adminSql', component: () => import('@/pages/admin/SqlConsolePage.vue') },
    ]},
    { path: '/:pathMatch(.*)*', name: 'notFound', component: () => import('@/pages/NotFoundPage.vue') },
  ],
})

export default router
