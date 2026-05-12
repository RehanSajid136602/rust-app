<template>
  <div class="relative" ref="containerRef">
    <input
      ref="inputRef"
      :value="query"
      type="text"
      class="w-full border-0 focus:ring-0 p-0 text-sm"
      :placeholder="placeholder"
      @input="onInputEv"
      @keydown="onKeydown"
      @focus="onFocus"
      @blur="onBlur"
      @keydown.enter.prevent="onEnter"
      @keydown.tab="onTab"
    />

    <!-- Dropdown portaled to body to avoid clipping -->
    <Teleport to="body">
      <div
        v-if="showDropdown"
        :style="dropdownStyle"
        class="fixed bg-white border border-gray-200 rounded-lg shadow-lg z-[9999] max-h-48 overflow-y-auto min-w-[200px]"
      >
        <div v-if="suggestions.length > 0">
          <div
            v-for="(item, idx) in suggestions"
            :key="item.id ?? idx"
            :ref="el => setItemRef(el, idx)"
            class="px-3 py-2 cursor-pointer text-sm flex items-center justify-between"
            :class="idx === selectedIdx ? 'bg-primary-50 text-primary-700' : 'hover:bg-gray-50 text-gray-700'"
            @mousedown.prevent="selectItem(item)"
          >
            <span class="font-medium truncate">{{ item.name }}</span>
            <span class="text-xs text-gray-500 ml-3 whitespace-nowrap">{{ item.detail }}</span>
          </div>
        </div>
        <div v-else class="px-3 py-3 text-sm text-gray-400 text-center">
          {{ query.length > 0 ? 'No matching products' : 'No products in catalog' }}
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface Suggestion {
  id: number | string
  name: string
  detail: string
  data?: any
}

const props = withDefaults(defineProps<{
  modelValue?: string
  placeholder?: string
  minChars?: number
  debounceMs?: number
}>(), {
  modelValue: '',
  placeholder: 'Type to search products...',
  minChars: 1,
  debounceMs: 150,
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'select', item: Suggestion): void
  (e: 'enter'): void
  (e: 'tab'): void
}>()

const query = ref(props.modelValue)
const suggestions = ref<Suggestion[]>([])
const selectedIdx = ref(-1)
const showDropdown = ref(false)
const containerRef = ref<HTMLElement | null>(null)
const inputRef = ref<HTMLInputElement | null>(null)
const itemRefs = ref<HTMLElement[]>([])
let debounceTimer: ReturnType<typeof setTimeout> | null = null

// Dropdown position tracking
const dropdownPos = ref({ top: 0, left: 0, width: 0 })
const dropdownStyle = computed(() => ({
  top: dropdownPos.value.top + 'px',
  left: dropdownPos.value.left + 'px',
  width: Math.max(dropdownPos.value.width, 200) + 'px',
}))

const updateDropdownPos = () => {
  if (containerRef.value) {
    const rect = containerRef.value.getBoundingClientRect()
    dropdownPos.value = {
      top: rect.bottom + 4,
      left: rect.left,
      width: rect.width,
    }
  }
}

// Sync external modelValue
watch(() => props.modelValue, (val) => {
  if (val !== undefined && val !== query.value) {
    query.value = val
  }
})

const setItemRef = (el: any, idx: number) => {
  if (el) itemRefs.value[idx] = el
}

const setValue = (value: string) => {
  query.value = value
  emit('update:modelValue', value)
}

const focus = () => {
  inputRef.value?.focus()
}

defineExpose({ setValue, focus })

const onInputEv = (e: Event) => {
  const val = (e.target as HTMLInputElement).value
  query.value = val
  emit('update:modelValue', val)

  if (val.length >= props.minChars) {
    if (debounceTimer) clearTimeout(debounceTimer)
    debounceTimer = setTimeout(() => {
      updateDropdownPos()
      searchProducts(val)
    }, props.debounceMs)
  } else {
    suggestions.value = []
    showDropdown.value = false
  }
}

const onFocus = () => {
  updateDropdownPos()
  searchProducts(query.value.length >= props.minChars ? query.value : '')
}

const onBlur = () => {
  setTimeout(() => {
    showDropdown.value = false
  }, 180)
}

const searchProducts = async (q: string) => {
  try {
    const results = await invoke<any[]>('search_products', { query: q })
    if (results && results.length > 0) {
      suggestions.value = results.map((p: any) => ({
        id: p.id,
        name: p.name,
        detail: new Intl.NumberFormat('en-IN', {
          style: 'currency',
          currency: 'INR',
          maximumFractionDigits: 0,
        }).format(p.price_per_unit),
        data: p,
      }))
      showDropdown.value = true
    } else {
      suggestions.value = []
      showDropdown.value = true
    }
  } catch (e) {
    console.error('searchProducts failed:', e)
    suggestions.value = []
    showDropdown.value = false
  }
}

const onKeydown = (e: KeyboardEvent) => {
  if (!showDropdown.value || suggestions.value.length === 0) return

  if (e.key === 'ArrowDown') {
    e.preventDefault()
    selectedIdx.value = Math.min(selectedIdx.value + 1, suggestions.value.length - 1)
    scrollToSelected()
  } else if (e.key === 'ArrowUp') {
    e.preventDefault()
    selectedIdx.value = Math.max(selectedIdx.value - 1, -1)
    scrollToSelected()
  } else if (e.key === 'Escape') {
    showDropdown.value = false
  }
}

const scrollToSelected = async () => {
  await nextTick()
  const el = itemRefs.value[selectedIdx.value]
  if (el) {
    el.scrollIntoView({ block: 'nearest' })
  }
}

const selectItem = (item: Suggestion) => {
  query.value = item.name
  emit('update:modelValue', item.name)
  showDropdown.value = false
  emit('select', item)
}

const onEnter = () => {
  if (showDropdown.value && selectedIdx.value >= 0 && selectedIdx.value < suggestions.value.length) {
    selectItem(suggestions.value[selectedIdx.value])
  } else {
    showDropdown.value = false
    emit('enter')
  }
}

const onTab = () => {
  showDropdown.value = false
  emit('tab')
}

watch(() => query.value, (val) => {
  if (!val) {
    suggestions.value = []
    showDropdown.value = false
  }
})

// Update position on scroll/resize
onMounted(() => {
  window.addEventListener('scroll', updateDropdownPos, true)
  window.addEventListener('resize', updateDropdownPos)
})

onUnmounted(() => {
  window.removeEventListener('scroll', updateDropdownPos, true)
  window.removeEventListener('resize', updateDropdownPos)
})
</script>
