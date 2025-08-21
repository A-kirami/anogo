import type {
  BackgroundStatement,
  DialogueStatement,
  NarrationStatement,
  SceneStatement,
} from './type'

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
          const figureAction = findAction(state.actionLink, action, figureID) || settings.figureDefaultAction || 'idle'
          const figureMotion = getFigureAction(defaultCostume.motions, figureAction)
          const figureExpression = getFigureAction(defaultCostume.expressions, figureAction)
          const argsMap = {
            id: figureID,
            transform: settings.figureDefaultTransform || undefined,
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

          if (settings.dialogueAssociateFigure) {
            dialogueArray.push(`-id -figureId=${figureID}`)
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

function getDefaultCostume(costumes: Costume[]): Costume {
  const preferred = costumes.find(c => c.name.includes('casual'))
  return preferred || randomChoice(costumes)
}

function findAction(actionLink: ActionLink, key: string, figureID?: string): string | undefined {
  const item = actionLink.find(obj => obj.key === key)
  if (!item?.value) {
    return undefined
  }
  if (figureID && item.value.includes('{name}')) {
    return item.value.replaceAll('{name}', figureID)
  }
  return item.value
}

function getFigureAction(motions: string[], action: string): string {
  const filteredMotions = motions.filter(motion => motion.includes(action))
  const finalMotions = filteredMotions.length > 0 ? filteredMotions : motions
  return randomChoice(finalMotions)
}

function buildArg(key: string, value: unknown): string {
  return value === '' ? `-${key}` : `-${key}=${value}`
}
