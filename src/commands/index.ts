import { invoke } from '@tauri-apps/api/core'

export const Commands = {
  /**
   * 写入文件内容
   * @param contents 文件内容数组
   * @param path 文件路径
   * @param overwrite 是否覆盖
   */
  async writeFile(contents: Uint8Array, path: string, overwrite = false) {
    return await invoke<void>('write_file', { contents, path, overwrite })
  },

  /**
   * 获取游戏列表
   * @param basePath WebGAL 基础路径
   */
  async listGames(basePath: string) {
    return await invoke<GameRecord>('list_games', { basePath })
  },

  /**
   * 分析游戏立绘
   * @param path 游戏路径
   */
  async analyzeFigure(path: string) {
    return await invoke<FigureRecord>('analyze_figure', { path })
  },
}
