use clap::{Parser, ValueEnum};
use lofty::{read_from_path, AudioFile};
use lofty::{Accessor, Probe};
use rayon::prelude::*;
use serde::Serialize;
use serde_json::{json, Value};
use std::io::SeekFrom;
use std::io::{Read, Seek};
use std::{
    collections::HashSet,
    fmt,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::{
    audio::{AudioBufferRef, Signal},
    codecs::{DecoderOptions, CODEC_TYPE_NULL},
};
use symphonia::default::get_codecs;
use symphonia::default::get_probe;
use walkdir::WalkDir;

use symphonia::core::errors::Error;
use symphonia::core::formats::{SeekMode, SeekTo};
use symphonia::core::io::MediaSourceStreamOptions;

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
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        };

        // Создаем медиа источник
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // Создаем hint для определения формата файла
        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            hint.with_extension(ext);
        }

        // Пробуем определить формат файла
        let probed = match symphonia::default::get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        ) {
            Ok(probed) => probed,
            Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        };

        let mut format = probed.format;

        // Находим аудио трек
        let track = match format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        {
            Some(track) => track,
            None => return Err(Box::from("no supported audio track")),
        };

        let track_id = track.id;

        // Создаем декодер
        let mut decoder = match symphonia::default::get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())
        {
            Ok(decoder) => decoder,
            Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        };

        // Получаем параметры трека
        let channels = match track.codec_params.channels {
            Some(channels) => channels.count(),
            None => return Err(Box::from("no channels information")),
        };

        let sample_rate = match track.codec_params.sample_rate {
            Some(sample_rate) => sample_rate,
            None => return Err(Box::from("no sample rate information")),
        };

        println!("Channels: {}, Sample rate: {}", channels, sample_rate);

        // Декодируем весь файл
        let mut all_samples = Vec::new();

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(Error::IoError(ref err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                    // Достигли конца файла
                    break;
                }
                Err(Error::ResetRequired) => {
                    // Требуется сброс декодера
                    decoder.reset();
                    continue;
                }
                Err(err) => {
                    println!("Error reading packet: {:?}", err);
                    break;
                }
            };

            // Проверяем, что пакет принадлежит нужному треку
            if packet.track_id() != track_id {
                continue;
            }

            // Декодируем пакет
            match decoder.decode(&packet) {
                Ok(decoded) => {
                    let spec = *decoded.spec();
                    let duration = decoded.frames() as u64;
                    let mut buf = SampleBuffer::<f32>::new(duration, spec);
                    buf.copy_interleaved_ref(decoded);

                    let samples = buf.samples();
                    all_samples.extend_from_slice(samples);

                    println!(
                        "Decoded {} samples, total: {}",
                        samples.len(),
                        all_samples.len()
                    );
                }
                Err(Error::DecodeError(err)) => {
                    println!("Decode error: {:?}, skipping packet", err);
                    continue;
                }
                Err(err) => {
                    println!("Fatal decode error: {:?}", err);
                    return Err(Box::new(err) as Box<dyn std::error::Error + Send + Sync>);
                }
            }
        }

        println!("Total samples decoded: {}", all_samples.len());

        if all_samples.is_empty() {
            println!("No samples decoded, returning zeros");
            return Ok(vec![0.0; points]);
        }

        // Конвертируем в моно если нужно (берем среднее по каналам)
        let mono_samples: Vec<f32> = if channels > 1 {
            println!("Converting {} channels to mono", channels);
            let mut mono = Vec::with_capacity(all_samples.len() / channels);
            for chunk in all_samples.chunks_exact(channels) {
                let sum: f32 = chunk.iter().sum();
                let avg = sum / channels as f32;
                mono.push(avg);
            }
            mono
        } else {
            println!("Already mono");
            all_samples
        };

        println!("Mono samples: {}", mono_samples.len());

        // Создаем waveform
        let total_samples = mono_samples.len();
        let samples_per_point = total_samples / points;

        println!(
            "Creating waveform with {} points, {} samples per point",
            points, samples_per_point
        );

        let mut result = Vec::with_capacity(points);

        if samples_per_point == 0 {
            // Если семплов меньше чем точек, интерполируем
            println!("Interpolating: fewer samples than points");
            for i in 0..points {
                let sample_idx = (i * total_samples) / points;
                if sample_idx < mono_samples.len() {
                    result.push(mono_samples[sample_idx].abs());
                } else {
                    result.push(0.0);
                }
            }
        } else {
            // Для каждой точки берем максимальное абсолютное значение в соответствующем сегменте
            for i in 0..points {
                let start_idx = i * samples_per_point;
                let end_idx = if i == points - 1 {
                    // Для последней точки берем все оставшиеся семплы
                    total_samples
                } else {
                    ((i + 1) * samples_per_point).min(total_samples)
                };

                if start_idx < total_samples {
                    let segment = &mono_samples[start_idx..end_idx];
                    if !segment.is_empty() {
                        let max_amplitude = segment
                            .iter()
                            .map(|&s| s.abs())
                            .fold(0.0f32, |acc, x| if x > acc { x } else { acc });
                        result.push(max_amplitude);
                    } else {
                        result.push(0.0);
                    }
                } else {
                    result.push(0.0);
                }
            }
        }

        println!("Waveform created with {} points", result.len());

        // Находим максимальное значение для нормализации
        let max_val = result
            .iter()
            .fold(0.0f32, |acc, &x| if x > acc { x } else { acc });

        println!("Max value before normalization: {}", max_val);

        // Нормализуем результат для лучшей визуализации
        if max_val > 0.0 {
            for val in &mut result {
                *val /= max_val;
            }
            println!("Normalized waveform");
        } else {
            println!("Max value is 0, no normalization needed");
        }

        // Выводим первые несколько значений для отладки
        println!(
            "First 10 waveform values: {:?}",
            &result[..result.len().min(10)]
        );

        Ok(result)
    })
    .await?
}

