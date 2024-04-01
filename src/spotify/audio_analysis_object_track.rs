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
pub struct AudioAnalysisObjectTrack {
    /// The exact number of audio samples analyzed from this track. See also `analysis_sample_rate`.
    #[serde(rename = "num_samples", skip_serializing_if = "Option::is_none")]
    pub num_samples: Option<i32>,
    /// Length of the track in seconds.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// This field will always contain the empty string.
    #[serde(rename = "sample_md5", skip_serializing_if = "Option::is_none")]
    pub sample_md5: Option<String>,
    /// An offset to the start of the region of the track that was analyzed. (As the entire track is analyzed, this should always be 0.)
    #[serde(rename = "offset_seconds", skip_serializing_if = "Option::is_none")]
    pub offset_seconds: Option<i32>,
    /// The length of the region of the track was analyzed, if a subset of the track was analyzed. (As the entire track is analyzed, this should always be 0.)
    #[serde(rename = "window_seconds", skip_serializing_if = "Option::is_none")]
    pub window_seconds: Option<i32>,
    /// The sample rate used to decode and analyze this track. May differ from the actual sample rate of this track available on Spotify.
    #[serde(rename = "analysis_sample_rate", skip_serializing_if = "Option::is_none")]
    pub analysis_sample_rate: Option<i32>,
    /// The number of channels used for analysis. If 1, all channels are summed together to mono before analysis.
    #[serde(rename = "analysis_channels", skip_serializing_if = "Option::is_none")]
    pub analysis_channels: Option<i32>,
    /// The time, in seconds, at which the track's fade-in period ends. If the track has no fade-in, this will be 0.0.
    #[serde(rename = "end_of_fade_in", skip_serializing_if = "Option::is_none")]
    pub end_of_fade_in: Option<f64>,
    /// The time, in seconds, at which the track's fade-out period starts. If the track has no fade-out, this should match the track's length.
    #[serde(rename = "start_of_fade_out", skip_serializing_if = "Option::is_none")]
    pub start_of_fade_out: Option<f64>,
    /// The overall loudness of a track in decibels (dB). Loudness values are averaged across the entire track and are useful for comparing relative loudness of tracks. Loudness is the quality of a sound that is the primary psychological correlate of physical strength (amplitude). Values typically range between -60 and 0 db. 
    #[serde(rename = "loudness", skip_serializing_if = "Option::is_none")]
    pub loudness: Option<f32>,
    /// The overall estimated tempo of a track in beats per minute (BPM). In musical terminology, tempo is the speed or pace of a given piece and derives directly from the average beat duration. 
    #[serde(rename = "tempo", skip_serializing_if = "Option::is_none")]
    pub tempo: Option<f32>,
    /// The confidence, from 0.0 to 1.0, of the reliability of the `tempo`.
    #[serde(rename = "tempo_confidence", skip_serializing_if = "Option::is_none")]
    pub tempo_confidence: Option<f64>,
    /// An estimated time signature. The time signature (meter) is a notational convention to specify how many beats are in each bar (or measure). The time signature ranges from 3 to 7 indicating time signatures of \"3/4\", to \"7/4\".
    #[serde(rename = "time_signature", skip_serializing_if = "Option::is_none")]
    pub time_signature: Option<i32>,
    /// The confidence, from 0.0 to 1.0, of the reliability of the `time_signature`.
    #[serde(rename = "time_signature_confidence", skip_serializing_if = "Option::is_none")]
    pub time_signature_confidence: Option<f64>,
    /// The key the track is in. Integers map to pitches using standard [Pitch Class notation](https://en.wikipedia.org/wiki/Pitch_class). E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on. If no key was detected, the value is -1. 
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<i32>,
    /// The confidence, from 0.0 to 1.0, of the reliability of the `key`.
    #[serde(rename = "key_confidence", skip_serializing_if = "Option::is_none")]
    pub key_confidence: Option<f64>,
    /// Mode indicates the modality (major or minor) of a track, the type of scale from which its melodic content is derived. Major is represented by 1 and minor is 0. 
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// The confidence, from 0.0 to 1.0, of the reliability of the `mode`.
    #[serde(rename = "mode_confidence", skip_serializing_if = "Option::is_none")]
    pub mode_confidence: Option<f64>,
    /// An [Echo Nest Musical Fingerprint (ENMFP)](https://academiccommons.columbia.edu/doi/10.7916/D8Q248M4) codestring for this track.
    #[serde(rename = "codestring", skip_serializing_if = "Option::is_none")]
    pub codestring: Option<String>,
    /// A version number for the Echo Nest Musical Fingerprint format used in the codestring field.
    #[serde(rename = "code_version", skip_serializing_if = "Option::is_none")]
    pub code_version: Option<f64>,
    /// An [EchoPrint](https://github.com/spotify/echoprint-codegen) codestring for this track.
    #[serde(rename = "echoprintstring", skip_serializing_if = "Option::is_none")]
    pub echoprintstring: Option<String>,
    /// A version number for the EchoPrint format used in the echoprintstring field.
    #[serde(rename = "echoprint_version", skip_serializing_if = "Option::is_none")]
    pub echoprint_version: Option<f64>,
    /// A [Synchstring](https://github.com/echonest/synchdata) for this track.
    #[serde(rename = "synchstring", skip_serializing_if = "Option::is_none")]
    pub synchstring: Option<String>,
    /// A version number for the Synchstring used in the synchstring field.
    #[serde(rename = "synch_version", skip_serializing_if = "Option::is_none")]
    pub synch_version: Option<f64>,
    /// A Rhythmstring for this track. The format of this string is similar to the Synchstring.
    #[serde(rename = "rhythmstring", skip_serializing_if = "Option::is_none")]
    pub rhythmstring: Option<String>,
    /// A version number for the Rhythmstring used in the rhythmstring field.
    #[serde(rename = "rhythm_version", skip_serializing_if = "Option::is_none")]
    pub rhythm_version: Option<f64>,
}

impl AudioAnalysisObjectTrack {
    pub fn new() -> AudioAnalysisObjectTrack {
        AudioAnalysisObjectTrack {
            num_samples: None,
            duration: None,
            sample_md5: None,
            offset_seconds: None,
            window_seconds: None,
            analysis_sample_rate: None,
            analysis_channels: None,
            end_of_fade_in: None,
            start_of_fade_out: None,
            loudness: None,
            tempo: None,
            tempo_confidence: None,
            time_signature: None,
            time_signature_confidence: None,
            key: None,
            key_confidence: None,
            mode: None,
            mode_confidence: None,
            codestring: None,
            code_version: None,
            echoprintstring: None,
            echoprint_version: None,
            synchstring: None,
            synch_version: None,
            rhythmstring: None,
            rhythm_version: None,
        }
    }
}

