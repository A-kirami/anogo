<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod'
import { Pen } from 'lucide-vue-next'
import { OverlayScrollbarsComponent } from 'overlayscrollbars-vue'
import * as z from 'zod'

import { Form, FormField } from '~/components/ui/form'

let open = defineModel<boolean>('open')

const settings = useSettingsStore()

const settingsSchema = toTypedSchema(
  z.object({
    theme: z.enum(['light', 'dark', 'auto']),
    dialogueAssociateFigure: z.boolean(),
    enableStrictScript: z.boolean(),
    removeTrailingPeriodInDialogue: z.boolean(),
  }),
)

const formRef = $ref<InstanceType<typeof Form>>()

const onSubmit = getSubmitFn(settingsSchema, (values) => {
  settings.$patch(values)
  open.value = false
  notify.success('设置保存成功')
})

const themeOptions = {
  light: '浅色',
  dark: '深色',
  auto: '自动',
}

const settingsKeys = Object.keys(settings.$state) as (keyof typeof settings.$state)[]

async function resetForm() {
  if (formRef) {
    let initialValues = {} as Record<string, unknown>
    for (const key of settingsKeys) {
      initialValues[key] = settings[key]
    }
    formRef.resetForm({ values: initialValues })
  }
}

onMounted(resetForm)
onUpdated(resetForm)
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger as-child>
      <slot />
    </DialogTrigger>
    <DialogContent class="grid-rows-[auto_minmax(0,1fr)_auto] max-h-[85dvh]" @open-auto-focus.prevent>
      <DialogHeader>
        <DialogTitle>设置</DialogTitle>
        <DialogDescription>
          配置用户界面与脚本生成规则
        </DialogDescription>
      </DialogHeader>
      <OverlayScrollbarsComponent defer class="px-2">
        <Form id="settings-form" ref="formRef" :validation-schema="settingsSchema" class="space-y-4" @submit="onSubmit">
          <FormField v-slot="{ componentField }" name="theme">
            <FormItem>
              <FormLabel>主题</FormLabel>
              <Select v-bind="componentField">
                <FormControl>
                  <SelectTrigger class="h-9 max-w-80">
                    <SelectValue />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem v-for="(name, value) in themeOptions" :key="value" :value="value">
                      {{ name }}
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
              <FormDescription>选择应用的主题</FormDescription>
              <FormMessage />
            </FormItem>
          </FormField>
          <FormField v-slot="{ value, handleChange }" name="dialogueAssociateFigure">
            <FormItem class="max-w-120 flex flex-row items-center justify-between rounded-lg">
              <div class="space-y-0.5">
                <FormLabel>关联对话角色</FormLabel>
                <FormDescription>将对话关联到角色以模拟口型同步</FormDescription>
              </div>
              <FormControl>
                <Switch :checked="value" aria-readonly @update:checked="handleChange" />
              </FormControl>
            </FormItem>
          </FormField>
          <FormField v-slot="{ value, handleChange }" name="enableStrictScript">
            <FormItem class="max-w-120 flex flex-row items-center justify-between rounded-lg">
              <div class="space-y-0.5">
                <FormLabel>启用严格脚本模式</FormLabel>
                <FormDescription>对输入脚本的格式和内容进行严格检查</FormDescription>
              </div>
              <FormControl>
                <Switch :checked="value" aria-readonly @update:checked="handleChange" />
              </FormControl>
            </FormItem>
          </FormField>
          <FormField v-slot="{ value, handleChange }" name="removeTrailingPeriodInDialogue">
            <FormItem class="max-w-120 flex flex-row items-center justify-between rounded-lg">
              <div class="space-y-0.5">
                <FormLabel>移除对话末尾的句号</FormLabel>
                <FormDescription>在生成脚本时移除对话末尾的句号</FormDescription>
              </div>
              <FormControl>
                <Switch :checked="value" aria-readonly @update:checked="handleChange" />
              </FormControl>
            </FormItem>
          </FormField>
        </Form>
      </OverlayScrollbarsComponent>
      <DialogFooter>
        <Button class="w-full" form="settings-form" type="submit">
          <Pen class="size-4" />保存更改
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
