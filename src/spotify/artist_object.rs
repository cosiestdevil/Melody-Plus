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
pub struct ArtistObject {
    /// Known external URLs for this artist. 
    #[serde(rename = "external_urls", skip_serializing_if = "Option::is_none")]
    pub external_urls: Option<Box<spotify::ExternalUrlObject>>,
    /// Information about the followers of the artist. 
    #[serde(rename = "followers", skip_serializing_if = "Option::is_none")]
    pub followers: Option<Box<spotify::FollowersObject>>,
    /// A list of the genres the artist is associated with. If not yet classified, the array is empty. 
    #[serde(rename = "genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    /// A link to the Web API endpoint providing full details of the artist. 
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the artist. 
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Images of the artist in various sizes, widest first. 
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<spotify::ImageObject>>,
    /// The name of the artist. 
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The popularity of the artist. The value will be between 0 and 100, with 100 being the most popular. The artist's popularity is calculated from the popularity of all the artist's tracks. 
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<i32>,
    /// The object type. 
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the artist. 
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ArtistObject {
    pub fn new() -> ArtistObject {
        ArtistObject {
            external_urls: None,
            followers: None,
            genres: None,
            href: None,
            id: None,
            images: None,
            name: None,
            popularity: None,
            r#type: None,
            uri: None,
        }
    }
}
/// The object type. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "artist")]
    Artist,
}

impl Default for Type {
    fn default() -> Type {
        Self::Artist
    }
}
