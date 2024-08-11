use std::collections::HashMap;

use audiotags::{AudioTagEdit, Id3v2Tag, Tag};

pub trait AudioTag {
    fn metadata_fields(&self) -> Vec<(String, String)>;
}

impl AudioTag for Id3v2Tag {
    fn metadata_fields(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();

        if let Some(title) = self.title() {
            result.push(("Title".to_string(), title.to_string()));
        }

        if let Some(artist) = self.artist() {
            result.push(("Artist".to_string(), artist.to_string()));
        }

        if let Some(date) = self.date() {
            result.push(("Date".to_string(), date.to_string()));
        }

        if let Some(year) = self.year() {
            result.push(("Year".to_string(), year.to_string()));
        }

        if let Some(album) = self.album_title() {
            result.push(("Album Title".to_string(), album.to_string()));
        }

        if let Some(album_artist) = self.album_artist() {
            result.push(("Album Artist".to_string(), album_artist.to_string()));
        }

        if let Some(track) = self.track_number() {
            result.push(("Track Number".to_string(), track.to_string()));
        }

        if let Some(total_tracks) = self.total_tracks() {
            result.push(("Total Tracks".to_string(), total_tracks.to_string()));
        }

        if let Some(disc) = self.disc_number() {
            result.push(("Disc Number".to_string(), disc.to_string()));
        }

        if let Some(total_discs) = self.total_discs() {
            result.push(("Total Discs".to_string(), total_discs.to_string()));
        }

        if let Some(genre) = self.genre() {
            result.push(("Genre".to_string(), genre.to_string()));
        }

        if let Some(composer) = self.composer() {
            result.push(("Composer".to_string(), composer.to_string()));
        }

        if let Some(comment) = self.comment() {
            result.push(("Comment".to_string(), comment.to_string()));
        }

        result
    }
}

pub fn extract_metadata(file_path: &str) -> Result<HashMap<String, String>, std::io::Error> {

    // This function only works for mp3, m4a/mp4/, flac, still need to handle for wav and possibly other formats in another function
    let tag = Id3v2Tag::from(Tag::new().read_from_path(file_path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?);
    let mut metadata: HashMap<String, String> = HashMap::new();

    for field in tag.metadata_fields() {
        metadata.insert(field.0.clone(), field.1.clone());
    }
    Ok(metadata)

}
