<script setup lang="ts">
import { Link2 } from 'lucide-vue-next'
import { OverlayScrollbarsComponent } from 'overlayscrollbars-vue'

let open = defineModel<boolean>('open')

const state = useStateStore()

const gameFigures = $computed(() => {
  const result: Record<string, string> = {}
  if (state.figureRecord) {
    for (const key of Object.keys(state.figureRecord)) {
      const figureData = state.figureRecord[key]
      result[figureData.name] = figureData.path
    }
  }
  return result
})
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger as-child>
      <slot />
    </DialogTrigger>
    <DialogContent class="grid-rows-[auto_minmax(0,1fr)_auto] max-h-[85dvh]">
      <DialogHeader>
        <DialogTitle>关联立绘</DialogTitle>
        <DialogDescription>
          在此处将立绘文件夹与角色名进行关联
        </DialogDescription>
      </DialogHeader>
      <OverlayScrollbarsComponent defer class="px-2">
        <div class="flex flex-col gap-2">
          <div v-for="(path, id) in gameFigures" :key="id" class="space-y-2">
            <div class="flex items-baseline gap-2 overflow-hidden">
              <span class="text-lg text-primary/80 font-semibold">{{ id }}</span>
              <span class="truncate text-sm text-muted-foreground">{{ path }}</span>
            </div>
            <TagsInput v-model="state.figureLink[id as string]">
              <TagsInputItem v-for="item in state.figureLink[id as string]" :key="item" :value="item">
                <TagsInputItemText />
                <TagsInputItemDelete />
              </TagsInputItem>
              <TagsInputInput placeholder="输入回车添加" />
            </TagsInput>
          </div>
        </div>
      </OverlayScrollbarsComponent>
      <DialogFooter>
        <Button class="w-full" @click="open=false">
          <Link2 class="size-4" />确认关联
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
