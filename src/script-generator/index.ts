import { eol as getEOL } from '@tauri-apps/plugin-os'
import YAML from 'yaml'

import { statementStrategy } from './strategy'

import type { SceneScript, SceneStatement } from './type'

function hasAllProperties(stmt: SceneStatement, props: string[]): boolean {
  return props.every(prop => prop in stmt)
}

function processSceneScript(script: SceneScript): string[] {
  const state = useStateStore()
  const settings = useSettingsStore()
  return script.flatMap((statement) => {
    const strategy = statementStrategy.find(s => hasAllProperties(statement, s.keys))
    if (!strategy) {
      void logger.error(`未知类型的语句: \n${YAML.stringify(statement)}`)
      return []
    }
    const result = strategy.handle(statement, state, settings)
    return typeof result === 'string' ? result : [...result]
  })
}

export function generateScript(sceneScript: SceneScript): string {
  const scriptsStatements = processSceneScript(sceneScript)
  const separator = `;${getEOL()}`
  return scriptsStatements.join(separator) + separator
}
