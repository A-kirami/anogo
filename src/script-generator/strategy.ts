import type { SceneStatement, BackgroundStatement, NarrationStatement, DialogueStatement } from './type'

interface StrategyObject {
  keys: string[]
  handle: (stmt: SceneStatement) => string | string[]
}

export const statementStrategy = [
  {
    keys: ['背景'],
    handle: (stmt: BackgroundStatement) => `changeBg: ${stmt.背景} -next`,
  },
  {
    keys: ['旁白'],
    handle: (stmt: NarrationStatement) => `: ${stmt.旁白}`,
  },
  {
    keys: ['角色', '动作', '对话'],
    handle: (stmt: DialogueStatement) => {
      const state = useStateStore()
      const settings = useSettingsStore()
      const result: string[] = []
      const { 角色: figure, 动作: action, 对话: dialogue } = stmt
      const dialogueArray = [`${figure}:`]
      if (settings.removeTrailingPeriodInDialogue) {
        const newDialogue = dialogue.replace(/。$/, '')
        dialogueArray.push(newDialogue)
      } else {
        dialogueArray.push(dialogue)
      }
      if (state.figureRecord) {
        const figureID = getFigureID(state.figureRecord, state.figureLink, figure)
        if (figureID) {
          const defaultCostume = getDefaultCostume(state.figureRecord[figureID].costumes)
          const figureFile = defaultCostume.path
          const defaultTransform = getDefaultTransform()
          const figureAction = getAction(state.actionLink, action)
          const figureMotion = getFigureAction(defaultCostume.motions, figureAction)
          const figureExpression = getFigureAction(defaultCostume.expressions, figureAction)
          const argsMap = {
            id: figureID,
            transform: defaultTransform,
            next: '',
            motion: figureMotion,
            expression: figureExpression,
          }
          const figureArgs = Object.entries(argsMap)
            .filter(([_, value]) => value !== undefined)
            .map(([key, value]) => buildArg(key, value))
            .join(' ')
          const changeFigureStmt = ['changeFigure:', figureFile, figureArgs].join(' ')
          result.push(changeFigureStmt)

          const settings = useSettingsStore()
          if (settings.dialogueAssociateFigure) {
            dialogueArray.push(`-figureId=${figureID}`)
          }
        }
      }
      result.push(dialogueArray.join(' '))
      return result
    },
  },
] as StrategyObject[]

function getFigureID(figureRecord: FigureRecord, figureLink: FigureLink, name: string): string | undefined {
  for (const id of Object.keys(figureRecord)) {
    const linkNames = figureLink[id] ?? []
    if (linkNames.includes(name)) {
      return id
    }
  }
  return undefined
}

const DEFAULT_ACTION = 'idle'

function getAction(actionLink: ActionLink, key: string): string {
  const item = actionLink.find(obj => obj.key === key)
  return item ? item.value : DEFAULT_ACTION
}

function getDefaultCostume(costumes: Costume[]): Costume {
  for (const costume of costumes) {
    if (costume.name.includes('casual')) {
      return costume
    }
  }
  return randomChoice(costumes)
}

function getFigureAction(motions: string[], action: string): string {
  const filteredMotions = motions.filter(motion => motion.includes(action))
  const finalMotions = filteredMotions.length > 0 ? filteredMotions : motions
  return randomChoice(finalMotions)
}

function getDefaultTransform(): string | undefined {
  return '{"position":{"x":220,"y":-190},"scale":{"x":0.83,"y":0.83}}'
}

function buildArg(key: string, value: unknown): string {
  return value === '' ? `-${key}` : `-${key}=${value}`
}
