<script setup lang="ts">
import { FolderHeart } from 'lucide-vue-next'
import naturalCompare from 'natural-compare-lite'
import { OverlayScrollbarsComponent } from 'overlayscrollbars-vue'

let open = defineModel<boolean>('open')

const state = useStateStore()

let selected = $ref<string>(state.selectedGame ?? '')

const gameList = $computed(() => {
  if (state.gameRecord) {
    return Object.keys(state.gameRecord).map((key) => {
      return { name: key, ...state.gameRecord![key] }
    }).sort((a, b) => naturalCompare(a.name, b.name))
  }
})

async function selectGame() {
  state.selectedGame = selected
  open.value = false
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger as-child>
      <slot />
    </DialogTrigger>
    <DialogContent class="grid-rows-[auto_minmax(0,1fr)_auto] max-h-[85dvh]">
      <DialogHeader>
        <DialogTitle>选择游戏</DialogTitle>
        <DialogDescription>
          选择需要加载的游戏
        </DialogDescription>
      </DialogHeader>
      <OverlayScrollbarsComponent defer class="overflow-hidden">
        <div v-if="state.gameRecord" class="flex flex-col gap-2">
          <div
            v-for="{name, icon, path} in gameList" :key="name"
            class="flex cursor-pointer items-center gap-2 border rounded-md px-3 py-2"
            :class="[selected === name? 'bg-primary/10 border-primary/80' : 'hover:(bg-primary/5 border-primary/40)']"
            @click="selected = name"
          >
            <img :src="icon || '/webgal.webp'" alt="game icon" class="size-12 rounded">
            <div class="flex flex-col gap-1 overflow-hidden">
              <span class="text-lg font-semibold">{{ name }}</span>
              <span class="shrink-0 truncate text-xs text-muted-foreground" style="direction: rtl;">{{ path }}</span>
            </div>
          </div>
        </div>
      </OverlayScrollbarsComponent>
      <DialogFooter>
        <Button class="w-full" @click="selectGame">
          <FolderHeart class="size-4" />确认选择
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
