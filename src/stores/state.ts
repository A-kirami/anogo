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

const defaultActionLink = [
  { key: '生气', value: 'angry' },
  { key: '告别', value: 'bye' },
  { key: '哭泣', value: 'cry' },
  { key: '感动', value: 'kandou' },
  { key: '决心', value: 'kime' },
  { key: '悲伤', value: 'sad' },
  { key: '认真', value: 'serious' },
  { key: '害羞', value: 'shame' },
  { key: '微笑', value: 'smile' },
  { key: '惊讶', value: 'surprised' },
  { key: '思考', value: 'thinking' },
]

export const useStateStore = defineStore('state', () => {
  const editorCode = $ref<string>('')
  const scriptCode = $ref<string>('')
  const validated = $ref(true)
  let webgalPath = $ref<string>()
  let gameRecord = $ref<GameRecord>()
  let selectedGame = $ref<string>()
  let figureRecord = $ref<FigureRecord>()
  const figureLink = $ref<FigureLink>(defaultFigureLink)
  const actionLink = $ref<ActionLink>(defaultActionLink)

  const selectedGameInfo = $computed(() => {
    if (gameRecord && selectedGame) {
      return gameRecord[selectedGame]
    }
  })

  watch($$(webgalPath), async (basePath) => {
    if (!basePath) {
      return
    }
    try {
      gameRecord = await Commands.listGames(basePath)
    } catch (error) {
      webgalPath = undefined
      selectedGame = undefined
      void logger.error(`获取游戏失败: ${error}`)
      return
    }
    if (selectedGame && !(selectedGame in gameRecord)) {
      selectedGame = undefined
    }
  }, { immediate: true })

  watch($$(selectedGameInfo), async (gameInfo) => {
    if (!gameInfo) {
      return
    }
    try {
      const analyzeResult = await Commands.analyzeFigure(gameInfo.path)
      figureRecord = Object.keys(analyzeResult).length > 0 ? analyzeResult : undefined
    } catch (error) {
      notify.error('分析立绘失败')
      void logger.error(`分析立绘失败: ${error}`)
    }
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
    actionLink,
  })
}, {
  persist: {
    pick: ['webgalPath', 'selectedGame', 'figureLink', 'actionLink'],
  },
})
