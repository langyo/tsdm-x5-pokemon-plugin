import { defineComponent, computed } from 'vue'
import './StatsRadar.scss'

interface Stats {
  atk: number
  def: number
  spatk: number
  spdef: number
  speed: number
  hp: number
}

const labels = ['HP', '攻击', '防御', '特攻', '特防', '速度']
const keys: (keyof Stats)[] = ['hp', 'atk', 'def', 'spatk', 'spdef', 'speed']
const cx = 100
const cy = 100
const r = 80
const maxVal = 255
const minSize = 16
const gridLevels = [0.2, 0.4, 0.6, 0.8, 1]

function gridPoint(i: number, level: number) {
  const angle = (Math.PI * 2 * i) / keys.length - Math.PI / 2
  return {
    x: cx + Math.cos(angle) * r * level,
    y: cy + Math.sin(angle) * r * level,
  }
}

export default defineComponent({
  name: 'StatsRadar',
  props: {
    stats: {
      type: Object as () => Stats,
      required: true,
    },
  },
  setup(props) {
    const points = computed(() => {
      return keys.map((key, i) => {
        const angle = (Math.PI * 2 * i) / keys.length - Math.PI / 2
        const val = Math.min(props.stats[key], maxVal) / maxVal
        const px = cx + Math.cos(angle) * r * val
        const py = cy + Math.sin(angle) * r * val
        return { x: px, y: py, label: labels[i], val: props.stats[key], angle }
      })
    })

    const labelPositions = computed(() => {
      return points.value.map((p) => {
        const dx = p.x - cx
        const dy = p.y - cy
        const dist = Math.sqrt(dx * dx + dy * dy)
        const ndx = dist > 0 ? dx / dist : 0
        const ndy = dist > 0 ? dy / dist : 0
        const labelR = r + 20
        return {
          x: cx + ndx * labelR,
          y: cy + ndy * labelR,
          label: p.label,
          val: p.val,
        }
      })
    })

    return () => (
      <div class="flex justify-center">
        <svg viewBox={`0 0 ${cx * 2 + minSize} ${cy * 2 + minSize}`} class="w-56 h-56">
          {gridLevels.map((level) => (
            <g key={level} fill="none" stroke="#e5e7eb" strokeWidth="0.5">
              <polygon
                points={keys.map((_, i) => {
                  const gp = gridPoint(i, level)
                  return `${gp.x + minSize / 2},${gp.y + minSize / 2}`
                }).join(' ')}
              />
            </g>
          ))}
          {keys.map((_, i) => (
            <line
              key={'axis-' + i}
              x1={cx + minSize / 2}
              y1={cy + minSize / 2}
              x2={gridPoint(i, 1).x + minSize / 2}
              y2={gridPoint(i, 1).y + minSize / 2}
              stroke="#e5e7eb"
              strokeWidth="0.5"
            />
          ))}
          <polygon
            points={keys.map((_, i) => {
              const gp = gridPoint(i, 1)
              return `${gp.x + minSize / 2},${gp.y + minSize / 2}`
            }).join(' ')}
            fill="#3b82f6"
            fillOpacity="0.25"
            stroke="#3b82f6"
            strokeWidth="1.5"
          />
          <polygon
            points={points.value.map((p) => `${p.x + minSize / 2},${p.y + minSize / 2}`).join(' ')}
            fill="#3b82f6"
            fillOpacity="0.3"
            stroke="#3b82f6"
            strokeWidth="1.5"
          />
          {labelPositions.value.map((lp) => (
            <text
              key={lp.label}
              x={lp.x + minSize / 2}
              y={lp.y + minSize / 2}
              textAnchor="middle"
              dominantBaseline="middle"
              class="text-xs fill-gray-600 font-medium"
            >
              {lp.label}
            </text>
          ))}
          {points.value.map((p) => (
            <text
              key={'val-' + p.label}
              x={p.x + minSize / 2}
              y={p.y + minSize / 2 - 8}
              textAnchor="middle"
              dominantBaseline="middle"
              class="text-xs fill-blue-600 font-bold"
            >
              {p.val}
            </text>
          ))}
        </svg>
      </div>
    )
  },
})
