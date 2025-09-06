use clap::{Parser, ValueEnum};
use lofty::{read_from_path, AudioFile};
use lofty::{Accessor, Probe};
use rayon::prelude::*;
use serde::Serialize;
use serde_json::{json, Value};
use std::io::SeekFrom;
use std::{
    collections::HashSet,
    fmt,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::{
    audio::{AudioBufferRef, Signal},
    codecs::{DecoderOptions, CODEC_TYPE_NULL},
};
use symphonia::core::audio::SampleBuffer;
use symphonia::default::get_codecs;
use symphonia::default::get_probe;
use walkdir::WalkDir;
use std::io::{Read, Seek};

/// Формат вывода
#[derive(ValueEnum, Clone, Debug)]
pub enum Output {
    Text,
    Json,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::Text => write!(f, "text"),
            Output::Json => write!(f, "json"),
        }
    }
}

/// Аргументы CLI
#[derive(Parser, Debug)]
#[command(version, about = "Рекурсивный поиск музыки и тегов")]
pub struct Args {
    /// Папки для сканирования
    #[arg()]
    pub paths: Vec<String>,

    /// Формат вывода: text | json
    #[arg(long, default_value_t = Output::Text)]
    pub output: Output,
}

/// Трек
#[derive(Serialize)]
pub struct Track {
    pub path: String,
    pub tags: Value, // JSON объект со всеми тегами
}

fn is_audio(path: &Path, allowed: &HashSet<&'static str>) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| allowed.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Чтение всех тегов
fn read_tags(path: &Path) -> Track {
    let mut obj = serde_json::Map::new();

    if let Ok(tagged) = Probe::open(path).and_then(|p| p.read(true)) {
        // Стандартные поля через Accessor
        if let Some(tag) = tagged.primary_tag() {
            obj.insert("title".to_string(), json!(tag.title()));
            obj.insert("artist".to_string(), json!(tag.artist()));
            obj.insert("album".to_string(), json!(tag.album()));
            obj.insert("genre".to_string(), json!(tag.genre()));
            if let Some(y) = tag.get_string(&lofty::ItemKey::Year) {
                obj.insert("year".to_string(), json!(y));
            }
            if let Some(y) = tag.get_string(&lofty::ItemKey::TrackNumber) {
                obj.insert("track_number".to_string(), json!(y));
            }
            if let Some(y) = tag.get_string(&lofty::ItemKey::TrackTotal) {
                obj.insert("total_tracks".to_string(), json!(y));
            }
            if let Some(y) = tag.get_string(&lofty::ItemKey::DiscNumber) {
                obj.insert("disc_number".to_string(), json!(y));
            }
            if let Some(y) = tag.get_string(&lofty::ItemKey::DiscTotal) {
                obj.insert("total_discs".to_string(), json!(y));
            }
            if let Some(y) = tag.get_string(&lofty::ItemKey::Composer) {
                obj.insert("composer".to_string(), json!(y));
            }
            if let Some(y) = tag.get_string(&lofty::ItemKey::Comment) {
                obj.insert("comment".to_string(), json!(y));
            }

            let mut cache = CoverCache::global().lock().unwrap();

            if let Some((cover, _)) = cache.cover_art(path, tag) {
                obj.insert("cover".to_string(), json!(cover.as_base64()));
            }   

            // Любые нестандартные поля
            for item in tag.items() {
                let key = format!("{:?}", item.key());
                let val = match item.value() {
                    lofty::ItemValue::Text(s) => json!(s),
                    lofty::ItemValue::Locator(s) => json!(s),
                    lofty::ItemValue::Binary(_) => json!("<binary>"),
                };
                obj.insert(key, val);
            }



            // obj.insert("bitrates".to_string(), json!(extract_levels(&path.display().to_string(), 80)));
        }

        // Duration
        obj.insert(
            "duration_ms".to_string(),
            json!(tagged.properties().duration().as_millis() as u64),
        );
    }

    Track {
        path: path.display().to_string(),
        tags: Value::Object(obj),
    }
}

/// Получить список треков
pub fn get_music(paths: Vec<String>) -> Vec<Track> {
    let exts: HashSet<&'static str> = [
        "mp3", "flac", "wav", "ogg", "m4a", "aac", "wma", "opus", "aiff", "alac", "mpc",
    ]
    .into_iter()
    .collect();

    let mut tracks: Vec<Track> = Vec::new();

    for root_str in paths {
        let root = PathBuf::from(&root_str);

        if !root.exists() {
            eprintln!("Пропускаю отсутствующий путь: {}", root.display());
            continue;
        }
        if root.is_dir() {
            for entry in WalkDir::new(&root)
                .follow_links(false)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let path = entry.path();
                if !is_audio(path, &exts) {
                    continue;
                }

                tracks.push(read_tags(path));
            }
        } else {
            if root.is_file() {
                let path = root.as_path();
                if !is_audio(path, &exts) {
                    continue;
                }

                tracks.push(read_tags(path));
            }
        }
    }

    tracks
}

