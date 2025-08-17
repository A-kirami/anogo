import type {
  SceneStatement,
  BackgroundStatement,
  NarrationStatement,
  DialogueStatement,
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

      // 对话句号处理
      if (settings.removeTrailingPeriodInDialogue) {
        const newDialogue = dialogue.replace(/。$/, '')
        dialogueArray.push(newDialogue)
      } else {
        dialogueArray.push(dialogue)
      }

      if (state.figureRecord) {
        const fullPath = getFigureID(state.figureRecord, state.figureLink, figure)
        if (fullPath) {
          const [figureID, costumeName] = fullPath.split('/')
          const character = state.figureRecord[figureID]

          // 优先查找指定的服装
          let costume = costumeName
            ? character?.costumes?.find(c => c.name === costumeName)
            : undefined

          // 如果没有找到指定服装，使用默认服装策略
          if (!costume) {
            costume = getDefaultCostume(character?.costumes)
          }

          if (costume) {
            const figureFile = costume.path
            const figureAction = findAction(state.actionLink, action) || settings.figureDefaultAction || 'idle'

            // get motions / expressions (支持 jsonl 或普通 model.json)
            const figureMotion = getFigureAction(costume.motions ?? [], figureAction)
            const figureExpression = getFigureAction(costume.expressions ?? [], figureAction)

            const argsMap = {
              id: figureID?.split('/').pop(),
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
      }

      result.push(dialogueArray.join(' '))
      return result
    },
  },
] as StrategyObject[]

function getFigureID(
  figureRecord: FigureRecord,
  figureLink: FigureLink,
  name: string,
): string | undefined {
  for (const figureID of Object.keys(figureRecord)) {
    const costumes = figureRecord[figureID].costumes
    if (!costumes) {
      continue
    }

    for (const costume of costumes) {
      const pathKey = `${figureID}/${costume.name}`
      const aliasList = figureLink[pathKey] || []
      if (aliasList.includes(name)) {
        // 返回完整的路径信息，包含服装名称
        return `${figureID}/${costume.name}`
      }
    }
  }
  return undefined
}

/** 默认服装策略 */
function getDefaultCostume(costumes?: Costume[]): Costume | undefined {
  if (!costumes || costumes.length === 0) {
    return undefined
  }
  const preferred = costumes.find(c => c.name.includes('casual'))
  return preferred || randomChoice(costumes)
}

/** 动作映射 */
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

/** 根据关键字过滤动作或表情 */
function getFigureAction(motions: string[], action: string): string {
  if (!motions || motions.length === 0) {
    return ''
  }
  const filteredMotions = motions.filter(motion => motion.includes(action))
  const finalMotions = filteredMotions.length > 0 ? filteredMotions : motions
  return randomChoice(finalMotions)
}

/** 构建 WebGAL 参数 */
function buildArg(key: string, value: unknown): string {
  return value === '' ? `-${key}` : `-${key}=${value}`
}

/** 随机选择 */
function randomChoice<T>(arr: T[]): T {
  return arr[Math.floor(Math.random() * arr.length)]
}
