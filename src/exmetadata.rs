use pyo3::prelude::*;
use audiotags::{AudioTagEdit, Id3v2Tag, Tag};
use std::{fs::File, path::Path};
use rodio::{Decoder, Source};
use std::io::BufReader;
use crate::AudioSink;

#[derive(Debug)]
#[derive(Default, Clone)]
#[allow(dead_code)]
#[pyclass]
pub struct MetaData {
    title: Option<String>,
    artist: Option<String>,
    date: Option<String>,
    year: Option<String>,
    album_title: Option<String>,
    album_artist: Option<String>,
    track_number: Option<String>,
    total_tracks: Option<String>,
    disc_number: Option<String>,
    total_discs: Option<String>,
    genre: Option<String>,
    composer: Option<String>,
    comment: Option<String>,
    sample_rate: Option<u32>,
    channels: Option<String>,
    duration: Option<f64>,
}

#[pymethods]
impl MetaData {
    #[new]
    fn new(audio_sink: &AudioSink) -> Self {
        MetaData {
            title: audio_sink.metadata.title.clone(),
            artist: audio_sink.metadata.artist.clone(),
            date: audio_sink.metadata.date.clone(),
            year: audio_sink.metadata.year.clone(),
            album_title: audio_sink.metadata.album_title.clone(),
            album_artist: audio_sink.metadata.album_artist.clone(),
            track_number: audio_sink.metadata.track_number.clone(),
            total_tracks: audio_sink.metadata.total_tracks.clone(),
            disc_number: audio_sink.metadata.disc_number.clone(),
            total_discs: audio_sink.metadata.total_discs.clone(),
            genre: audio_sink.metadata.genre.clone(),
            composer: audio_sink.metadata.composer.clone(),
            comment: audio_sink.metadata.comment.clone(),
            sample_rate: audio_sink.metadata.sample_rate,
            channels: audio_sink.metadata.channels.clone(),
            duration: audio_sink.metadata.duration,
        }
    }

    #[getter]
    fn title(&self) -> Option<String> {
        self.title.clone()
    }

    #[getter]
    fn artist(&self) -> Option<String> {
        self.artist.clone()
    }

    #[getter]
    fn date(&self) -> Option<String> {
        self.date.clone()
    }

    #[getter]
    fn year(&self) -> Option<String> {
        self.year.clone()
    }

    #[getter]
    fn album_title(&self) -> Option<String> {
        self.album_title.clone()
    }

    #[getter]
    fn album_artist(&self) -> Option<String> {
        self.album_artist.clone()
    }

    #[getter]
    fn track_number(&self) -> Option<String> {
        self.track_number.clone()
    }

    #[getter]
    fn total_tracks(&self) -> Option<String> {
        self.total_tracks.clone()
    }

    #[getter]
    fn disc_number(&self) -> Option<String> {
        self.disc_number.clone()
    }

    #[getter]
    fn total_discs(&self) -> Option<String> {
        self.total_discs.clone()
    }

    #[getter]
    fn genre(&self) -> Option<String> {
        self.genre.clone()
    }

    #[getter]
    fn composer(&self) -> Option<String> {
        self.composer.clone()
    }

    #[getter]
    fn comment(&self) -> Option<String> {
        self.comment.clone()
    }

    #[getter]
    fn sample_rate(&self) -> Option<u32> {
        self.sample_rate
    }

    #[getter]
    fn channels(&self) -> Option<String> {
        self.channels.clone()
    }

    #[getter]
    fn duration(&self) -> Option<f64> {
        self.duration
    }
    
}


pub trait AudioTag {
    fn metadata_fields(&self) -> MetaData;
}

fn data_to_string<T: ToString>(data: Option<T>) -> Option<String> {
    data.and_then(|s| Some(s.to_string()))
}

impl AudioTag for Id3v2Tag {
    fn metadata_fields(&self) -> MetaData {
        MetaData {
            title: data_to_string(self.title()),
            artist: data_to_string(self.artist()),
            date: data_to_string(self.date()),
            year: data_to_string(self.year()),
            album_title: data_to_string(self.album_title()),
            album_artist: data_to_string(self.album_artist()),
            track_number: data_to_string(self.track_number()),
            total_tracks: data_to_string(self.total_tracks()),
            disc_number: data_to_string(self.disc_number()),
            total_discs: data_to_string(self.total_discs()),
            genre: data_to_string(self.genre()),
            composer: data_to_string(self.composer()),   
            comment: data_to_string(self.comment()),  
            sample_rate: None,
            channels: None,
            duration: None,       
        }
    }
}


pub fn metadata(file_path: &str) -> Result<Option<MetaData>, String> {
    let path = Path::new(file_path);

    match path.extension().and_then(|ext| ext.to_str()) {
        Some("mp3") | Some("m4a") | Some("mp4") | Some("flac") => {
            let tag = Tag::new()
                .read_from_path(path)
                .map_err(|e| e.to_string())?;
            let id3_tag = Id3v2Tag::from(tag);
            let metadata = id3_tag.metadata_fields();
            Ok(Some(metadata))
        },
        Some("wav") => {
            let file = File::open(file_path).map_err(|e| e.to_string())?;
            let source = Decoder::new(BufReader::new(file)).map_err(|e| e.to_string())?;
            let sample_rate = source.sample_rate();
            let channels = source.channels();
            let duration = source.total_duration().map_or(0.0, |d| d.as_secs_f64());
            let metadata = MetaData {
                sample_rate: Some(sample_rate),
                channels: Some(channels.to_string()),
                duration: Some(duration),
                ..MetaData::default()
            };
            Ok(Some(metadata)) 
        },
        _ => Err("Unsupported file format".to_string())
    }
}
