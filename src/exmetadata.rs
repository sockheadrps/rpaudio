use pyo3::pyclass;
use audiotags::{AudioTagEdit, Id3v2Tag, Tag};

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
        }
    }
}

pub fn extract_metadata(file_path: &str) -> Option<MetaData> {
    // This function only works for mp3, m4a/mp4/, flac, still need to handle for wav and possibly other formats in another function
    let tag = Tag::new().read_from_path(file_path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));

    if tag.is_err() {
        return None;
    } 

    let tag = Id3v2Tag::from(tag.unwrap());
    let metadata = tag.metadata_fields();
    Some(metadata)
}
