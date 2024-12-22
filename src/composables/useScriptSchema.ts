import { sceneScriptSchema, getStrictSchema } from '~/script-generator/schema'

export default function useScriptSchema() {
  const state = useStateStore()
  const settings = useSettingsStore()

  const strictSceneScriptSchema = $computed(() => {
    const actionsEnum = state.actionLink.map(item => item.key) as [string, ...string[]]
    return getStrictSchema(actionsEnum)
  })

  const scriptSchema = $computed(() => {
    return settings.enableStrictScript ? strictSceneScriptSchema : sceneScriptSchema
  })

  return $$(scriptSchema)
}
