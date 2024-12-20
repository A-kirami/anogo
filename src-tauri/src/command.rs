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
    let entries = fs::read_dir(path,).map_err(|e| e.to_string(),)?;
    let mut directories = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string(),)?;
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
        let icon_data = fs::read(icon_path,).map_err(|e| e.to_string(),)?;
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
        let mut file = match File::create(&path,) {
            Ok(file,) => file,
            Err(err,) => return Err(format!("创建文件失败: {}", err),),
        };

        if let Err(err,) = file.write_all(&contents,) {
            return Err(format!("写入文件失败: {}", err),);
        }

        Ok((),)
    } else {
        Err("文件已存在".into(),)
    }
}

use glob::glob;
use serde_json::{Map, Value};

#[tauri::command(async)]
pub fn analyze_figure(path: String,) -> Result<Map<String, Value,>, String,> {
    let figure_dir = Path::new(&path,).join("game/figure",);
    if !figure_dir.is_dir() {
        return Err("figure 目录不存在".to_string(),);
    }

    let json_files = get_json_files(&figure_dir,)?;
    let model_files = filter_model_files(&json_files,)?;
    let character_data = build_character_data(&figure_dir, &model_files,)?;

    Ok(character_data,)
}

fn get_json_files(dir: &Path,) -> Result<Vec<PathBuf,>, String,> {
    let pattern = dir.join("**/*.json",).to_str().unwrap().to_string();
    let mut files = Vec::new();
    for entry in glob(&pattern,).expect("无法读取 glob 模式",) {
        match entry {
            Ok(path,) => files.push(path,),
            Err(e,) => return Err(format!("读取文件失败: {}", e),),
        }
    }
    Ok(files,)
}

fn filter_model_files(files: &[PathBuf],) -> Result<Vec<PathBuf,>, String,> {
    let mut model_files = Vec::new();
    for file in files {
        let content = fs::read_to_string(file,).map_err(|e| e.to_string(),)?;
        let json: Value = serde_json::from_str(&content,).map_err(|e| e.to_string(),)?;
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
            .map_err(|e| e.to_string(),)?
            .to_string_lossy()
            .replace('\\', "/",);
        let content = fs::read_to_string(file,).map_err(|e| e.to_string(),)?;
        let model: Value = serde_json::from_str(&content,).map_err(|e| e.to_string(),)?;

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
        .ok_or("路径无效：未找到服装名称",)?;
    let character_name = components
        .next()
        .and_then(|c| c.as_os_str().to_str().map(|s| s.to_string(),),)
        .ok_or("路径无效：未找到角色名称",)?;
    Ok((costume_name, character_name,),)
}

fn get_character_path(path: &Path,) -> Result<String, String,> {
    let mut components = path.components().rev();

    components.next();
    components.next();

    let mut path_components = Vec::new();
    for component in components {
        if component.as_os_str() == "figure" {
            break;
        }
        path_components.push(component,);
    }

    if path_components.is_empty() {
        return Err("路径无效：未找到 figure 目录".to_string(),);
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
