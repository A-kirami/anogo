<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { join } from '@tauri-apps/api/path'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import { save } from '@tauri-apps/plugin-dialog'
import { ClipboardCopy, FilePlus2 } from 'lucide-vue-next'
import { OverlayScrollbarsComponent } from 'overlayscrollbars-vue'

let open = defineModel<boolean>('open')

const state = useStateStore()

async function copyToClipboard() {
  await writeText(state.scriptCode)
  notify.success('已复制到剪贴板')
}

async function createSceneFile() {
  const defaultPath = state.selectedGameInfo ? await join(state.selectedGameInfo?.path, 'game', 'scene') : undefined
  const path = await save({
    title: '创建脚本文件',
    defaultPath,
    filters: [
      {
        name: 'WebGAL 脚本',
        extensions: ['txt'],
      },
    ],
  })
  if (path) {
    let encoder = new TextEncoder()
    let data = encoder.encode(state.scriptCode)
    try {
      await invoke('write_file', { contents: data, path, overwrite: true })
      notify.success('脚本文件已创建')
    } catch (error) {
      notify.error('脚本文件创建失败')
      void logger.error(`脚本文件创建失败: ${error}`)
    }
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger as-child>
      <slot />
    </DialogTrigger>
    <DialogContent class="grid-rows-[auto_minmax(0,1fr)_auto] max-h-[85dvh]">
      <DialogHeader>
        <DialogTitle>脚本已生成</DialogTitle>
        <DialogDescription>
          将以下脚本添加到你的 WebGAL 游戏中
        </DialogDescription>
      </DialogHeader>
      <OverlayScrollbarsComponent defer class="border rounded-lg px-2 py-1">
        <div class="select-text whitespace-pre-wrap text-sm prose prose-truegray">
          {{ state.scriptCode }}
        </div>
      </OverlayScrollbarsComponent>
      <DialogFooter>
        <div class="w-full space-y-2">
          <Button class="w-full" @click="copyToClipboard">
            <ClipboardCopy class="size-4" />复制到剪贴板
          </Button>
          <Button class="w-full" @click="createSceneFile">
            <FilePlus2 class="size-4" />创建脚本文件
          </Button>
        </div>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
