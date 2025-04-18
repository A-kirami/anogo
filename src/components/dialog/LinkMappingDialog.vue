<script setup lang="ts">
import { Link2, ArrowRightLeft, X, Plus } from 'lucide-vue-next'
import naturalCompare from 'natural-compare-lite'
import { OverlayScrollbarsComponent } from 'overlayscrollbars-vue'

let open = defineModel<boolean>('open')

const state = useStateStore()

const gameFigures = $computed(() => {
  const result: { id: string, path: string }[] = []
  if (state.figureRecord) {
    for (const key of Object.keys(state.figureRecord)) {
      const figureData = state.figureRecord[key]
      result.push({ id: key, path: figureData.path })
    }
  }
  return result.sort((a, b) => naturalCompare(a.path, b.path))
})

const activeTab = $ref<'figure' | 'action'>('figure')
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger as-child>
      <slot />
    </DialogTrigger>
    <DialogContent class="grid-rows-[auto_minmax(0,1fr)_auto] max-h-[85dvh]">
      <DialogHeader>
        <DialogTitle>建立关联</DialogTitle>
        <DialogDescription>
          {{ activeTab === 'figure'? '将立绘文件夹与角色名进行关联' : '将动作名与角色动作进行关联' }}
        </DialogDescription>
      </DialogHeader>
      <Tabs v-model="activeTab" class="flex flex-col overflow-hidden">
        <TabsList class="grid grid-cols-2 w-full">
          <TabsTrigger value="figure">
            立绘
          </TabsTrigger>
          <TabsTrigger value="action">
            动作
          </TabsTrigger>
        </TabsList>
        <TabsContent value="figure" class="flex overflow-hidden" :class="{'mt-0': activeTab !== 'figure'}">
          <OverlayScrollbarsComponent defer class="px-2 py-1">
            <div class="flex flex-col gap-2">
              <div v-for="{id, path} in gameFigures" :key="id" class="space-y-2">
                <div class="flex flex-wrap items-baseline gap-2 overflow-hidden">
                  <span class="text-lg text-primary/80 font-semibold">{{ id }}</span>
                  <span class="truncate text-sm text-muted-foreground" style="direction: rtl;">{{ path }}</span>
                </div>
                <TagsInput v-model="state.figureLink[id]">
                  <TagsInputItem v-for="item in state.figureLink[id]" :key="item" :value="item">
                    <TagsInputItemText />
                    <TagsInputItemDelete />
                  </TagsInputItem>
                  <TagsInputInput placeholder="输入回车添加" />
                </TagsInput>
              </div>
            </div>
          </OverlayScrollbarsComponent>
        </TabsContent>
        <TabsContent value="action" class="flex overflow-hidden">
          <OverlayScrollbarsComponent defer class="w-full px-2 py-1">
            <div class="flex flex-col gap-4">
              <div v-for="index in state.actionLink.length" :key="index" class="flex items-center gap-2">
                <Input v-model="state.actionLink[index-1].key" placeholder="动作名" />
                <div class="group h-full flex-shrink-0 cursor-pointer p-1" @click="state.actionLink.splice(index-1, 1)">
                  <ArrowRightLeft class="size-5 text-muted-foreground group-hover:hidden" />
                  <X class="hidden size-5 text-primary group-hover:block" :stroke-width="2.5" />
                </div>
                <Input v-model="state.actionLink[index-1].value" placeholder="角色动作" />
              </div>
              <Button class="w-full" variant="outline" @click="state.actionLink.push({ key: '', value: '' })">
                <Plus class="size-4" />添加关联动作
              </Button>
            </div>
          </OverlayScrollbarsComponent>
        </TabsContent>
      </Tabs>
      <DialogFooter>
        <Button class="w-full" @click="open=false">
          <Link2 class="size-4" />确认关联
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
