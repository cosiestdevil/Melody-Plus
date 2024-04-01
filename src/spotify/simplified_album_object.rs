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
use crate::spotify::{AlbumType, ReleaseDatePrecision};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimplifiedAlbumObject {
    /// The type of the album. 
    #[serde(rename = "album_type")]
    pub album_type: AlbumType,
    /// The number of tracks in the album.
    #[serde(rename = "total_tracks")]
    pub total_tracks: i32,
    /// The markets in which the album is available: [ISO 3166-1 alpha-2 country codes](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2). _**NOTE**: an album is considered available in a market when at least 1 of its tracks is available in that market._ 
    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,
    /// Known external URLs for this album. 
    #[serde(rename = "external_urls")]
    pub external_urls: Box<spotify::ExternalUrlObject>,
    /// A link to the Web API endpoint providing full details of the album. 
    #[serde(rename = "href")]
    pub href: String,
    /// The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the album. 
    #[serde(rename = "id")]
    pub id: String,
    /// The cover art for the album in various sizes, widest first. 
    #[serde(rename = "images")]
    pub images: Vec<spotify::ImageObject>,
    /// The name of the album. In case of an album takedown, the value may be an empty string. 
    #[serde(rename = "name")]
    pub name: String,
    /// The date the album was first released. 
    #[serde(rename = "release_date")]
    pub release_date: String,
    /// The precision with which `release_date` value is known. 
    #[serde(rename = "release_date_precision")]
    pub release_date_precision: ReleaseDatePrecision,
    /// Included in the response when a content restriction is applied. 
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Box<spotify::AlbumRestrictionObject>>,
    /// The object type. 
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the album. 
    #[serde(rename = "uri")]
    pub uri: String,
    /// The artists of the album. Each artist object includes a link in `href` to more detailed information about the artist. 
    #[serde(rename = "artists")]
    pub artists: Vec<spotify::SimplifiedArtistObject>,
}

impl SimplifiedAlbumObject {
    pub fn new(album_type: AlbumType, total_tracks: i32, available_markets: Vec<String>, external_urls: spotify::ExternalUrlObject, href: String, id: String, images: Vec<spotify::ImageObject>, name: String, release_date: String, release_date_precision: ReleaseDatePrecision, r#type: Type, uri: String, artists: Vec<spotify::SimplifiedArtistObject>) -> SimplifiedAlbumObject {
        SimplifiedAlbumObject {
            album_type,
            total_tracks,
            available_markets,
            external_urls: Box::new(external_urls),
            href,
            id,
            images,
            name,
            release_date,
            release_date_precision,
            restrictions: None,
            r#type,
            uri,
            artists,
        }
    }
}


/// The object type. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "album")]
    Album,
}

impl Default for Type {
    fn default() -> Type {
        Self::Album
    }
}

