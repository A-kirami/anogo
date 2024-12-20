export interface BackgroundStatement {
  背景: string
}

export interface NarrationStatement {
  旁白: string
}

export interface DialogueStatement {
  角色: string
  动作: EmotionAction
  对话: string
}

export type EmotionAction =
  | '生气'
  | '告别'
  | '哭泣'
  | '感动'
  | '决心'
  | '悲伤'
  | '认真'
  | '害羞'
  | '微笑'
  | '惊讶'
  | '思考'

export type SceneStatement = BackgroundStatement | NarrationStatement | DialogueStatement

export type SceneScript = SceneStatement[]
