import { defineStore } from 'pinia'

export const useSettingsStore = defineStore('settings', () => {
  const theme = $ref<'light' | 'dark' | 'auto'>('auto')
  const dialogueAssociateFigure = $ref(false)
  const enableStrictScript = $ref(false)
  const removeTrailingPeriodInDialogue = $ref(false)
  const figureDefaultAction = $ref('')

  return $$({
    theme,
    dialogueAssociateFigure,
    enableStrictScript,
    removeTrailingPeriodInDialogue,
    figureDefaultAction,
  })
}, {
  persist: true,
})
