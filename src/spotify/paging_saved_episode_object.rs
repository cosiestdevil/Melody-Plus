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
pub struct PagingSavedEpisodeObject {
    /// A link to the Web API endpoint returning the full result of the request 
    #[serde(rename = "href")]
    pub href: String,
    /// The maximum number of items in the response (as set in the query or by default). 
    #[serde(rename = "limit")]
    pub limit: i32,
    /// URL to the next page of items. ( `null` if none) 
    #[serde(rename = "next", deserialize_with = "Option::deserialize")]
    pub next: Option<String>,
    /// The offset of the items returned (as set in the query or by default) 
    #[serde(rename = "offset")]
    pub offset: i32,
    /// URL to the previous page of items. ( `null` if none) 
    #[serde(rename = "previous", deserialize_with = "Option::deserialize")]
    pub previous: Option<String>,
    /// The total number of items available to return. 
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "items")]
    pub items: Vec<spotify::SavedEpisodeObject>,
}

impl PagingSavedEpisodeObject {
    pub fn new(href: String, limit: i32, next: Option<String>, offset: i32, previous: Option<String>, total: i32, items: Vec<spotify::SavedEpisodeObject>) -> PagingSavedEpisodeObject {
        PagingSavedEpisodeObject {
            href,
            limit,
            next,
            offset,
            previous,
            total,
            items,
        }
    }
}

