<template>
  <div class="fixed inset-0 bg-black/40 flex items-center justify-center z-[9999]">
    <div class="bg-white rounded-xl p-8 w-[420px] shadow-2xl text-center">
      <div class="w-14 h-14 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4 text-3xl">
        ✓
      </div>

      <h3 class="text-lg font-bold mb-2">PDF Exported Successfully</h3>

      <p class="bg-gray-100 rounded-md py-2 px-3 text-xs text-gray-700 break-all mb-6">
        📄 {{ fileName }}
      </p>

      <div class="flex gap-3 justify-center">
        <button
          @click="handleOpenFile"
          class="px-5 py-2.5 bg-gray-900 text-white rounded-lg font-semibold text-sm hover:bg-gray-800 transition"
        >
          Open PDF
        </button>
        <button
          @click="handleOpenFolder"
          class="px-5 py-2.5 bg-gray-100 text-gray-900 rounded-lg font-semibold text-sm hover:bg-gray-200 transition"
        >
          Open Folder
        </button>
      </div>

      <p class="mt-4 text-xs text-gray-400">
        Auto-closing in {{ countdown }}s
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  filePath: string
}>()

const emit = defineEmits<{
  close: []
}>()

const countdown = ref(5)
const fileName = props.filePath.split(/[\\/]/).pop() || props.filePath

let timer: ReturnType<typeof setInterval>

onMounted(() => {
  timer = setInterval(() => {
    countdown.value--
    if (countdown.value <= 0) {
      clearInterval(timer)
      emit('close')
    }
  }, 1000)
})

onUnmounted(() => {
  clearInterval(timer)
})

const handleOpenFile = async () => {
  await invoke('open_file', { path: props.filePath })
  emit('close')
}

const handleOpenFolder = async () => {
  await invoke('open_folder', { path: props.filePath })
  emit('close')
}
</script>
