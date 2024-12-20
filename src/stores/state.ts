import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

const defaultFigureLink = {
  anon: ['千早爱音', '爱音'],
  soyo: ['长崎素世', '素世', '长崎爽世', '爽世'],
  tomori: ['高松灯', '灯'],
  taki: ['椎名立希', '立希'],
  rana: ['要乐奈', '乐奈'],
  sakiko: ['丰川祥子', '祥子'],
  umiri: ['八幡海铃', '海铃'],
  uika: ['三角初华', '初华'],
  nyamu: ['祐天寺若麦', '喵梦'],
  mutsumi: ['若叶睦', '睦'],
  mana: ['纯田真奈', '真奈'],
}

export const useStateStore = defineStore('state', () => {
  const editorCode = $ref<string>('')
  const scriptCode = $ref<string>('')
  const validated = $ref(true)
  const webgalPath = $ref<string>()
  let gameRecord = $ref<GameRecord>()
  let selectedGame = $ref<string>()
  let figureRecord = $ref<FigureRecord>()
  const figureLink = $ref<FigureLink>(defaultFigureLink)

  const selectedGameInfo = $computed(() => {
    if (gameRecord && selectedGame) {
      return gameRecord[selectedGame]
    }
  })

  watch($$(webgalPath), async (basePath) => {
    if (!basePath) {
      return
    }
    gameRecord = await invoke<GameRecord>('list_games', { basePath })
    if (selectedGame && !(selectedGame in gameRecord)) {
      selectedGame = undefined
    }
  }, { immediate: true })

  watch($$(selectedGameInfo), async (gameInfo) => {
    if (!gameInfo) {
      return
    }
    const analyzeResult = await invoke<FigureRecord>('analyze_figure', { path: gameInfo.path })
    figureRecord = Object.keys(analyzeResult).length > 0 ? analyzeResult : undefined
  }, { immediate: true })

  return $$({
    editorCode,
    scriptCode,
    validated,
    webgalPath,
    gameRecord,
    selectedGame,
    selectedGameInfo,
    figureRecord,
    figureLink,
  })
}, {
  persist: {
    pick: ['webgalPath', 'selectedGame', 'figureLink'],
  },
})
