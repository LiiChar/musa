use std::fs::File;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::Instant;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatReader, FormatOptions, SeekMode, SeekTo};
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Symphonia error: {0}")]
    Symphonia(#[from] SymphoniaError),
    #[error("CPAL error: {0}")]
    Cpal(#[from] cpal::BuildStreamError),
    #[error("Unsupported audio format")]
    UnsupportedFormat,
    #[error("No supported audio tracks found")]
    NoTracks,
    #[error("Unsupported codec")]
    UnsupportedCodec,
    #[error("Stream playback error: {0}")]
    StreamPlay(#[from] cpal::PlayStreamError),
    #[error("Mutex poisoned")]
    MutexPoisoned,
    #[error("Seek failed")]
    SeekFailed,
}

pub struct Player {
    pub state: Arc<Mutex<PlayerState>>,
    pub stream: cpal::Stream,
}

pub struct PlayerState {
    pub buffer: VecDeque<f32>,
    pub format: Box<dyn FormatReader>,
    pub decoder: Box<dyn Decoder>,
    pub paused: bool,
    pub speed: f32,
    pub volume: f32,
    pub start_time: Instant,
    pub sample_rate: f32,
    pub pos: f32,
    pub last_ts: u64, // новый — timestamp последнего пакета
}

impl Player {
    pub fn new(path: &str) -> Result<Self, PlayerError> {
        let src = File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(src), MediaSourceStreamOptions::default());

        let hint = Hint::new();
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .map_err(PlayerError::Symphonia)?;

        let format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or(PlayerError::NoTracks)?;

        let dec_opts: DecoderOptions = Default::default();
        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .map_err(|_| PlayerError::UnsupportedCodec)?;

        let sample_rate = track
            .codec_params
            .sample_rate
            .ok_or(PlayerError::UnsupportedFormat)? as f32;

        let state = Arc::new(Mutex::new(PlayerState {
            buffer: VecDeque::with_capacity(8192),
            format,
            decoder,
            paused: false,
            speed: 1.0,
            volume: 1.0,
            start_time: Instant::now(),
            sample_rate,
            pos: 0.0,
            last_ts: 0,
        }));

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(PlayerError::UnsupportedFormat)?;
        let config = device
            .default_output_config()
            .map_err(|_| PlayerError::UnsupportedFormat)?;

        let state_clone = state.clone();
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _| {
                    let mut s = match state_clone.lock() {
                        Ok(s) => s,
                        Err(_) => return,
                    };
                    Player::fill_buffer(&mut s, data);
                },
                move |err| eprintln!("Stream error: {err}"),
                None,
            )?,
            _ => return Err(PlayerError::UnsupportedFormat),
        };

        stream.play()?;

        Ok(Player { state, stream })
    }

fn decode_next_packet(state: &mut PlayerState) {
    if let Ok(packet) = state.format.next_packet() {
        state.last_ts = packet.ts(); // сохраняем ts

        if let Ok(decoded) = state.decoder.decode(&packet) {
            let mut buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
            buf.copy_interleaved_ref(decoded);
            state.buffer.extend(buf.samples());
        }
    }
}

    pub fn fill_buffer(state: &mut PlayerState, data: &mut [f32]) {
        if state.paused {
            data.fill(0.0);
            return;
        }

        for d in data.iter_mut() {
            if state.buffer.len() < 1024 {
                Player::decode_next_packet(state);
            }

            let idx = state.pos.floor() as usize;
            let frac = state.pos - idx as f32;

            let sample = if idx + 1 < state.buffer.len() {
                let a = state.buffer[idx];
                let b = state.buffer[idx + 1];
                a + (b - a) * frac
            } else if idx < state.buffer.len() {
                state.buffer[idx]
            } else {
                0.0
            };

            *d = sample * state.volume;
            state.pos += 1.0;

            if state.pos >= state.buffer.len() as f32 {
                let drop = state.pos.floor() as usize;
                for _ in 0..drop.min(state.buffer.len()) {
                    state.buffer.pop_front();
                }
                state.pos = 0.0;
            }
        }
    }

    pub fn pause(&self) -> Result<(), PlayerError> {
        let mut s = self.state.lock().map_err(|_| PlayerError::MutexPoisoned)?;
        s.paused = true;
        Ok(())
    }

    pub fn resume(&self) -> Result<(), PlayerError> {
        let mut s = self.state.lock().map_err(|_| PlayerError::MutexPoisoned)?;
        s.paused = false;
        Ok(())
    }

    pub fn set_volume(&self, volume: f32) -> Result<(), PlayerError> {
        let mut s = self.state.lock().map_err(|_| PlayerError::MutexPoisoned)?;
        if (0.0..=1.0).contains(&volume) {
            s.volume = volume;
            Ok(())
        } else {
            Err(PlayerError::UnsupportedFormat)
        }
    }

    pub fn set_speed(&self, speed: f32) -> Result<(), PlayerError> {
        let mut s = self.state.lock().map_err(|_| PlayerError::MutexPoisoned)?;
        if speed > 0.0 && speed <= 4.0 {
            s.speed = speed;
            Ok(())
        } else {
            Err(PlayerError::UnsupportedFormat)
        }
    }

    pub fn seek(&self, sec: f32) -> Result<f32, PlayerError> {
        let mut s = self.state.lock().map_err(|_| PlayerError::MutexPoisoned)?;
        if sec < 0.0 {
            return Err(PlayerError::UnsupportedFormat);
        }

        let time = sec as u64 * s.sample_rate as u64;
        s.format
            .seek(
                SeekMode::Accurate,
                SeekTo::Time {
                    time: time.into(),
                    track_id: None,
                },
            )
            .map_err(|_| PlayerError::SeekFailed)?;

        s.buffer.clear();
        s.pos = 0.0;

        Player::decode_next_packet(&mut s);

        Ok(sec)
    }

pub fn current_time(&self) -> Result<f32, PlayerError> {
    let s = self.state.lock().map_err(|_| PlayerError::MutexPoisoned)?;

    let packet_time = s.last_ts as f32 / s.sample_rate;
    let buffer_offset = s.pos / s.sample_rate;

    Ok(packet_time + buffer_offset)
}
}

impl Drop for Player {
    fn drop(&mut self) {
        if let Err(err) = self.stream.pause() {
            eprintln!("Error pausing stream on drop: {}", err);
        }
    }
}
