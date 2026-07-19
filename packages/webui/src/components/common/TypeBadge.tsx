import { defineComponent } from 'vue'
import { getTypeColor } from '@/utils/pokemon'
import './TypeBadge.scss'

export default defineComponent({
  name: 'TypeBadge',
  props: { type: String },
  setup(props) {
    return () => (
      <span
        class="inline-block rounded-full px-2 py-0.5 text-xs font-bold text-white"
        style={{ backgroundColor: getTypeColor(props.type) }}
      >
        {props.type}
      </span>
    )
  },
})
