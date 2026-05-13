import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as settingsService from '../services/settingsService'
import type { CompanySettings } from '../types'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<CompanySettings | null>(null)
  const loading = ref(false)

  async function loadSettings() {
    loading.value = true
    try {
      settings.value = await settingsService.getCompanySettings()
    } catch (e) {
      console.error('Failed to load settings:', e)
    } finally {
      loading.value = false
    }
  }

  async function saveSettings() {
    if (!settings.value) return false
    try {
      await settingsService.updateCompanySettings(settings.value)
      return true
    } catch (e) {
      console.error('Failed to save settings:', e)
      throw e
    }
  }

  return { settings, loading, loadSettings, saveSettings }
})
