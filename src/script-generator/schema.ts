import { z } from 'zod'

const stringOrNumberToString = z.union([z.string(), z.number()]).transform(String)

const backgroundStatementSchema = z.object({
  背景: stringOrNumberToString,
})

const narrationStatementSchema = z.object({
  旁白: stringOrNumberToString,
})

const dialogueStatementSchema = z.object({
  角色: stringOrNumberToString,
  动作: stringOrNumberToString,
  对话: stringOrNumberToString,
})

const sceneStatementSchema = z.union([
  backgroundStatementSchema,
  narrationStatementSchema,
  dialogueStatementSchema,
])

export const sceneScriptSchema = z.array(sceneStatementSchema)
