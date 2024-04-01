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
pub struct AudioFeaturesObject {
    /// A confidence measure from 0.0 to 1.0 of whether the track is acoustic. 1.0 represents high confidence the track is acoustic. 
    #[serde(rename = "acousticness", skip_serializing_if = "Option::is_none")]
    pub acousticness: Option<f32>,
    /// A URL to access the full audio analysis of this track. An access token is required to access this data. 
    #[serde(rename = "analysis_url", skip_serializing_if = "Option::is_none")]
    pub analysis_url: Option<String>,
    /// Danceability describes how suitable a track is for dancing based on a combination of musical elements including tempo, rhythm stability, beat strength, and overall regularity. A value of 0.0 is least danceable and 1.0 is most danceable. 
    #[serde(rename = "danceability", skip_serializing_if = "Option::is_none")]
    pub danceability: Option<f32>,
    /// The duration of the track in milliseconds. 
    #[serde(rename = "duration_ms", skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i32>,
    /// Energy is a measure from 0.0 to 1.0 and represents a perceptual measure of intensity and activity. Typically, energetic tracks feel fast, loud, and noisy. For example, death metal has high energy, while a Bach prelude scores low on the scale. Perceptual features contributing to this attribute include dynamic range, perceived loudness, timbre, onset rate, and general entropy. 
    #[serde(rename = "energy", skip_serializing_if = "Option::is_none")]
    pub energy: Option<f32>,
    /// The Spotify ID for the track. 
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Predicts whether a track contains no vocals. \"Ooh\" and \"aah\" sounds are treated as instrumental in this context. Rap or spoken word tracks are clearly \"vocal\". The closer the instrumentalness value is to 1.0, the greater likelihood the track contains no vocal content. Values above 0.5 are intended to represent instrumental tracks, but confidence is higher as the value approaches 1.0. 
    #[serde(rename = "instrumentalness", skip_serializing_if = "Option::is_none")]
    pub instrumentalness: Option<f32>,
    /// The key the track is in. Integers map to pitches using standard [Pitch Class notation](https://en.wikipedia.org/wiki/Pitch_class). E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on. If no key was detected, the value is -1. 
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<i32>,
    /// Detects the presence of an audience in the recording. Higher liveness values represent an increased probability that the track was performed live. A value above 0.8 provides strong likelihood that the track is live. 
    #[serde(rename = "liveness", skip_serializing_if = "Option::is_none")]
    pub liveness: Option<f32>,
    /// The overall loudness of a track in decibels (dB). Loudness values are averaged across the entire track and are useful for comparing relative loudness of tracks. Loudness is the quality of a sound that is the primary psychological correlate of physical strength (amplitude). Values typically range between -60 and 0 db. 
    #[serde(rename = "loudness", skip_serializing_if = "Option::is_none")]
    pub loudness: Option<f32>,
    /// Mode indicates the modality (major or minor) of a track, the type of scale from which its melodic content is derived. Major is represented by 1 and minor is 0. 
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// Speechiness detects the presence of spoken words in a track. The more exclusively speech-like the recording (e.g. talk show, audio book, poetry), the closer to 1.0 the attribute value. Values above 0.66 describe tracks that are probably made entirely of spoken words. Values between 0.33 and 0.66 describe tracks that may contain both music and speech, either in sections or layered, including such cases as rap music. Values below 0.33 most likely represent music and other non-speech-like tracks. 
    #[serde(rename = "speechiness", skip_serializing_if = "Option::is_none")]
    pub speechiness: Option<f32>,
    /// The overall estimated tempo of a track in beats per minute (BPM). In musical terminology, tempo is the speed or pace of a given piece and derives directly from the average beat duration. 
    #[serde(rename = "tempo", skip_serializing_if = "Option::is_none")]
    pub tempo: Option<f32>,
    /// An estimated time signature. The time signature (meter) is a notational convention to specify how many beats are in each bar (or measure). The time signature ranges from 3 to 7 indicating time signatures of \"3/4\", to \"7/4\".
    #[serde(rename = "time_signature", skip_serializing_if = "Option::is_none")]
    pub time_signature: Option<i32>,
    /// A link to the Web API endpoint providing full details of the track. 
    #[serde(rename = "track_href", skip_serializing_if = "Option::is_none")]
    pub track_href: Option<String>,
    /// The object type. 
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The Spotify URI for the track. 
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// A measure from 0.0 to 1.0 describing the musical positiveness conveyed by a track. Tracks with high valence sound more positive (e.g. happy, cheerful, euphoric), while tracks with low valence sound more negative (e.g. sad, depressed, angry). 
    #[serde(rename = "valence", skip_serializing_if = "Option::is_none")]
    pub valence: Option<f32>,
}

impl AudioFeaturesObject {
    pub fn new() -> AudioFeaturesObject {
        AudioFeaturesObject {
            acousticness: None,
            analysis_url: None,
            danceability: None,
            duration_ms: None,
            energy: None,
            id: None,
            instrumentalness: None,
            key: None,
            liveness: None,
            loudness: None,
            mode: None,
            speechiness: None,
            tempo: None,
            time_signature: None,
            track_href: None,
            r#type: None,
            uri: None,
            valence: None,
        }
    }
}
/// The object type. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "audio_features")]
    AudioFeatures,
}

impl Default for Type {
    fn default() -> Type {
        Self::AudioFeatures
    }
}

