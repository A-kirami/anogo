import { z } from 'zod'

const emotionActionSchema = z.enum([
  '生气', '告别', '哭泣', '感动', '决心', '悲伤', '认真', '害羞', '微笑', '惊讶', '思考',
])

const stringOrNumberToString = z.union([z.string(), z.number()]).transform(String)

const backgroundStatementSchema = z.object({
  背景: stringOrNumberToString,
})

const narrationStatementSchema = z.object({
  旁白: stringOrNumberToString,
})

const dialogueStatementSchema = z.object({
  角色: stringOrNumberToString,
  动作: emotionActionSchema,
  对话: stringOrNumberToString,
})

const sceneStatementSchema = z.union([
  backgroundStatementSchema,
  narrationStatementSchema,
  dialogueStatementSchema,
])

export const sceneScriptSchema = z.array(sceneStatementSchema)
