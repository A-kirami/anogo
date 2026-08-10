import { randomChoice } from '../utils'

export interface FigureActionScope {
  currentFigureId: string
  figureIds: readonly string[]
}

export function getFigureId(figureRecord: FigureRecord, figureLink: FigureLink, name: string): string | undefined {
  for (const id of Object.keys(figureRecord)) {
    const linkNames = figureLink[id] ?? []
    if (linkNames.includes(name)) {
      return id
    }
  }
  return undefined
}

export function getDefaultCostume(costumes: Costume[]): Costume {
  const preferred = costumes.find(c => c.name.includes('casual'))
  return preferred || randomChoice(costumes)
}

export function findAction(actionLink: ActionLink, key: string, figureID?: string): string | undefined {
  const item = actionLink.find(obj => obj.key === key)
  if (!item?.value) {
    return undefined
  }
  if (figureID && item.value.includes('{name}')) {
    return item.value.replaceAll('{name}', figureID)
  }
  return item.value
}

export function getFigureAction(actions: string[], action: string, allowFallback = true, scope?: FigureActionScope): string | undefined {
  const scopedActions = getFigureActions(actions, scope)
  return selectFigureAction(scopedActions, action, allowFallback)
}

export function getFigureExpression(expressions: string[], action: string, scope?: FigureActionScope): string | undefined {
  const scopedExpressions = getFigureActions(expressions, scope)
  return selectFigureAction(scopedExpressions, action, false) ?? selectFigureAction(scopedExpressions, 'default', false)
}

function getFigureActions(actions: string[], scope?: FigureActionScope): string[] {
  if (!scope) {
    return actions
  }

  const isScopedAction = (action: string) => scope.figureIds.some(figureId => action.includes(figureId))
  if (!actions.some(action => isScopedAction(action))) {
    return actions
  }

  return actions.filter(action => !isScopedAction(action) || action.includes(scope.currentFigureId))
}

function selectFigureAction(actions: string[], action: string, allowFallback: boolean): string | undefined {
  const matchingActions = actions.filter(item => item.includes(action))
  if (matchingActions.length > 0) {
    return randomChoice(matchingActions)
  }

  return allowFallback && actions.length > 0 ? randomChoice(actions) : undefined
}

export function buildStatement(command: string = '', content: string = '', args: Record<string, unknown> = {}, isNext: boolean = false): string {
  const parts: string[] = [`${command}:`]
  if (content) {
    parts.push(content)
  }

  const argParts = Object.entries(args)
    .filter(([_, value]) => value !== undefined)
    .map(([key, value]) => value === '' ? `-${key}` : `-${key}=${value}`)
    .join(' ')

  if (argParts) {
    parts.push(argParts)
  }
  if (isNext) {
    parts.push('-next')
  }

  return parts.join(' ')
}
