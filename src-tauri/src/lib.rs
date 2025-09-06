mod music;

use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
    time::Duration,
};

use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};
use tauri::{Manager, State};

use crate::music::{extract_waveform, extract_waveform_fast, get_music, read_audio_samples, Track};

#[derive(Default)]
struct AppState {
    stream: Option<OutputStream>, // держим поток живым
    sink: Option<Arc<Sink>>,
    current_path: Option<String>, // чтобы знать какой файл проигрывается
}

#[tauri::command]
fn get_musics(paths: Vec<String>) -> Vec<Track> {
    get_music(paths)
}

#[tauri::command]
async fn play_music(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let state = state
        .lock()
        .map_err(|_| "Ошибка блокировки состояния".to_string())?;
    if let Some(sink) = &state.sink {
        sink.play();
        Ok(())
    } else {
        Err("Нет активного Sink".to_string())
    }
}

#[tauri::command]
async fn stop_music(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let state = state
        .lock()
        .map_err(|_| "Ошибка блокировки состояния".to_string())?;
    if let Some(sink) = &state.sink {
        sink.pause();
        Ok(())
    } else {
        Err("Нет активного Sink".to_string())
    }
}

#[tauri::command]
async fn set_volume(state: State<'_, Mutex<AppState>>, volume: f32) -> Result<(), String> {
    let state = state
        .lock()
        .map_err(|_| "Ошибка блокировки состояния".to_string())?;
    if let Some(sink) = &state.sink {
        sink.set_volume(volume);
        Ok(())
    } else {
        Err("Нет активного Sink".to_string())
    }
}

#[tauri::command]
async fn set_speed(state: State<'_, Mutex<AppState>>, speed: f32) -> Result<(), String> {
    let state = state
        .lock()
        .map_err(|_| "Ошибка блокировки состояния".to_string())?;
    if let Some(sink) = &state.sink {
        sink.set_speed(speed);
        Ok(())
    } else {
        Err("Нет активного Sink".to_string())
    }
}

#[tauri::command]
async fn set_music(state: State<'_, Mutex<AppState>>, path: &str) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    if state.stream.is_none() {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let file = BufReader::new(File::open(path).unwrap());
        let sink = rodio::play(&stream_handle.mixer(), file).unwrap();
        state.stream = Some(stream_handle);
        state.sink = Some(Arc::new(sink));
    } else {
        state.sink.as_ref().unwrap().clear();
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let file = BufReader::new(File::open(path).unwrap());
        let sink = rodio::play(&stream_handle.mixer(), file).unwrap();
        state.stream = Some(stream_handle);
        state.sink = Some(Arc::new(sink));
        state.current_path = Some(path.to_string());
    }
    Ok(())
}

#[tauri::command]
async fn seek_music(state: State<'_, Mutex<AppState>>, ms: u64) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    let path = state
        .current_path
        .clone()
        .ok_or_else(|| "Нет активного трека".to_string())?;

    let stream_handle = OutputStreamBuilder::open_default_stream().map_err(|e| e.to_string())?;
    let sink = Sink::connect_new(&stream_handle.mixer());

    let file = BufReader::new(File::open(&path).map_err(|e| e.to_string())?);
    let source = Decoder::try_from(file)
        .map_err(|e| e.to_string())?
        .skip_duration(Duration::from_millis(ms));

    stream_handle.mixer().add(source);

    state.stream = Some(stream_handle);
    state.sink = Some(Arc::new(sink));
    Ok(())
}

#[tauri::command]
async fn get_wave(path: String, points: usize) -> Result<Vec<f32>, String> {
    let wave = extract_waveform(path, points).await;

    match wave {
        Ok(w) => Ok(w),
        Err(_) => Err("Произошла ошибка".to_string()),
    }
}

// #[tauri::command]
// fn get_wave(path: String, points: usize) -> Result<Vec<f32>, String> {
//     let wave = read_audio_samples(path.as_str(), points);

//     Ok(wave)
//  }

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
            get_musics, set_music, play_music, stop_music, set_volume, set_speed, seek_music, get_wave
        ])
        .run(tauri::generate_context!())
        
        .expect("error while running tauri application");
}


