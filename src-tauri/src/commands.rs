use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use base64::{engine::general_purpose, Engine as _};
use glob::glob;
use serde_json::{Map, Value};

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

#[tauri::command(async)]
pub fn analyze_figure(path: String,) -> Result<Map<String, Value,>, String,> {
    let figure_dir = Path::new(&path,).join("game/figure",);
    if !figure_dir.is_dir() {
        return Err(format!(
            "路径'{}'中的figure目录不存在",
            figure_dir.display()
        ),);
    }

    let found_model_files = find_model_files(&figure_dir,)?;
    let valid_model_files = filter_model_files(&found_model_files,)?;
    let character_data = build_character_data(&figure_dir, &valid_model_files,)?;

    Ok(character_data,)
}

const MODEL_SUFFIXES: &[&str] = &["model.json", "model3.json",];

fn find_model_files(dir: &Path,) -> Result<Vec<PathBuf,>, String,> {
    let json_pattern = format!("{}/**/*.json", dir.display());
    let jsonl_pattern = format!("{}/**/*.jsonl", dir.display());

    let mut files = Vec::new();

    for entry in glob(&json_pattern,)
        .unwrap()
        .chain(glob(&jsonl_pattern,).unwrap(),)
    {
        match entry {
            Ok(path,) => {
                if let Some(fname,) = path.file_name().and_then(|n| n.to_str(),) {
                    if MODEL_SUFFIXES.iter().any(|s| fname.ends_with(s,),)
                        || path
                            .extension()
                            .is_some_and(|ext| ext.eq_ignore_ascii_case("jsonl",),)
                    {
                        files.push(path,);
                    }
                }
            }
            Err(e,) => {
                return Err(format!(
                    "搜寻模型文件失败: {} (路径: {})",
                    e,
                    e.path().display()
                ),);
            }
        }
    }

    Ok(files,)
}

const REQUIRED_FIELDS: &[&str] = &["model", "physics", "textures", "motions", "expressions",];

pub fn validate_model_file(file: &PathBuf,) -> bool {
    let content = match fs::read_to_string(file,) {
        Ok(content,) => content,
        Err(e,) => {
            log::warn!("读取模型文件'{}'失败: {}", file.display(), e);
            return false;
        }
    };

    let model: Value = match serde_json::from_str(&content,) {
        Ok(model,) => model,
        Err(e,) => {
            log::warn!("解析模型文件'{}'失败: {}", file.display(), e);
            return false;
        }
    };

    REQUIRED_FIELDS
        .iter()
        .all(|field| model.get(field,).is_some(),)
}

fn filter_model_files(files: &[PathBuf],) -> Result<Vec<PathBuf,>, String,> {
    let mut model_files: Vec<PathBuf,> = Vec::new();

    let jsonl_parent_dirs: Vec<PathBuf,> = files
        .iter()
        .filter(|file| {
            file.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("jsonl",),)
        },)
        .filter_map(|file| file.parent().map(|p| p.to_path_buf(),),)
        .collect();

    for file in files {
        let is_composite_model = file
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("jsonl",),);

        if !is_composite_model && jsonl_parent_dirs.iter().any(|dir| file.starts_with(dir,),) {
            continue;
        }

        if is_composite_model || validate_model_file(file,) {
            model_files.push(file.clone(),);
        }
    }

    Ok(model_files,)
}

fn build_character_data(
    figure_dir: &Path,
    model_files: &[PathBuf],
) -> Result<Map<String, Value,>, String,> {
    let mut character_data = Map::new();

    for file in model_files {
        let relative_path = file
            .strip_prefix(figure_dir,)
            .map_err(|e| {
                format!(
                    "路径转换失败: 无法从'{}'去除前缀'{}': {}",
                    file.display(),
                    figure_dir.display(),
                    e
                )
            },)?
            .to_string_lossy()
            .replace("\\", "/",);

        let extension = file.extension().and_then(|e| e.to_str(),).unwrap_or("",);

        let (motions, expressions,) = match extension {
            "jsonl" => parse_jsonl_model(file,)?,
            _ => parse_json_model(file,)?,
        };

        let (costume_name, character_name,) = get_names_from_path(file,)?;
        let character_path = get_character_path(file,)?;

        let character_info = character_data.entry(&character_name).or_insert_with(|| {
            serde_json::json!({ "name": character_name, "path": character_path, "costumes": [], "is_composite": extension.eq_ignore_ascii_case("jsonl",) })
        });

        if let Some(costumes_array,) = character_info
            .get_mut("costumes",)
            .and_then(|c| c.as_array_mut(),)
        {
            costumes_array.push(serde_json::json!({
                "name": costume_name,
                "path": relative_path,
                "motions": motions,
                "expressions": expressions
            }),);
        }
    }

    Ok(character_data,)
}

fn extract_model_actions(json: &Value,) -> (Vec<String,>, Vec<String,>,) {
    let motions = json
        .get("motions",)
        .and_then(|m| m.as_object(),)
        .map(|m| m.keys().cloned().collect(),)
        .unwrap_or_default();

    let expressions = json
        .get("expressions",)
        .and_then(|e| e.as_array(),)
        .map(|e| {
            e.iter()
                .filter_map(|expr| {
                    expr.get("name",)
                        .and_then(|n| n.as_str().map(|s| s.to_string(),),)
                },)
                .collect()
        },)
        .unwrap_or_default();

    (motions, expressions,)
}

fn parse_json_model(file: &Path,) -> Result<(Vec<String,>, Vec<String,>,), String,> {
    let content = fs::read_to_string(file,)
        .map_err(|e| format!("读取模型文件'{}'失败: {}", file.display(), e),)?;

    let model: Value = serde_json::from_str(&content,)
        .map_err(|e| format!("解析模型文件'{}'失败: {}", file.display(), e),)?;

    Ok(extract_model_actions(&model,),)
}

fn parse_jsonl_model(file: &Path,) -> Result<(Vec<String,>, Vec<String,>,), String,> {
    let content = fs::read_to_string(file,)
        .map_err(|e| format!("读取聚合模型文件 '{}' 失败: {}", file.display(), e),)?;

    for line in content.lines().rev() {
        if let Ok(json,) = serde_json::from_str::<Value,>(line,) {
            if json.get("motions",).is_some() || json.get("expressions",).is_some() {
                return Ok(extract_model_actions(&json,),);
            }
        }
    }

    Err(format!("未在文件'{}'中找到有效的模型数据", file.display()),)
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
