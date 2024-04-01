/*
 * Spotify Web API
 *
 * You can use Spotify's Web API to discover music and podcasts, manage your Spotify library, control audio playback, and much more. Browse our available Web API endpoints using the sidebar at left, or via the navigation bar on top of this page on smaller screens.  In order to make successful Web API requests your app will need a valid access token. One can be obtained through <a href=\"https://developer.spotify.com/documentation/general/guides/authorization-guide/\">OAuth 2.0</a>.  The base URI for all Web API requests is `https://api.spotify.com/v1`.  Need help? See our <a href=\"https://developer.spotify.com/documentation/web-api/guides/\">Web API guides</a> for more information, or visit the <a href=\"https://community.spotify.com/t5/Spotify-for-Developers/bd-p/Spotify_Developer\">Spotify for Developers community forum</a> to ask questions and connect with other developers. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::spotify;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Search200Response {
    #[serde(rename = "tracks", skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Box<spotify::PagingTrackObject>>,
    #[serde(rename = "artists", skip_serializing_if = "Option::is_none")]
    pub artists: Option<Box<spotify::PagingArtistObject>>,
    #[serde(rename = "albums", skip_serializing_if = "Option::is_none")]
    pub albums: Option<Box<spotify::PagingSimplifiedAlbumObject>>,
    #[serde(rename = "playlists", skip_serializing_if = "Option::is_none")]
    pub playlists: Option<Box<spotify::PagingPlaylistObject>>,
    #[serde(rename = "shows", skip_serializing_if = "Option::is_none")]
    pub shows: Option<Box<spotify::PagingSimplifiedShowObject>>,
    #[serde(rename = "episodes", skip_serializing_if = "Option::is_none")]
    pub episodes: Option<Box<spotify::PagingSimplifiedEpisodeObject>>,
    #[serde(rename = "audiobooks", skip_serializing_if = "Option::is_none")]
    pub audiobooks: Option<Box<spotify::PagingSimplifiedAudiobookObject>>,
}

impl Search200Response {
    pub fn new() -> Search200Response {
        Search200Response {
            tracks: None,
            artists: None,
            albums: None,
            playlists: None,
            shows: None,
            episodes: None,
            audiobooks: None,
        }
    }
}
