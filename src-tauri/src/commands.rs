use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use base64::{engine::general_purpose, Engine as _};

fn get_game_dir(base_path: &str,) -> PathBuf {
    let mut path = PathBuf::from(base_path,);
    path.push("public",);
    path.push("games",);
    path
}

fn list_directories(path: &Path,) -> Result<Vec<String,>, String,> {
    let entries =
        fs::read_dir(path,).map_err(|e| format!("无法读取目录'{}': {}", path.display(), e),)?;
    let mut directories = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("目录条目访问失败: {}", e),)?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name,) = path.file_name().and_then(|name| name.to_str(),) {
                directories.push(dir_name.to_string(),);
            }
        }
    }

    Ok(directories,)
}

fn get_icon_path(game_path: &Path,) -> PathBuf {
    game_path.join("icons/icon-192.png",)
}

fn get_icon_data_url(icon_path: &PathBuf,) -> Result<String, String,> {
    if icon_path.exists() {
        let icon_data = fs::read(icon_path,)
            .map_err(|e| format!("无法读取图标文件'{}': {}", icon_path.display(), e),)?;
        let encoded = general_purpose::STANDARD.encode(&icon_data,);
        let data_url = format!("data:image/png;base64,{}", encoded);
        Ok(data_url,)
    } else {
        Ok(String::new(),)
    }
}

#[derive(serde::Serialize,)]
pub struct GameInfo {
    icon: String,
    path: String,
}

#[tauri::command]
pub fn list_games(base_path: String,) -> Result<HashMap<String, GameInfo,>, String,> {
    let game_dir = get_game_dir(&base_path,);
    let directories = list_directories(&game_dir,)?;

    let mut games = HashMap::new();

    for dir_name in directories {
        let game_path = game_dir.join(&dir_name,);
        let icon_path = get_icon_path(&game_path,);
        let icon_data_url = get_icon_data_url(&icon_path,)?;

        games.insert(
            dir_name,
            GameInfo {
                icon: icon_data_url,
                path: game_path.to_string_lossy().into_owned(),
            },
        );
    }

    Ok(games,)
}

#[tauri::command]
pub fn write_file(contents: Vec<u8,>, path: PathBuf, overwrite: bool,) -> Result<(), String,> {
    if overwrite || !path.exists() {
        let mut file = File::create(&path,)
            .map_err(|err| format!("创建文件'{}'失败: {}", path.display(), err),)?;

        file.write_all(&contents,)
            .map_err(|err| format!("写入文件'{}'失败: {}", path.display(), err),)?;

        Ok((),)
    } else {
        Err(format!("文件'{}'已存在且不允许覆盖", path.display()),)
    }
}

use glob::glob;
use serde_json::{Map, Value};

#[tauri::command(async)]
pub fn analyze_figure(path: String,) -> Result<Map<String, Value,>, String,> {
    let figure_dir = Path::new(&path,).join("game/figure",);
    if !figure_dir.is_dir() {
        return Err(format!(
            "路径'{}'中的figure目录不存在",
            figure_dir.display()
        ),);
    }

    let json_files = get_json_files(&figure_dir,)?;
    let model_files = filter_model_files(&json_files,)?;
    let character_data = build_character_data(&figure_dir, &model_files,)?;

    Ok(character_data,)
}

