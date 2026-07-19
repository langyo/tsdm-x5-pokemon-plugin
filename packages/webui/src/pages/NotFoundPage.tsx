import { defineComponent } from 'vue'

export default defineComponent({
  name: 'NotFoundPage',
  setup() {
    return () => (
      <div class="min-h-screen flex items-center justify-center">
        <div class="text-center">
          <h1 class="text-4xl font-bold text-gray-300 mb-4">404</h1>
          <p class="text-gray-500">页面不存在</p>
        </div>
      </div>
    )
  }
})
