<template>
  <div class="space-y-6">
    <div class="card max-w-2xl">
      <h3 class="text-lg font-semibold mb-6">Company Settings</h3>
      
      <div v-if="settings" class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Company Name</label>
          <input v-model="settings.company_name" type="text" class="input" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Tagline</label>
          <input v-model="settings.tagline" type="text" class="input" placeholder="Deals in lab Consumables, Reagents & Medical Equipments." />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Address</label>
          <textarea v-model="settings.office_address" class="input" rows="3"></textarea>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Phone 1</label>
            <input v-model="settings.phone1" type="text" class="input" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Phone 2</label>
            <input v-model="settings.phone2" type="text" class="input" />
          </div>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">NTN Number</label>
            <input v-model="settings.ntn_number" type="text" class="input" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Invoice Prefix</label>
            <input v-model="settings.invoice_prefix" type="text" class="input" placeholder="ZE" />
          </div>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Quotation Prefix</label>
          <input v-model="settings.quotation_prefix" type="text" class="input" placeholder="QT" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Salutation</label>
          <input v-model="settings.salutation" type="text" class="input" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Body Text</label>
          <textarea v-model="settings.body_text" class="input" rows="2"></textarea>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
          <input v-model="settings.email" type="email" class="input" />
        </div>
        
        <div class="flex justify-end pt-4">
          <button @click="saveSettings" class="btn-primary">Save Settings</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface CompanySettings {
  company_name: string
  tagline: string
  ntn_number: string
  office_address: string
  phone1: string
  phone2: string
  email: string
  gst_number: string
  pan_number: string
  invoice_prefix: string
  quotation_prefix: string
  salutation: string
  body_text: string
  banner_color: string
  footer_color: string
}

const settings = ref<CompanySettings | null>(null)

const loadSettings = async () => {
  try {
    settings.value = await invoke<CompanySettings>('get_company_settings')
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
}

const saveSettings = async () => {
  if (!settings.value) return
  try {
    await invoke('update_company_settings', { settings: settings.value })
    alert('Settings saved successfully!')
  } catch (error) {
    console.error('Failed to save settings:', error)
    alert('Failed to save: ' + error)
  }
}

onMounted(() => { loadSettings() })
</script>
