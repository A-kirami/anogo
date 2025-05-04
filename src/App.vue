<script setup lang="ts">
import { relaunch } from '@tauri-apps/plugin-process'
import { check } from '@tauri-apps/plugin-updater'
import { storeToRefs } from 'pinia'

import { isRelease } from '~build/meta'

const settings = useSettingsStore()

const { theme } = storeToRefs(settings)

const themeMode = useColorMode({
  emitAuto: true,
  storageRef: theme,
})

provide('themeMode', themeMode)

document.addEventListener('contextmenu', (e) => {
  if (import.meta.env.PROD) {
    e.preventDefault()
  }
})

onMounted(async () => {
  await logger.attachConsole()

  if (isRelease) {
    try {
      const update = await check()
      if (update?.available) {
        await update.downloadAndInstall()
        await relaunch()
      }
    } catch (error) {
      void logger.error(`检查更新失败: ${error as string}`)
    }
  }
})
</script>

<template>
  <div class="grid grid-rows-[auto_minmax(0,1fr)] h-screen">
    <AppTitlebar />
    <div class="w-screen flex flex-col gap-3 px-6 py-3">
      <Menubar />
      <CodeEditor class="grow" />
      <GenerateButton />
      <AboutInfo />
    </div>
  </div>
  <Notification />
</template>
