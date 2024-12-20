export interface GameInfo {
  icon: string
  path: string
}

export type GameRecord = Record<string, GameInfo>

export interface FigureData {
  name: string
  path: string
  costumes: Costume[]
}

export interface Costume {
  name: string
  path: string
  motions: string[]
  expressions: string[]
}

export type FigureRecord = Record<string, FigureData>

export type FigureLink = Record<string, string[]>