// Альтернативная версия для больших файлов с потоковой обработкой
pub async fn extract_waveform_streaming<P: AsRef<Path> + Send + 'static>(
    path: P,
    points: usize,
) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
    let path = path.as_ref().to_path_buf();

    tokio::task::spawn_blocking(move || {
        // открываем файл
        let file = File::open(&path)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            hint.with_extension(ext);
        }

        let probed = symphonia::default::get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;

        let mut format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or("no supported audio track")?;

        let track_id = track.id;

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())?;

        let channels = track
            .codec_params
            .channels
            .map(|c| c.count())
            .ok_or("no channels info")?;

        let sample_rate = track
            .codec_params
            .sample_rate
            .ok_or("no sample rate info")?;

        // оценка общего количества семплов
        let estimated_total_samples = if let Some(n_frames) = track.codec_params.n_frames {
            n_frames as usize
        } else {
            (sample_rate as usize) * 240 /* запас: 4 минуты */
        };

        let mut result = vec![0.0f32; points];
        let mut max_val = 0.0f32;

        // шаг выборки — чтобы не читать все сэмплы
        let stride = (estimated_total_samples / (points * 10)).max(1);
        let mut total_samples = 0usize;

        // основной цикл
        while let Ok(packet) = format.next_packet() {
            if packet.track_id() != track_id {
                continue;
            }

            let decoded = match decoder.decode(&packet) {
                Ok(decoded) => decoded,
                Err(Error::DecodeError(_)) => continue,
                Err(Error::IoError(ref err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(Error::ResetRequired) => {
                    decoder.reset();
                    continue;
                }
                Err(err) => return Err(Box::new(err) as _),
            };

            let spec = *decoded.spec();
            let duration = decoded.frames() as u64;
            let mut buf = SampleBuffer::<f32>::new(duration, spec);
            buf.copy_interleaved_ref(decoded);

            for (i, chunk) in buf.samples().chunks(channels).enumerate() {
                total_samples += 1;
                if total_samples % stride != 0 {
                    continue;
                }

                // downmix to mono
                let mono = if channels > 1 {
                    chunk.iter().sum::<f32>() / channels as f32
                } else {
                    chunk[0]
                };

                let abs_val = mono.abs();
                let point_idx = (total_samples * points / estimated_total_samples).min(points - 1);

                if abs_val > result[point_idx] {
                    result[point_idx] = abs_val;
                    if abs_val > max_val {
                        max_val = abs_val;
                    }
                }
            }

            if total_samples >= estimated_total_samples {
                break;
            }
        }

        // нормализация
        if max_val > 0.0 {
            for val in &mut result {
                *val /= max_val;
            }
        }

        Ok(result)
    })
    .await?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_extract_waveform() {
        let result = extract_waveform("test.mp3", 60).await;
        match result {
            Ok(waveform) => {
                println!("Waveform extracted successfully: {} points", waveform.len());
                println!("Values: {:?}", waveform);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_extract_waveform_streaming() {
        let result = extract_waveform_streaming("test.mp3", 60).await;
        match result {
            Ok(waveform) => {
                println!(
                    "Streaming waveform extracted successfully: {} points",
                    waveform.len()
                );
                println!("Values: {:?}", waveform);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
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
    use symphonia::core::audio::AudioBufferRef;
    use symphonia::core::codecs::DecoderOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::default::{get_codecs, get_probe};

    let file = File::open(path).expect("Cannot open audio file");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe()
        .format(
            &Default::default(),
            mss,
            &Default::default(),
            &Default::default(),
        )
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

use base64::{engine::general_purpose, Engine as _};
use lofty::{self, PictureType, Tag};
use once_cell::sync::OnceCell;

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

            let res = CoverArt {
                base64: base64_str,
                cache: None,
            };
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
