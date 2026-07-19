import { defineComponent } from 'vue'

export default defineComponent({
  name: 'AdminListBottom',
  props: {
    totalCount: { type: Number, required: true },
    loadedCount: { type: Number, required: true },
  },
  emits: ['reload', 'filter', 'create'],
  setup(props, { emit }) {
    return () => (
      <div class="fixed bottom-0 left-48 right-0 h-14 bg-white border-t border-border flex items-center justify-between px-6 z-30">
        <span class="text-sm text-gray-500">
          共 {props.totalCount} 条 / 已加载 {props.loadedCount} 条
        </span>
        <div class="flex gap-3">
          <button class="btn border border-border text-sm text-gray-600 hover:bg-gray-100" onClick={() => emit('reload')}>
            刷新
          </button>
          <button class="btn border border-border text-sm text-gray-600 hover:bg-gray-100" onClick={() => emit('filter')}>
            筛选
          </button>
          <button class="btn-primary text-sm" onClick={() => emit('create')}>
            新增
          </button>
        </div>
      </div>
    )
  },
})
