import { defineComponent, watch, Teleport } from 'vue'
import './PopupMenu.scss'

export default defineComponent({
  name: 'PopupMenu',
  props: {
    open: Boolean,
    x: Number,
    y: Number,
  },
  emits: ['close'],
  setup(props, { emit, slots }) {
    function onClickOutside(_e: MouseEvent) {
      emit('close')
    }

    function onKey(e: KeyboardEvent) {
      if (e.key === 'Escape') emit('close')
    }

    watch(
      () => props.open,
      (val) => {
        if (val) {
          document.addEventListener('click', onClickOutside)
          document.addEventListener('keydown', onKey)
        } else {
          document.removeEventListener('click', onClickOutside)
          document.removeEventListener('keydown', onKey)
        }
      }
    )

    return () => (
      <Teleport to="body">
        {props.open ? (
          <div
            class="fixed z-50 min-w-32 bg-white rounded-lg shadow-lg border border-border py-1"
            style={{ left: props.x + 'px', top: props.y + 'px' }}
            onClick={(e: Event) => e.stopPropagation()}
          >
            {slots.default?.()}
          </div>
        ) : null}
      </Teleport>
    )
  },
})
