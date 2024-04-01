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
pub struct DisallowsObject {
    /// Interrupting playback. Optional field.
    #[serde(rename = "interrupting_playback", skip_serializing_if = "Option::is_none")]
    pub interrupting_playback: Option<bool>,
    /// Pausing. Optional field.
    #[serde(rename = "pausing", skip_serializing_if = "Option::is_none")]
    pub pausing: Option<bool>,
    /// Resuming. Optional field.
    #[serde(rename = "resuming", skip_serializing_if = "Option::is_none")]
    pub resuming: Option<bool>,
    /// Seeking playback location. Optional field.
    #[serde(rename = "seeking", skip_serializing_if = "Option::is_none")]
    pub seeking: Option<bool>,
    /// Skipping to the next context. Optional field.
    #[serde(rename = "skipping_next", skip_serializing_if = "Option::is_none")]
    pub skipping_next: Option<bool>,
    /// Skipping to the previous context. Optional field.
    #[serde(rename = "skipping_prev", skip_serializing_if = "Option::is_none")]
    pub skipping_prev: Option<bool>,
    /// Toggling repeat context flag. Optional field.
    #[serde(rename = "toggling_repeat_context", skip_serializing_if = "Option::is_none")]
    pub toggling_repeat_context: Option<bool>,
    /// Toggling shuffle flag. Optional field.
    #[serde(rename = "toggling_shuffle", skip_serializing_if = "Option::is_none")]
    pub toggling_shuffle: Option<bool>,
    /// Toggling repeat track flag. Optional field.
    #[serde(rename = "toggling_repeat_track", skip_serializing_if = "Option::is_none")]
    pub toggling_repeat_track: Option<bool>,
    /// Transfering playback between devices. Optional field.
    #[serde(rename = "transferring_playback", skip_serializing_if = "Option::is_none")]
    pub transferring_playback: Option<bool>,
}

impl DisallowsObject {
    pub fn new() -> DisallowsObject {
        DisallowsObject {
            interrupting_playback: None,
            pausing: None,
            resuming: None,
            seeking: None,
            skipping_next: None,
            skipping_prev: None,
            toggling_repeat_context: None,
            toggling_shuffle: None,
            toggling_repeat_track: None,
            transferring_playback: None,
        }
    }
}

