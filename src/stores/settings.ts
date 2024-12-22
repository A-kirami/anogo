import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', () => {
  const theme = $ref<'light' | 'dark' | 'auto'>('auto')
  const dialogueAssociateFigure = $ref(false)
  const enableStrictScript = $ref(false)

  return $$({
    theme,
    dialogueAssociateFigure,
    enableStrictScript,
  })
}, {
  persist: true,
})
