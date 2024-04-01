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
pub struct LinkedTrackObject {
    /// Known external URLs for this track. 
    #[serde(rename = "external_urls", skip_serializing_if = "Option::is_none")]
    pub external_urls: Option<Box<spotify::ExternalUrlObject>>,
    /// A link to the Web API endpoint providing full details of the track. 
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The [Spotify ID](/documentation/web-api/concepts/spotify-uris-ids) for the track. 
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The object type: \"track\". 
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The [Spotify URI](/documentation/web-api/concepts/spotify-uris-ids) for the track. 
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl LinkedTrackObject {
    pub fn new() -> LinkedTrackObject {
        LinkedTrackObject {
            external_urls: None,
            href: None,
            id: None,
            r#type: None,
            uri: None,
        }
    }
}

