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

export function getStrictSchema(actions: readonly [string, ...string[]]) {
  const strictActions = z.enum(actions)
  const dialogueStatementStrictSchema = z.object({
    角色: stringOrNumberToString,
    动作: strictActions,
    对话: stringOrNumberToString,
  })
  const sceneStatementStrictSchema = z.union([
    backgroundStatementSchema,
    narrationStatementSchema,
    dialogueStatementStrictSchema,
  ])
  return z.array(sceneStatementStrictSchema)
}
