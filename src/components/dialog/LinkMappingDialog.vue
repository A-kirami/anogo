<script setup lang="ts">
import { Link2, ArrowRightLeft, X, Plus } from 'lucide-vue-next'
import naturalCompare from 'natural-compare-lite'
import { OverlayScrollbarsComponent } from 'overlayscrollbars-vue'
import { ref, computed, watch, nextTick } from 'vue'

const open = defineModel<boolean>('open')

const state = useStateStore()

const searchQuery = ref('')
const prioritizeLinked = ref(false)
const activeTab = ref<'figure' | 'action'>('figure')

// 分页相关
const currentPage = ref(1)
const pageSize = 30
const totalPages = computed(() => Math.ceil(filteredFigures.value.length / pageSize))
const isLoading = ref(false)

// 所有立绘结果（预处理、缓存排序）
const rawFigures = computed(() => {
  const result: { id: string, path: string }[] = []

  if (state.figureRecord) {
    for (const characterKey of Object.keys(state.figureRecord)) {
      const figureData = state.figureRecord[characterKey]
      if (Array.isArray(figureData.costumes)) {
        for (const costume of figureData.costumes) {
          result.push({
            id: `${characterKey}/${costume.name}`,
            path: costume.path,
          })
        }
      } else {
        result.push({ id: characterKey, path: figureData.path })
      }
    }
  }

  return result
})

// 过滤 + 排序后的完整列表（不分页）
const filteredFigures = computed(() => {
  const keyword = searchQuery.value.trim().toLowerCase()

  let filtered = rawFigures.value.filter(({ id, path }) =>
    !keyword || id.toLowerCase().includes(keyword) || path.toLowerCase().includes(keyword),
  )

  if (prioritizeLinked.value) {
    filtered.sort((a, b) => {
      const aLinked = !!state.figureLink[a.id]?.length
      const bLinked = !!state.figureLink[b.id]?.length
      return aLinked === bLinked
        ? naturalCompare(a.path, b.path)
        : (aLinked ? -1 : 1)
    })
  } else {
    filtered.sort((a, b) => naturalCompare(a.path, b.path))
  }

  return filtered
})

// 当前页数据
const pagedFigures = computed(() => {
  const start = (currentPage.value - 1) * pageSize
  const end = start + pageSize
  return filteredFigures.value.slice(start, end)
})

// 当搜索关键词或优先级排序变化时，重置页码 + 添加 loading 延迟
watch([searchQuery, prioritizeLinked], async () => {
  isLoading.value = true
  currentPage.value = 1
  await nextTick()
  setTimeout(() => {
    isLoading.value = false
  }, 100)
})
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
          {{ activeTab === 'figure'
            ? '将立绘文件夹与角色名进行关联'
            : '将动作名与角色动作进行关联' }}
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

        <TabsContent value="figure" class="flex overflow-hidden">
          <div class="w-full flex flex-col">
            <!-- 搜索与排序控件 -->
            <div class="flex items-center justify-between gap-2 px-2 pb-1">
              <input
                v-model="searchQuery"
                placeholder="搜索角色/路径..."
                class="w-full border rounded-md px-2 py-1 text-sm"
              >
              <label class="flex cursor-pointer items-center gap-1 text-xs">
                <input v-model="prioritizeLinked" type="checkbox">
                已关联优先
              </label>
            </div>

            <!-- loading 动画 -->
            <div v-if="isLoading" class="py-4 text-center text-sm text-muted-foreground">
              正在筛选中...
            </div>

            <!-- 展示立绘条目 -->
            <OverlayScrollbarsComponent v-else defer class="flex-1 px-2 py-1">
              <div class="flex flex-col gap-2">
                <div
                  v-for="{ id, path } in pagedFigures"
                  :key="id"
                  class="space-y-2"
                >
                  <div class="flex flex-wrap items-baseline gap-2 overflow-hidden">
                    <span class="text-lg text-primary/80 font-semibold">{{ id }}</span>
                    <span
                      class="truncate text-sm text-muted-foreground"
                      style=" overflow: hidden;text-overflow: ellipsis; text-align: right; white-space: nowrap;"
                    >
                      {{ path }}
                      <span v-if="path.endsWith('.jsonl')" class="ml-2 text-pink-500">(聚合模型)</span>
                    </span>
                  </div>
                  <TagsInput v-model="state.figureLink[id]">
                    <TagsInputItem
                      v-for="item in state.figureLink[id]"
                      :key="item"
                      :value="item"
                    >
                      <TagsInputItemText />
                      <TagsInputItemDelete />
                    </TagsInputItem>
                    <TagsInputInput placeholder="输入回车添加" />
                  </TagsInput>
                </div>
              </div>
            </OverlayScrollbarsComponent>

            <!-- 分页控件 -->
            <div v-if="totalPages > 1" class="flex justify-center gap-4 py-2 text-center text-xs text-muted-foreground">
              <button
                :disabled="currentPage <= 1"
                class="hover:underline disabled:opacity-30"
                @click="currentPage--"
              >
                上一页
              </button>
              <span>第 {{ currentPage }} / {{ totalPages }} 页</span>
              <button
                :disabled="currentPage >= totalPages"
                class="hover:underline disabled:opacity-30"
                @click="currentPage++"
              >
                下一页
              </button>
            </div>
          </div>
        </TabsContent>

        <TabsContent value="action" class="flex overflow-hidden">
          <OverlayScrollbarsComponent defer class="w-full px-2 py-1">
            <div class="flex flex-col gap-4">
              <div
                v-for="(action, index) in state.actionLink"
                :key="index"
                class="flex items-center gap-2"
              >
                <Input v-model="action.key" placeholder="动作名" />
                <div class="group h-full flex-shrink-0 cursor-pointer p-1" @click="state.actionLink.splice(index, 1)">
                  <ArrowRightLeft class="size-5 text-muted-foreground group-hover:hidden" />
                  <X class="hidden size-5 text-primary group-hover:block" :stroke-width="2.5" />
                </div>
                <Input v-model="action.value" placeholder="角色动作" />
              </div>
              <Button class="w-full" variant="outline" @click="state.actionLink.push({ key: '', value: '' })">
                <Plus class="size-4" /> 添加关联动作
              </Button>
            </div>
          </OverlayScrollbarsComponent>
        </TabsContent>
      </Tabs>

      <DialogFooter>
        <Button class="w-full" @click="open = false">
          <Link2 class="size-4" /> 确认关联
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
