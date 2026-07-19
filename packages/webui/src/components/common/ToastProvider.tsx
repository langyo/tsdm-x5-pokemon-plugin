import { defineComponent, reactive, inject, provide } from 'vue'
import './ToastProvider.scss'

interface Toast {
  id: number
  message: string
  type: 'info' | 'success' | 'error'
}

let nextId = 0

const key = Symbol('toast')

export function useToast() {
  return inject<{
    info: (msg: string) => void
    success: (msg: string) => void
    error: (msg: string) => void
  }>(key)!
}

export default defineComponent({
  name: 'ToastProvider',
  setup(_props, { slots }) {
    const toasts = reactive<Toast[]>([])

    function addToast(message: string, type: 'info' | 'success' | 'error' = 'info') {
      const id = nextId++
      toasts.push({ id, message, type })
      setTimeout(() => {
        const idx = toasts.findIndex((t) => t.id === id)
        if (idx !== -1) toasts.splice(idx, 1)
      }, 3000)
    }

    const toast = {
      info: (msg: string) => addToast(msg, 'info'),
      success: (msg: string) => addToast(msg, 'success'),
      error: (msg: string) => addToast(msg, 'error'),
    }

    provide(key, toast)

    return () => (
      <div class="fixed top-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
        {toasts.map((t) => (
          <div
            key={t.id}
            class={[
              'px-4 py-2 rounded-lg text-sm text-white shadow-lg pointer-events-auto transition-all',
              t.type === 'info' ? 'bg-blue-500' : t.type === 'success' ? 'bg-green-500' : 'bg-red-500',
            ].join(' ')}
          >
            {t.message}
          </div>
        ))}
        {slots.default?.()}
      </div>
    )
  },
})
