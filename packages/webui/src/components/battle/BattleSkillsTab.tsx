import { computed, defineComponent } from 'vue'
import { useBattleStore } from '@/stores'
import { api } from '@/api/client'
import TypeBadge from '@/components/common/TypeBadge'

export default defineComponent({
  name: 'BattleSkillsTab',
  props: {
    waiting: { type: Boolean, required: true },
  },
  emits: ['action'],
  setup(props, { emit }) {
    const battleStore = useBattleStore()

    const skills = computed(() => {
      return battleStore.scene?.my_pokemon?.skills ?? []
    })

    function hasSkill(skill: any): string {
      const attrs = skill.attribute?.split(',') ?? []
      if (attrs.includes('must_hit')) return '必中'
      if (attrs.includes('high_crit')) return '易暴击'
      if (attrs.includes('first_strike')) return '先制'
      return ''
    }

    async function useSkill(skillId: number) {
      if (props.waiting) return
      emit('action')
      try {
        const data: any = await api.post('battle', { action: 'turn', skill_id: skillId })
        battleStore.setScene(data.scene ?? data)
      } catch {
        // handled by parent
      }
    }

    return () => (
      <div class="space-y-2">
        {!skills.value.length ? (
          <div class="text-center py-8 text-gray-400 text-sm">
            当前宝可梦没有技能
          </div>
        ) : null}

        {skills.value.map((skill: any) => {
          const tagText = hasSkill(skill)
          return (
            <button
              key={skill.id}
              class={['w-full flex items-center gap-3 p-3 rounded-lg border border-border bg-white hover:bg-gray-50 transition-colors text-left', props.waiting ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer']}
              disabled={props.waiting}
              onClick={() => useSkill(skill.id)}
            >
              <div class="shrink-0 w-12 text-center">
                <TypeBadge type={skill.type ?? '普'} />
              </div>
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2">
                  <span class="text-sm font-bold truncate">{skill.name}</span>
                  {tagText && (
                    <span class="text-xs px-1.5 py-0.5 rounded bg-blue-100 text-blue-700 font-medium">
                      {tagText}
                    </span>
                  )}
                </div>
                <div class="text-xs text-gray-400 mt-0.5">
                  {skill.power && <span>威力 {skill.power}</span>}
                  {skill.power && skill.pp !== undefined && <span class="mx-1">·</span>}
                  {skill.pp !== undefined && (
                    <span>
                      PP {skill.pp}/{skill.max_pp ?? skill.pp_max}
                    </span>
                  )}
                </div>
              </div>
              {skill.pp !== undefined && skill.pp <= 0 && (
                <span class="text-xs text-red-500 font-bold shrink-0">
                  PP不足
                </span>
              )}
            </button>
          )
        })}

        <div class="relative my-3 flex items-center">
          <div class="flex-1 border-t border-border" />
          <span class="px-3 text-xs text-gray-400">基础攻击</span>
          <div class="flex-1 border-t border-border" />
        </div>

        <button
          class={['w-full flex items-center gap-3 p-3 rounded-lg border border-border bg-white hover:bg-gray-50 transition-colors text-left cursor-pointer', props.waiting ? 'opacity-50 cursor-not-allowed' : '']}
          disabled={props.waiting}
          onClick={() => useSkill(0)}
        >
          <div class="shrink-0 w-12 text-center">
            <TypeBadge type="普" />
          </div>
          <div class="flex-1">
            <div class="text-sm font-bold">普通攻击</div>
            <div class="text-xs text-gray-400">威力 40</div>
          </div>
        </button>
      </div>
    )
  },
})
