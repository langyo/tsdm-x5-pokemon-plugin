import { defineComponent } from 'vue'
import './Card.scss'

export default defineComponent({
  name: 'Card',
  props: { title: String },
  setup(props, { slots }) {
    return () => (
      <div class="rounded-xl shadow-sm border border-border bg-white p-4">
        {props.title ? <div class="text-base font-bold mb-3">{props.title}</div> : null}
        {slots.default?.()}
      </div>
    )
  },
})
