import { eol as getEOL } from '@tauri-apps/plugin-os'
import YAML from 'yaml'

import { statementStrategy } from './strategy'

import type { SceneScript, SceneStatement } from './type'

function hasAllProperties(stmt: SceneStatement, props: string[]): boolean {
  return props.every(prop => prop in stmt)
}

function processSceneScript(script: SceneScript): (string | string[])[] {
  const results: (string | string[])[] = []

  for (const statement of script) {
    let processed = false

    for (const strategy of statementStrategy) {
      if (hasAllProperties(statement, strategy.keys)) {
        results.push(strategy.handle(statement))
        processed = true
        break
      }
    }

    if (!processed) {
      void logger.error(`未知类型的语句: \n${YAML.stringify(statement)}`)
    }
  }

  return results
}

export function generateScript(sceneScript: SceneScript): string {
  const scriptsStatements = processSceneScript(sceneScript)
  const separator = `;${getEOL()}`
  return scriptsStatements.flat().join(separator) + separator
}
