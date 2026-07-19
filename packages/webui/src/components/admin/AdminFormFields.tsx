import { defineComponent, h, type PropType } from 'vue'

const fieldClass = 'bg-gray-100 rounded-lg px-3 py-2 border border-border focus:outline-none focus:ring-2 focus:ring-primary/50 w-full transition-colors disabled:opacity-50'
const labelClass = 'text-sm text-gray-600 font-medium'
const wrapperClass = 'flex flex-col gap-1.5'

export const AdminTextField = defineComponent({
  name: 'AdminTextField',
  props: {
    label: { type: String, required: true },
    modelValue: { type: String, default: '' },
    placeholder: { type: String, default: '' },
    disabled: { type: Boolean, default: false },
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    return () =>
      h('label', { class: wrapperClass }, [
        h('span', { class: labelClass }, props.label),
        h('input', {
          type: 'text',
          class: fieldClass,
          value: props.modelValue,
          placeholder: props.placeholder,
          disabled: props.disabled,
          onInput: (e: Event) => emit('update:modelValue', (e.target as HTMLInputElement).value),
        }),
      ])
  },
})

export const AdminNumberField = defineComponent({
  name: 'AdminNumberField',
  props: {
    label: { type: String, required: true },
    modelValue: { type: Number, default: 0 },
    min: { type: Number, default: undefined },
    max: { type: Number, default: undefined },
    step: { type: Number, default: undefined },
    disabled: { type: Boolean, default: false },
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    return () =>
      h('label', { class: wrapperClass }, [
        h('span', { class: labelClass }, props.label),
        h('input', {
          type: 'number',
          class: fieldClass,
          value: props.modelValue,
          min: props.min,
          max: props.max,
          step: props.step,
          disabled: props.disabled,
          onInput: (e: Event) => {
            const v = (e.target as HTMLInputElement).value
            emit('update:modelValue', v === '' ? 0 : Number(v))
          },
        }),
      ])
  },
})

export const AdminSelectField = defineComponent({
  name: 'AdminSelectField',
  props: {
    label: { type: String, required: true },
    modelValue: { type: String, default: '' },
    options: { type: Array as PropType<{ value: string; label: string }[]>, required: true },
    disabled: { type: Boolean, default: false },
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    return () =>
      h('label', { class: wrapperClass }, [
        h('span', { class: labelClass }, props.label),
        h('select', {
          class: fieldClass + ' appearance-none',
          value: props.modelValue,
          disabled: props.disabled,
          onChange: (e: Event) => emit('update:modelValue', (e.target as HTMLSelectElement).value),
        }, props.options.map((opt) =>
          h('option', { key: opt.value, value: opt.value }, opt.label),
        )),
      ])
  },
})

export const AdminSwitchField = defineComponent({
  name: 'AdminSwitchField',
  props: {
    label: { type: String, required: true },
    modelValue: { type: Boolean, default: false },
    disabled: { type: Boolean, default: false },
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    return () =>
      h('label', { class: 'flex items-center justify-between gap-2 cursor-pointer' }, [
        h('span', { class: labelClass }, props.label),
        h('button', {
          type: 'button',
          role: 'switch',
          class: `relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
            props.modelValue ? 'bg-primary' : 'bg-gray-300'
          } ${props.disabled ? 'opacity-50 cursor-not-allowed' : ''}`,
          disabled: props.disabled,
          onClick: () => emit('update:modelValue', !props.modelValue),
        }, [
          h('span', {
            class: `inline-block h-4 w-4 rounded-full bg-white transition-transform ${
              props.modelValue ? 'translate-x-6' : 'translate-x-1'
            }`,
          }),
        ]),
      ])
  },
})

export const AdminTextAreaField = defineComponent({
  name: 'AdminTextAreaField',
  props: {
    label: { type: String, required: true },
    modelValue: { type: String, default: '' },
    rows: { type: Number, default: 4 },
    disabled: { type: Boolean, default: false },
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    return () =>
      h('label', { class: wrapperClass }, [
        h('span', { class: labelClass }, props.label),
        h('textarea', {
          class: fieldClass + ' resize-y min-h-[80px]',
          value: props.modelValue,
          rows: props.rows,
          disabled: props.disabled,
          onInput: (e: Event) => emit('update:modelValue', (e.target as HTMLTextAreaElement).value),
        }),
      ])
  },
})

export const AdminFloatField = defineComponent({
  name: 'AdminFloatField',
  props: {
    label: { type: String, required: true },
    modelValue: { type: Number, default: 0 },
    min: { type: Number, default: undefined },
    max: { type: Number, default: undefined },
    step: { type: Number, default: 0.1 },
    disabled: { type: Boolean, default: false },
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    return () =>
      h('label', { class: wrapperClass }, [
        h('span', { class: labelClass }, props.label),
        h('input', {
          type: 'number',
          class: fieldClass,
          value: props.modelValue,
          min: props.min,
          max: props.max,
          step: props.step,
          disabled: props.disabled,
          onInput: (e: Event) => {
            const v = (e.target as HTMLInputElement).value
            emit('update:modelValue', v === '' ? 0 : parseFloat(v))
          },
        }),
      ])
  },
})