// 实现对jsonl拼好模的支持
// 嗯，jsonl也可以是json
fn get_json_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    use std::collections::HashSet;

    let pattern = format!("{}/**/*", dir.display());
    let mut files_to_return = Vec::new(); // 最终返回文件列表
    let mut skip_dirs: HashSet<PathBuf> = HashSet::new();

    // 阶段1：先记录所有包含 .jsonl 的目录（这些目录及其子目录的 .json 都要跳过）
    for entry in glob(&pattern).map_err(|e| format!("处理glob模式'{}'失败: {}", pattern, e))? {
        match entry {
            Ok(path) => {
                if path.is_file() && path.extension().map(|ext| ext == "jsonl").unwrap_or(false) {
                    // ⚠️ 安全获取父目录；若无父目录（几乎不可能，但做好兜底），回退为传入的 dir
                    let parent_path = path.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| dir.to_path_buf());
                    skip_dirs.insert(parent_path);
                }
            }
            Err(e) => {
                return Err(format!(
                    "阶段1遍历文件失败: {} (路径: {})",
                    e,
                    e.path().display()
                ));
            }
        }
    }

    // 阶段2：收集文件（含 .jsonl；若在 skip_dirs 下则跳过 .json）
    for entry in glob(&pattern).map_err(|e| format!("处理glob模式'{}'失败: {}", pattern, e))? {
        match entry {
            Ok(path) => {
                if path.is_dir() {
                    continue;
                }

                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if ext != "json" && ext != "jsonl" {
                    continue;
                }

                // .exp.json 忽略
                let is_exp_json = path
                    .file_name()
                    .and_then(|f| f.to_str())
                    .map(|name| name.ends_with(".exp.json"))
                    .unwrap_or(false);
                if is_exp_json {
                    continue;
                }

                if ext == "jsonl" {
                    // ✅ jsonl 一律保留
                    files_to_return.push(path);
                    continue;
                }

                // ext == "json" 的情况：若在 skip_dirs（含 .jsonl 的目录）之下，则跳过
                let mut should_skip_json = false;
                for skip_dir in &skip_dirs {
                    if path.starts_with(skip_dir) {
                        should_skip_json = true;
                        break;
                    }
                }
                if should_skip_json {
                    continue;
                }

                files_to_return.push(path);
            }
            Err(e) => {
                return Err(format!(
                    "阶段2遍历文件失败: {} (路径: {})",
                    e,
                    e.path().display()
                ));
            }
        }
    }

    Ok(files_to_return)
}

