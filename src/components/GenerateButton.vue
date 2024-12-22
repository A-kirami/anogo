<script setup lang="ts">
import { ScrollText } from 'lucide-vue-next'
import YAML from 'yaml'

import { generateScript } from '~/script-generator'

const state = useStateStore()

let openResult = $ref(false)

const scriptSchema = $(useScriptSchema())

function generateResult() {
  try {
    const schemaScript = YAML.parse(state.editorCode)
    const parseResult = scriptSchema.safeParse(schemaScript)
    if (!parseResult.success) {
      throw parseResult.error
    }
    try {
      state.scriptCode = generateScript(parseResult.data)
      openResult = true
    } catch (error) {
      notify.error('生成脚本失败')
      void logger.error(`生成脚本失败: ${error}`)
    }
  } catch (error) {
    notify.error('解析 YAML 脚本失败')
    void logger.error(`解析 YAML 脚本失败: ${error}`)
  }
}
</script>

<template>
  <Button class="w-full" :disabled="!state.editorCode || !state.validated" @click="generateResult">
    <ScrollText class="h-4 w-4" />生成脚本
  </Button>
  <GenerateResultDialog v-model:open="openResult" />
</template>
