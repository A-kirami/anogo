import { buildStatement, findAction, getDefaultCostume, getFigureAction, getFigureId } from './utils'

import type {
  BackgroundStatement,
  DialogueStatement,
  NarrationStatement,
  SceneStatement,
} from './type'

interface StrategyObject {
  keys: string[]
  handle: (stmt: SceneStatement, state: ReturnType<typeof useStateStore>, settings: ReturnType<typeof useSettingsStore>) => string | Iterable<string>
}

export const statementStrategy = [
  {
    keys: ['背景'],
    handle: (stmt: BackgroundStatement) => buildStatement('changeBg', stmt.背景, {}, true),
  },
  {
    keys: ['旁白'],
    handle: (stmt: NarrationStatement) => buildStatement('', stmt.旁白),
  },
  {
    keys: ['角色', '动作', '对话'],
    * handle(stmt: DialogueStatement, state, settings) {
      const { 角色: figure, 动作: action, 对话: dialogue } = stmt
      let dialogueArgs: Record<string, unknown> = {}

      if (state.figureRecord) {
        const figureId = getFigureId(state.figureRecord, state.figureLink, figure)
        if (figureId) {
          const defaultCostume = getDefaultCostume(state.figureRecord[figureId].costumes)
          const figureFile = defaultCostume.path
          const figureAction = findAction(state.actionLink, action, figureId) || settings.figureDefaultAction || 'idle'
          const figureMotion = getFigureAction(defaultCostume.motions, figureAction)
          const figureExpression = getFigureAction(defaultCostume.expressions, figureAction)
          const figureArgs = {
            id: figureId,
            transform: settings.figureDefaultTransform || undefined,
            motion: figureMotion,
            expression: figureExpression,
          }
          yield buildStatement('changeFigure', figureFile, figureArgs, true)

          if (settings.dialogueAssociateFigure) {
            dialogueArgs = { id: '', figureId }
          }
        }
      }

      const newDialogue = settings.removeTrailingPeriodInDialogue ? dialogue.replace(/。$/, '') : dialogue
      yield buildStatement(figure, newDialogue, dialogueArgs)
    },
  },
] as StrategyObject[]
