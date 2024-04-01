use serde::{Deserialize, Serialize};

/// The type of the album.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlbumType {
    #[serde(rename = "album")]
    Album,
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "compilation")]
    Compilation,
}
impl Default for AlbumType {
    fn default() -> AlbumType {
        Self::Album
    }
}