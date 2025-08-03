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

fn get_json_files(dir: &Path,) -> Result<Vec<PathBuf,>, String,> {
    let pattern = dir
        .join("**/*.json",)
        .to_str()
        .ok_or("无效的非UTF8路径",)?
        .to_string();

    let mut files = Vec::new();
    for entry in glob(&pattern,).map_err(|e| format!("处理glob模式'{}'失败: {}", pattern, e),)?
    {
        match entry {
            Ok(path,) => files.push(path,),
            Err(e,) => {
                return Err(format!(
                    "遍历文件失败: {} (路径: {})",
                    e,
                    e.path().display()
                ),)
            }
        }
    }
    Ok(files,)
}

fn filter_model_files(files: &[PathBuf],) -> Result<Vec<PathBuf,>, String,> {
    let mut model_files = Vec::new();
    for file in files {
        let content = fs::read_to_string(file,)
            .map_err(|e| format!("读取文件'{}'失败: {}", file.display(), e),)?;

        let json: Value = match serde_json::from_str(&content,) {
            Ok(json,) => json,
            Err(e,) => {
                log::warn!("解析JSON文件'{}'失败: {}", file.display(), e);
                continue;
            }
        };

        if json.get("model",).is_some()
            && json.get("physics",).is_some()
            && json.get("textures",).is_some()
            && json.get("motions",).is_some()
            && json.get("expressions",).is_some()
        {
            model_files.push(file.to_path_buf(),);
        }
    }
    Ok(model_files,)
}

fn build_character_data(
    figure_dir: &Path,
    model_files: &[PathBuf],
) -> Result<Map<String, Value,>, String,> {
    let mut character_data: Map<String, Value,> = Map::new();
    for file in model_files {
        let (costume_name, character_name,) = get_names_from_path(file,)?;
        let character_path = get_character_path(file,)?;

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
            .replace('\\', "/",);
        let content = fs::read_to_string(file,)
            .map_err(|e| format!("读取角色文件'{}'失败: {}", file.display(), e),)?;
        let model: Value = serde_json::from_str(&content,)
            .map_err(|e| format!("解析角色文件'{}'失败: {}", file.display(), e),)?;

        let motions: Vec<String,> = model
            .get("motions",)
            .and_then(|m| m.as_object(),)
            .map(|m| m.keys().cloned().collect(),)
            .unwrap_or_default();
        let expressions: Vec<String,> = model
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

        let character_info = character_data
            .entry(character_name.clone(),)
            .or_insert_with(|| {
                serde_json::json!({
                    "name": character_name,
                    "path": character_path,
                    "costumes": []
                })
            },);

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

fn get_names_from_path(path: &Path,) -> Result<(String, String,), String,> {
    let mut components = path.components().rev();

    components.next();

    let costume_name = components
        .next()
        .and_then(|c| c.as_os_str().to_str().map(|s| s.to_string(),),)
        .ok_or_else(|| format!("路径'{}'中未找到服装名称", path.display()),)?;

    let character_name = if let Some(character_component,) = components.next() {
        if character_component.as_os_str() == "figure" {
            costume_name.clone()
        } else {
            character_component
                .as_os_str()
                .to_str()
                .map(|s| s.to_string(),)
                .ok_or_else(|| format!("路径'{}'中未找到角色名称", path.display()),)?
        }
    } else {
        costume_name.clone()
    };

    Ok((costume_name, character_name,),)
}

fn get_character_path(path: &Path,) -> Result<String, String,> {
    let mut components = path.components().rev();

    components.next();

    let costume_name = components
        .next()
        .and_then(|c| c.as_os_str().to_str().map(|s| s.to_string(),),)
        .ok_or_else(|| format!("路径'{}'中未找到服装名称", path.display()),)?;

    let mut path_components = Vec::new();
    for component in components {
        if component.as_os_str() == "figure" {
            break;
        }
        path_components.push(component,);
    }

    if path_components.is_empty() {
        return Ok(costume_name,);
    }

    path_components.reverse();

    let character_path = path_components
        .iter()
        .map(|c| c.as_os_str().to_str().unwrap(),)
        .collect::<PathBuf>()
        .to_string_lossy()
        .replace('\\', "/",);

    Ok(character_path,)
}
