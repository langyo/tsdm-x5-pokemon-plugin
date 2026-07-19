import { defineComponent, watch, Teleport } from 'vue'
import './Modal.scss'

export default defineComponent({
  name: 'Modal',
  props: {
    open: Boolean,
    title: String,
    wide: Boolean,
  },
  emits: ['close'],
  setup(props, { emit, slots }) {
    function onKey(e: KeyboardEvent) {
      if (e.key === 'Escape') emit('close')
    }

    watch(
      () => props.open,
      (val) => {
        if (val) {
          document.addEventListener('keydown', onKey)
        } else {
          document.removeEventListener('keydown', onKey)
        }
      }
    )

    return () => (
      <Teleport to="body">
        {props.open ? (
          <div class="fixed inset-0 z-40 flex items-center justify-center">
            <div class="absolute inset-0 bg-black/50" onClick={() => emit('close')} />
            <div
              class={[
                'relative bg-white rounded-xl shadow-lg border border-border z-10 mx-4',
                props.wide ? 'w-full max-w-3xl' : 'w-full max-w-md',
              ].join(' ')}
            >
              <div class="flex items-center justify-between px-5 py-3 border-b border-border">
                <h3 class="text-base font-bold">{props.title || ''}</h3>
                <button
                  class="text-gray-400 hover:text-gray-600 text-lg leading-none p-1"
                  onClick={() => emit('close')}
                >
                  &#x2715;
                </button>
              </div>
              <div class="p-5">
                {slots.default?.()}
              </div>
            </div>
          </div>
        ) : null}
      </Teleport>
    )
  },
})