pub fn play(path: String) -> bool {
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");

    let file = BufReader::new(File::open(path).unwrap());
    let sink = rodio::play(&stream_handle.mixer(), file).unwrap();

    return true;
}

pub fn extract_levels(path: &str, _chunks: usize) -> Vec<f32> {
    let file = File::open(path).unwrap();
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe()
        .format(
            &Default::default(),
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .unwrap();

    let mut format = probed.format;
    let track = format.default_track().unwrap();

    let mut decoder = get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .unwrap();

    let mut amplitudes = Vec::new();

    while let Ok(packet) = format.next_packet() {
        let decoded = decoder.decode(&packet).unwrap();

        if let AudioBufferRef::S16(buf) = decoded {
            // Берём только первый канал
            let chan = buf.chan(0);
            println!("Каналы: {:?}", chan);
            let avg: f32 = chan
                .iter()
                .map(|&s| s as f32 / i16::MAX as f32)
                .map(|v| v.abs())
                .sum::<f32>()
                / chan.len() as f32;

            amplitudes.push(avg);
        }
    }

    println!("Амплитуды: {:?}", amplitudes);
    amplitudes
}



pub async fn extract_waveform<P: AsRef<Path> + Send + 'static>(
    path: P,
    points: usize,
) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
    let path = path.as_ref().to_path_buf();

    tokio::task::spawn_blocking(move || {
        // Открываем файл
        let file = File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // Подсказываем формату
        let mut hint = Hint::new();

        // Пробуем распарсить формат
        let probed = symphonia::default::get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;

        let mut format = probed.format;

        // Берём первый трек
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or("no supported audio track")?;

        // Декодер
        let mut decoder =
            symphonia::default::get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

        let mut values = Vec::new();

        // Читаем пакеты
        while let Ok(packet) = format.next_packet() {
            let decoded = decoder.decode(&packet)?;

            // Буфер под сэмплы
            let mut buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
            buf.copy_interleaved_ref(decoded);

            // Берём RMS чанками (пример: 1000 сэмплов)
            let chunk_size = 1000;
            for chunk in buf.samples().chunks(chunk_size) {
                let rms =
                    (chunk.iter().map(|s| s * s).sum::<f32>() / chunk.len() as f32).sqrt();
                values.push(rms);
            }
        }

        // Если значений меньше, чем нужно → просто возвращаем
        if values.len() <= points {
            return Ok(values);
        }

        // Ресемплинг: берём равномерно points значений
        let step = values.len() as f32 / points as f32;
        let mut result = Vec::with_capacity(points);

        for i in 0..points {
            let idx = (i as f32 * step).floor() as usize;
            result.push(values[idx]);
        }

        Ok(result)
    })
    .await? // unwrap JoinError
}


pub fn extract_waveform_fast<P: AsRef<Path>>(
    path: P,
    points: usize,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut waveform = Vec::with_capacity(points);

    let mut buffer = [0u8; 4]; // заголовок MP3 фрейма
    let mut frame_count = 0usize;

    // Пробегаем весь файл по 4 байта
    while reader.read_exact(&mut buffer).is_ok() {
        // проверяем синхро-биты: 11 единиц в начале
        if buffer[0] == 0xFF && (buffer[1] & 0xE0) == 0xE0 {
            // Берём амплитуду как пример: среднее первых 2 байт фрейма
            let amp = ((buffer[2] as f32).abs() + (buffer[3] as f32).abs()) / 255.0;
            waveform.push(amp);

            frame_count += 1;
            // Пропускаем весь фрейм: длина может быть рассчитана по заголовку
            // Чтобы ускорить, просто читаем следующий 418 байт (примерно 1152 сэмплов на 44.1kHz)
            let _ = reader.seek(SeekFrom::Current(418));
        } else {
            // если не фрейм, смещаемся на 1 байт
            reader.seek(SeekFrom::Current(-3))?;
        }
    }

    // Ресемплим до нужного количества точек
    if waveform.len() <= points {
        return Ok(waveform);
    }

    let step = waveform.len() as f32 / points as f32;
    let mut result = Vec::with_capacity(points);
    for i in 0..points {
        let idx = (i as f32 * step).floor() as usize;
        result.push(waveform[idx]);
    }

    Ok(result)
}

