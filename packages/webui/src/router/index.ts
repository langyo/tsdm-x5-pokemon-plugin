import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', name: 'home', component: () => import('@/pages/HomePage') },
    { path: '/welcome', name: 'welcome', component: () => import('@/pages/WelcomePage') },
    { path: '/my-pokemon', name: 'myPokemon', component: () => import('@/pages/MyPokemonPage') },
    { path: '/shop', name: 'shop', component: () => import('@/pages/ShopPage') },
    { path: '/pokemon-center', name: 'pc', component: () => import('@/pages/PokemonCenterPage') },
    { path: '/adventure', name: 'adventure', component: () => import('@/pages/AdventurePage') },
    { path: '/inventory', name: 'inventory', component: () => import('@/pages/InventoryPage') },
    { path: '/storage', name: 'storage', component: () => import('@/pages/StoragePage') },
    { path: '/admin', name: 'admin', component: () => import('@/pages/admin/AdminLayout'), children: [
      { path: '', redirect: { name: 'adminConfig' } },
      { path: 'config', name: 'adminConfig', component: () => import('@/pages/admin/GlobalConfigPage') },
      { path: 'pokemon', name: 'adminPokemon', component: () => import('@/pages/admin/PokemonDataPage') },
      { path: 'items', name: 'adminItems', component: () => import('@/pages/admin/ItemDataPage') },
      { path: 'maps', name: 'adminMaps', component: () => import('@/pages/admin/MapDataPage') },
      { path: 'users', name: 'adminUsers', component: () => import('@/pages/admin/UserDataPage') },
      { path: 'evolution', name: 'adminEvolution', component: () => import('@/pages/admin/EvolutionDataPage') },
      { path: 'skills', name: 'adminSkills', component: () => import('@/pages/admin/SkillDataPage') },
      { path: 'sql', name: 'adminSql', component: () => import('@/pages/admin/SqlConsolePage') },
    ]},
    { path: '/:pathMatch(.*)*', name: 'notFound', component: () => import('@/pages/NotFoundPage') },
  ],
})

export default router
