import { invoke } from '@tauri-apps/api/core'
import type { CompanySettings } from '../types'

export async function getCompanySettings(): Promise<CompanySettings> {
  return invoke<CompanySettings>('get_company_settings')
}

export async function updateCompanySettings(settings: CompanySettings): Promise<void> {
  return invoke('update_company_settings', { settings })
}