pub fn read_audio_samples(path: &str, points: usize) -> Vec<f32> {
    use std::fs::File;
    use symphonia::core::audio::{AudioBufferRef};
    use symphonia::core::codecs::DecoderOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::default::{get_codecs, get_probe};

    let file = File::open(path).expect("Cannot open audio file");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe()
        .format(&Default::default(), mss, &Default::default(), &Default::default())
        .expect("Unsupported audio format");

    let mut format = probed.format;
    let track = &format.tracks()[0];
    let mut decoder = get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .expect("Cannot create decoder");

    let mut all_samples = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(_) => break,
        };

        let decoded = decoder.decode(&packet).unwrap();

        match decoded {
            AudioBufferRef::F32(buf) => all_samples.extend(buf.chan(0)),
            AudioBufferRef::S16(buf) => {
                all_samples.extend(buf.chan(0).iter().map(|s| *s as f32 / i16::MAX as f32));
            }
            AudioBufferRef::U8(buf) => {
                all_samples.extend(buf.chan(0).iter().map(|s| (*s as f32 - 128.0) / 128.0));
            }
            _ => {}
        }
    }

    // --- Сжатие до num_points точек ---
    let mut reduced = Vec::with_capacity(points);
    let chunk_size = all_samples.len() / points.max(1); // защита от деления на 0

    for i in 0..points {
        let start = i * chunk_size;
        let end = ((i + 1) * chunk_size).min(all_samples.len());
        let chunk = &all_samples[start..end];
        let avg = if !chunk.is_empty() {
            chunk.iter().copied().sum::<f32>() / chunk.len() as f32
        } else {
            0.0
        };
        reduced.push(avg);
    }

    reduced
}


use std::collections::HashMap;
use std::sync::Mutex;

use lofty::{self, Tag, PictureType};
use once_cell::sync::OnceCell;
use base64::{engine::general_purpose, Engine as _};

#[derive(Clone, Debug)]
pub struct CoverArt {
    pub base64: String,
    pub cache: Option<PathBuf>,
}

impl CoverArt {
    pub fn as_base64(&self) -> &str {
        &self.base64
    }
}

#[derive(Debug)]
pub struct CoverCache {
    entries: HashMap<String, CoverArt>,
}

impl CoverCache {
    pub fn global() -> &'static Mutex<CoverCache> {
        static CACHE: OnceCell<Mutex<CoverCache>> = OnceCell::new();

        CACHE.get_or_init(|| {
            let c = CoverCache::new();
            Mutex::new(c)
        })
    }

    fn new() -> Self {
        CoverCache {
            entries: HashMap::new(),
        }
    }

    fn add_entry(&mut self, uuid: &str, cover: CoverArt) -> &CoverArt {
        self.entries.entry(uuid.to_string()).or_insert(cover)
    }

    fn lookup(&self, uuid: &str) -> Option<&CoverArt> {
        self.entries.get(uuid)
    }

    fn load_cover_art(&self, tag: &Tag, path: Option<&Path>) -> Option<(Vec<u8>, String)> {
        // 1) ищем в тэгах
        if let Some(picture) = tag.get_picture_type(PictureType::CoverFront) {
            return Some((picture.data().to_vec(), picture.mime_type().to_string()));
        } else {
            for picture in tag.pictures() {
                match picture.pic_type() {
                    PictureType::Other | PictureType::BandLogo => {
                        return Some((picture.data().to_vec(), picture.mime_type().to_string()))
                    }
                    _ => (),
                }
            }
        }

        // 2) ищем рядом с файлом
        if let Some(p) = path {
            let ext_covers = ["Cover.jpg", "Cover.png", "cover.jpg", "cover.png"];
            for name in ext_covers {
                let mut cover_file = PathBuf::from(p);
                cover_file.push(name);
                if let Ok(data) = std::fs::read(&cover_file) {
                    // определяем MIME по расширению
                    let mime = match cover_file.extension().and_then(|e| e.to_str()) {
                        Some("png") => "image/png",
                        Some("jpg") | Some("jpeg") => "image/jpeg",
                        _ => "application/octet-stream",
                    };
                    return Some((data, mime.to_string()));
                }
            }
        }

        None
    }

    pub fn cover_art(&mut self, path: &Path, tag: &Tag) -> Option<(CoverArt, String)> {
        // UUID упростим: просто имя файла
        let uuid = path.to_string_lossy().to_string();

        if let Some(c) = self.lookup(&uuid) {
            return Some((c.clone(), uuid));
        }

        if let Some((data, mime)) = self.load_cover_art(tag, path.parent()) {
            let encoded = general_purpose::STANDARD.encode(&data);
            let base64_str = format!("data:{};base64,{}", mime, encoded);

            let res = CoverArt { base64: base64_str, cache: None };
            self.add_entry(&uuid, res.clone());
            Some((res, uuid))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}