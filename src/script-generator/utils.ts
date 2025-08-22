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

export function getFigureAction(motions: string[], action: string): string {
  const filteredMotions = motions.filter(motion => motion.includes(action))
  const finalMotions = filteredMotions.length > 0 ? filteredMotions : motions
  return randomChoice(finalMotions)
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
