mod music;
mod player;

use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use tauri::{Manager, State};

use crate::music::{extract_waveform, extract_waveform_streaming, get_music, Track};
use crate::player::{Player, PlayerError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MusicError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Player error: {0}")]
    Player(#[from] PlayerError),
    #[error("Mutex poisoned")]
    MutexPoisoned,
}

#[derive(Default)]
struct AppState {
    player: Option<Arc<Mutex<Player>>>,
    current_path: Option<String>,
}

#[tauri::command]
fn get_musics(paths: Vec<String>) -> Vec<Track> {
    get_music(paths)
}
#[tauri::command]
async fn set_music(state: State<'_, Mutex<AppState>>, path: String) -> Result<(), String> {
    if !Path::new(&path).exists() {
        return Err(MusicError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File does not exist",
        ))
        .to_string());
    }

    std::fs::File::open(&path).expect("File does not exist");

    let mut state = state
        .lock()
        .map_err(|_| MusicError::MutexPoisoned)
        .expect("State is poisoned");

    let player = Player::new(&path).expect("Failed to create player");
    state.player = Some(Arc::new(Mutex::new(player)));
    state.current_path = Some(path);

    Ok(())
}

#[tauri::command]
async fn get_time(state: State<'_, Mutex<AppState>>) -> Result<f32, String> {
    if let Some(player) = &state.lock().unwrap().player {
        Ok(player.lock().unwrap().current_time().unwrap())
    } else {
        Err("Нет активного трека".to_string())
    }
}

#[tauri::command]
async fn play_music(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    if let Some(player) = &state.lock().unwrap().player {
        player.lock().unwrap().resume();
        Ok(())
    } else {
        Err("Нет активного трека".to_string())
    }
}

#[tauri::command]
async fn stop_music(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    if let Some(player) = &state.lock().unwrap().player {
        player.lock().unwrap().pause();
        Ok(())
    } else {
        Err("Нет активного трека".to_string())
    }
}

#[tauri::command]
async fn set_speed(state: State<'_, Mutex<AppState>>, speed: f32) -> Result<(), String> {
    if let Some(player) = &state.lock().unwrap().player {
        player.lock().unwrap().set_speed(speed);
        Ok(())
    } else {
        Err("Нет активного трека".to_string())
    }
}

#[tauri::command]
async fn seek_music(state: State<'_, Mutex<AppState>>, sec: f32) -> Result<(), String> {
    if let Some(player) = &state.lock().unwrap().player {
        player.lock().unwrap().seek(sec);
        Ok(())
    } else {
        Err("Нет активного трека".to_string())
    }
}

#[tauri::command]
async fn set_volume(state: State<'_, Mutex<AppState>>, volume: f32) -> Result<(), String> {
    if let Some(player) = &state.lock().unwrap().player {
        player.lock().unwrap().set_volume(volume);
        Ok(())
    } else {
        Err("Нет активного трека".to_string())
    }
}

#[tauri::command]
async fn get_wave(path: String, points: usize) -> Result<Vec<f32>, String> {
    let wave = extract_waveform_streaming(path, points).await;

    match wave {
        Ok(w) => Ok(w),
        Err(_) => Err("Произошла ошибка".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_musics, set_music, play_music, stop_music, set_volume, set_speed, seek_music,
            get_wave, get_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
