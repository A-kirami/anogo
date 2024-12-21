export interface BackgroundStatement {
  背景: string
}

export interface NarrationStatement {
  旁白: string
}

export interface DialogueStatement {
  角色: string
  动作: string
  对话: string
}

export type SceneStatement = BackgroundStatement | NarrationStatement | DialogueStatement

export type SceneScript = SceneStatement[]
