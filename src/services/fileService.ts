import { invoke } from '@tauri-apps/api/core'

export async function openFile(path: string): Promise<void> {
  return invoke('open_file', { path })
}

export async function openFolder(path: string): Promise<void> {
  return invoke('open_folder', { path })
}
