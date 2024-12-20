<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'
import { CircleSlash2, Folder, Settings, Link } from 'lucide-vue-next'

let openSelect = $ref(false)

let openSettings = $ref(false)

const state = useStateStore()

async function loadGame() {
  const basePath = await open({
    title: '选择 WebGAL 目录',
    directory: true,
  })
  if (basePath) {
    state.webgalPath = basePath
    openSelect = true
  }
}
</script>

<template>
  <div class="flex items-center justify-between gap-2">
    <div v-if="state.selectedGameInfo" class="flex gap-2 overflow-hidden">
      <div class="group flex cursor-pointer items-center gap-2 overflow-hidden" @click="openSelect = true">
        <img :src="state.selectedGameInfo.icon || '/webgal.png'" alt="game icon" class="size-9 rounded">
        <span class="truncate text-lg font-semibold transition-colors group-hover:text-primary/80">{{ state.selectedGame }}</span>
      </div>
      <LinkFigureDialog>
        <Button variant="outline" size="icon" class="shrink-0">
          <Link class="h-4 w-4" />
        </Button>
      </LinkFigureDialog>
    </div>
    <div v-else class="flex items-center gap-2 text-muted-foreground">
      <CircleSlash2 class="size-4" />未选择 WebGAL 目录
    </div>
    <div class="flex gap-2">
      <Button size="icon" @click="loadGame">
        <Folder class="size-4" />
      </Button>
      <Button size="icon" @click="openSettings = true">
        <Settings class="size-4" />
      </Button>
    </div>
  </div>
  <GameSelectDialog v-model:open="openSelect" />
  <SettingsDialog v-model:open="openSettings" />
</template>
