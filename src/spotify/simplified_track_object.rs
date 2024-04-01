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
pub struct SimplifiedTrackObject {
    /// The artists who performed the track. Each artist object includes a link in `href` to more detailed information about the artist.
    #[serde(rename = "artists", skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<spotify::SimplifiedArtistObject>>,
    /// A list of the countries in which the track can be played, identified by their [ISO 3166-1 alpha-2](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code. 
    #[serde(rename = "available_markets", skip_serializing_if = "Option::is_none")]
    pub available_markets: Option<Vec<String>>,
    /// The disc number (usually `1` unless the album consists of more than one disc).
    #[serde(rename = "disc_number", skip_serializing_if = "Option::is_none")]
    pub disc_number: Option<i32>,
    /// The track length in milliseconds.
    #[serde(rename = "duration_ms", skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i32>,
    /// Whether or not the track has explicit lyrics ( `true` = yes it does; `false` = no it does not OR unknown).
    #[serde(rename = "explicit", skip_serializing_if = "Option::is_none")]
    pub explicit: Option<bool>,
    /// External URLs for this track. 
    #[serde(rename = "external_urls", skip_serializing_if = "Option::is_none")]
    pub external_urls: Option<Box<spotify::ExternalUrlObject>>,
    /// A link to the Web API endpoint providing full details of the track.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the track. 
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Part of the response when [Track Relinking](/documentation/web-api/concepts/track-relinking/) is applied. If `true`, the track is playable in the given market. Otherwise `false`. 
    #[serde(rename = "is_playable", skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,
    /// Part of the response when [Track Relinking](/documentation/web-api/concepts/track-relinking/) is applied and is only part of the response if the track linking, in fact, exists. The requested track has been replaced with a different track. The track in the `linked_from` object contains information about the originally requested track.
    #[serde(rename = "linked_from", skip_serializing_if = "Option::is_none")]
    pub linked_from: Option<Box<spotify::LinkedTrackObject>>,
    /// Included in the response when a content restriction is applied. 
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Box<spotify::TrackRestrictionObject>>,
    /// The name of the track.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A URL to a 30 second preview (MP3 format) of the track. 
    #[serde(rename = "preview_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<Option<String>>,
    /// The number of the track. If an album has several discs, the track number is the number on the specified disc. 
    #[serde(rename = "track_number", skip_serializing_if = "Option::is_none")]
    pub track_number: Option<i32>,
    /// The object type: \"track\". 
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the track. 
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// Whether or not the track is from a local file. 
    #[serde(rename = "is_local", skip_serializing_if = "Option::is_none")]
    pub is_local: Option<bool>,
}

impl SimplifiedTrackObject {
    pub fn new() -> SimplifiedTrackObject {
        SimplifiedTrackObject {
            artists: None,
            available_markets: None,
            disc_number: None,
            duration_ms: None,
            explicit: None,
            external_urls: None,
            href: None,
            id: None,
            is_playable: None,
            linked_from: None,
            restrictions: None,
            name: None,
            preview_url: None,
            track_number: None,
            r#type: None,
            uri: None,
            is_local: None,
        }
    }
}
