<script setup lang="ts">
import { computed } from 'vue'

interface Stats {
  atk: number
  def: number
  spatk: number
  spdef: number
  speed: number
  hp: number
}

const props = defineProps<{ stats: Stats }>()

const labels = ['HP', '攻击', '防御', '特攻', '特防', '速度']
const keys: (keyof Stats)[] = ['hp', 'atk', 'def', 'spatk', 'spdef', 'speed']

const cx = 100
const cy = 100
const r = 80
const maxVal = 255

const points = computed(() => {
  return keys.map((key, i) => {
    const angle = (Math.PI * 2 * i) / keys.length - Math.PI / 2
    const val = Math.min(props.stats[key], maxVal) / maxVal
    const px = cx + Math.cos(angle) * r * val
    const py = cy + Math.sin(angle) * r * val
    return { x: px, y: py, label: labels[i], val: props.stats[key], angle }
  })
})

const polygonPoints = computed(() =>
  points.value.map((p) => `${p.x},${p.y}`).join(' ')
)

const gridLevels = [0.2, 0.4, 0.6, 0.8, 1]

function gridPoint(i: number, level: number) {
  const angle = (Math.PI * 2 * i) / keys.length - Math.PI / 2
  return {
    x: cx + Math.cos(angle) * r * level,
    y: cy + Math.sin(angle) * r * level,
  }
}

const minSize = 16

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
</script>

<template>
  <div class="flex justify-center">
    <svg :viewBox="`0 0 ${cx * 2 + minSize} ${cy * 2 + minSize}`" class="w-56 h-56">
      <!-- grid polygons -->
      <g
        v-for="level in gridLevels"
        :key="level"
        fill="none"
        stroke="#e5e7eb"
        stroke-width="0.5"
      >
        <polygon
          :points="keys.map((_, i) => {
            const gp = gridPoint(i, level)
            return `${gp.x + minSize / 2},${gp.y + minSize / 2}`
          }).join(' ')"
        />
      </g>

      <!-- axes -->
      <line
        v-for="(_, i) in keys"
        :key="'axis-' + i"
        :x1="cx + minSize / 2"
        :y1="cy + minSize / 2"
        :x2="gridPoint(i, 1).x + minSize / 2"
        :y2="gridPoint(i, 1).y + minSize / 2"
        stroke="#e5e7eb"
        stroke-width="0.5"
      />

      <!-- data polygon -->
      <polygon
        :points="keys.map((_, i) => {
          const gp = gridPoint(i, 1)
          return `${gp.x + minSize / 2},${gp.y + minSize / 2}`
        }).join(' ')"
        fill="#3b82f6"
        fill-opacity="0.25"
        stroke="#3b82f6"
        stroke-width="1.5"
      />

      <!-- shape polygon (actual stat fill) -->
      <polygon
        :points="points.map((p) => `${p.x + minSize / 2},${p.y + minSize / 2}`).join(' ')"
        fill="#3b82f6"
        fill-opacity="0.3"
        stroke="#3b82f6"
        stroke-width="1.5"
      />

      <!-- labels -->
      <text
        v-for="lp in labelPositions"
        :key="lp.label"
        :x="lp.x + minSize / 2"
        :y="lp.y + minSize / 2"
        text-anchor="middle"
        dominant-baseline="middle"
        class="text-xs fill-gray-600 font-medium"
      >
        {{ lp.label }}
      </text>

      <!-- stat values at points -->
      <text
        v-for="p in points"
        :key="'val-' + p.label"
        :x="p.x + minSize / 2"
        :y="p.y + minSize / 2 - 8"
        text-anchor="middle"
        dominant-baseline="middle"
        class="text-xs fill-blue-600 font-bold"
      >
        {{ p.val }}
      </text>
    </svg>
  </div>
</template>