fn filter_model_files(files: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
    use serde_json::Value;
    const MODEL_SUFFIXES: &[&str] = &["model.json", "model3.json"];

    let mut model_files: Vec<PathBuf> = Vec::new();

    'outer: for file in files {
        let fname = match file.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => {
                log::warn!("跳过无效文件名: {}", file.display());
                continue;
            }
        };

        let is_jsonl = file.extension().map(|ext| ext.eq_ignore_ascii_case("jsonl")).unwrap_or(false);

        if is_jsonl {
            // 直接接受 .jsonl，不再读文件逐行解析
            model_files.push(file.clone());
            continue 'outer;
        }

        // 普通 json：仍按后缀过滤
        if !MODEL_SUFFIXES.iter().any(|s| fname.ends_with(s)) {
            continue;
        }

        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("读取文件'{}'失败: {}", file.display(), e);
                continue;
            }
        };

        match serde_json::from_str::<Value>(&content) {
            Ok(json) => {
                if json.get("model").is_some() && json.get("textures").is_some() {
                    model_files.push(file.clone());
                }
            }
            Err(e) => {
                log::warn!("解析JSON文件'{}'失败: {}", file.display(), e);
            }
        }
    }

    Ok(model_files)
}
fn build_character_data(
    figure_dir: &Path,
    model_files: &[PathBuf],
) -> Result<Map<String, Value>, String> {
    let mut character_data: Map<String, Value> = Map::new();

    for file in model_files {
        let relative_path = file
            .strip_prefix(figure_dir)
            .map_err(|e| {
                format!(
                    "路径转换失败: 无法从'{}'去除前缀'{}': {}",
                    file.display(),
                    figure_dir.display(),
                    e
                )
            })?
            .to_string_lossy()
            .replace('\\', "/");

        let extension = file.extension().and_then(|e| e.to_str()).unwrap_or("");
        if extension == "jsonl" {
            // jsonl 文件：解析最后一行为 motions / expressions
            let content = fs::read_to_string(file)
                .map_err(|e| format!("读取 jsonl 文件 '{}' 失败: {}", file.display(), e))?;

            let mut motions = Vec::new();
            let mut expressions = Vec::new();
            for line in content.lines().rev() {
                if let Ok(json) = serde_json::from_str::<Value>(line) {
                    if json.get("motions").is_some() || json.get("expressions").is_some() {
                        motions = json.get("motions")
                            .and_then(|v| v.as_array())
                            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                            .unwrap_or_default();

                        expressions = json.get("expressions")
                            .and_then(|v| v.as_array())
                            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                            .unwrap_or_default();
                        break;
                    }
                }
            }

            // 使用与json文件相同的逻辑来提取角色名称和路径
            let (costume_name, character_name) = match get_names_from_path(file) {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("跳过无效jsonl模型 '{}': {}", file.display(), e);
                    continue;
                }
            };

            let character_path = match get_character_path(file) {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("跳过无效jsonl模型 '{}': {}", file.display(), e);
                    continue;
                }
            };

            let character_info = character_data
                .entry(character_name.clone())
                .or_insert_with(|| {
                    serde_json::json!({
                        "name": character_name,
                        "path": character_path,
                        "costumes": []
                    })
                });

            if let Some(costumes_array) = character_info.get_mut("costumes").and_then(|c| c.as_array_mut()) {
                costumes_array.push(serde_json::json!({
                    "name": costume_name,
                    "path": relative_path,
                    "motions": motions,
                    "expressions": expressions
                }));
            }

            continue;
        }

        // 普通 model.json 文件处理
        let (costume_name, character_name) = match get_names_from_path(file) {
            Ok(v) => v,
            Err(e) => {
                log::warn!("跳过无效模型 '{}': {}", file.display(), e);
                continue;
            }
        };

        let character_path = match get_character_path(file) {
            Ok(v) => v,
            Err(e) => {
                log::warn!("跳过无效模型 '{}': {}", file.display(), e);
                continue;
            }
        };

        let content = fs::read_to_string(file)
            .map_err(|e| format!("读取角色文件'{}'失败: {}", file.display(), e))?;

        let model: Value = serde_json::from_str(&content)
            .map_err(|e| format!("解析角色文件'{}'失败: {}", file.display(), e))?;

        let motions: Vec<String> = model
            .get("motions")
            .and_then(|m| m.as_object())
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();

        let expressions: Vec<String> = model
            .get("expressions")
            .and_then(|e| e.as_array())
            .map(|e| {
                e.iter()
                    .filter_map(|expr| expr.get("name").and_then(|n| n.as_str().map(|s| s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let character_info = character_data
            .entry(character_name.clone())
            .or_insert_with(|| {
                serde_json::json!({
                    "name": character_name,
                    "path": character_path,
                    "costumes": []
                })
            });

        if let Some(costumes_array) = character_info.get_mut("costumes").and_then(|c| c.as_array_mut()) {
            costumes_array.push(serde_json::json!({
                "name": costume_name,
                "path": relative_path,
                "motions": motions,
                "expressions": expressions
            }));
        }
    }

    Ok(character_data)
}


fn get_names_from_path(path: &Path,) -> Result<(String, String,), String,> {
    let parts: Vec<_,> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str(),)
        .collect();

    if parts.len() < 2 {
        return Err(format!("路径'{}'中未找到足够的路径组件", path.display()),);
    }

    let costume = parts[parts.len() - 2].to_string();

    let character = parts
        .get(parts.len().saturating_sub(3,),)
        .filter(|&&s| s != "figure",)
        .map(|&s| s.to_string(),)
        .unwrap_or_else(|| costume.clone(),);

    Ok((costume, character,),)
}

fn get_character_path(path: &Path,) -> Result<String, String,> {
    let parts: Vec<_,> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str(),)
        .collect();

    if parts.len() < 2 {
        return Err(format!("路径'{}'中未找到足够的路径组件", path.display()),);
    }

    let costume = parts[parts.len() - 2].to_string();

    let figure_idx = parts[..parts.len() - 2]
        .iter()
        .rposition(|&p| p == "figure",)
        .map(|i| i + 1,)
        .unwrap_or(0,);

    let char_parts = &parts[figure_idx..parts.len() - 2];
    if char_parts.is_empty() {
        return Ok(costume,);
    }

    Ok(char_parts.join("/",),)
}
