<script setup lang="ts">
import { yaml as yamlHighlight } from '@codemirror/lang-yaml'
import { linter, lintGutter } from '@codemirror/lint'
import { githubLight, githubDark } from '@uiw/codemirror-theme-github'
import { OverlayScrollbarsComponent } from 'overlayscrollbars-vue'
import { Codemirror } from 'vue-codemirror'
import YAML from 'yaml'

import type { EditorState, Extension } from '@codemirror/state'
import type { EditorView, PluginValue, ViewPlugin } from '@codemirror/view'

const state = useStateStore()

const settings = useSettingsStore()

const scriptSchema = $(useScriptSchema())

const yamlLinter = linter((view: EditorView) => {
  const doc = YAML.parseDocument(view.state.doc.toString())
  let errors = doc.errors.map((error) => {
    const { pos: [from, to], message } = error
    return {
      from,
      to,
      severity: 'error' as const,
      message,
    }
  })
  const result = scriptSchema.safeParse(doc.toJS())
  if (!result.success) {
    const mainError = result.error.issues[0]
    const node = doc.getIn(mainError.path, true)
    if (YAML.isNode(node) && node.range) {
      const [from, to, _] = node.range
      errors.push({
        from,
        to,
        severity: 'error' as const,
        message: mainError.message,
      })
    }
  }
  state.validated = errors.length === 0
  return errors
})

let { system, store } = inject('themeMode') as { system: ComputedRef<'dark' | 'light'>, store: ComputedRef<'dark' | 'light' | 'auto'> }

const isDark = $computed(() => {
  const currentTheme = store.value === 'auto' ? system.value : store.value
  return currentTheme === 'dark'
})

const extensions = $computed(() => {
  return [yamlHighlight(), lintGutter(), yamlLinter, isDark ? githubDark : githubLight]
})

const ScrollbarsOptions = computed(() => {
  return { scrollbars: { theme: isDark ? 'os-theme-light' : 'os-theme-dark' } } as const
})

let codeMirrorView = $shallowRef<EditorView>()

function handleReady({ view }: {
  view: EditorView
  state: EditorState
  container: HTMLDivElement
}) {
  codeMirrorView = view
}

const [_config, lintPlugin, _exts] = yamlLinter as Extension[]

// based on discussion here:
// https://discuss.codemirror.net/t/can-we-manually-force-linting-even-if-the-document-hasnt-changed/3570
function forceLinting(view: EditorView) {
  const plugin = view.plugin(lintPlugin as ViewPlugin<PluginValue & { set: boolean, force: () => void }>)
  if (plugin) {
    plugin.set = true
    plugin.force()
  }
}

watch(() => [scriptSchema, settings.enableStrictScript], () => {
  if (codeMirrorView) {
    forceLinting(codeMirrorView)
  }
})
</script>

<template>
  <OverlayScrollbarsComponent
    defer
    :options="ScrollbarsOptions"
    class="z-9 flex border rounded-lg"
  >
    <Codemirror
      id="code-editor"
      v-model="state.editorCode"
      placeholder="粘贴 YAML 格式脚本到此处"
      :extensions="extensions"
      :autofocus="true"
      @ready="handleReady"
    />
  </OverlayScrollbarsComponent>
</template>

<style>
#code-editor {
  @apply flex! absolute inset-0 min-h-full max-h-max min-w-full max-w-max w-fit font-mono;

  & .cm-editor {
    @apply w-full;
  }

  & .cm-focused {
    @apply outline-none;
  }

  & .cm-scroller {
    @apply overflow-visible;
  }

  & .cm-tooltip-hover {
    @apply rounded-lg overflow-hidden;
  }
}
</style>
